use image::{RgbImage, Rgb};
use rusttype::{Font, Scale};
use imageproc::drawing::draw_text_mut;

const SIZE: usize = 128;

// 文字の長さ*32+余白の計算
fn calc(text: &str) -> usize {
    text.chars().count() * (SIZE / 2)
}

// 黒い画像を生成して、文字列を描画する
pub fn draw_text(up: &str, down: &str) -> RgbImage {
    let up_width = calc(up);
    let width = up_width + calc(down) + 30;
    let height = SIZE + 28;
    let mut image = RgbImage::new(width as u32, height as u32);
    for x in 0..width {
        for y in 0..height {
            image.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
        }
    }
    
    // 文字列を描画する
    let font_data = include_bytes!("../Roboto-Medium.ttf");
    let font = Vec::from(font_data as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    let scale = Scale { x: SIZE as f32, y: SIZE as f32 };
    let color = Rgb([255, 255, 255]);
    draw_text_mut(&mut image, color, 8, 8, scale, &font, up);
    // 後ろにオレンジ色を描画する
    let color = Rgb([255, 165, 0]);
    for x in up_width..(width - 8) {
        for y in 18..(height - 18) {
            image.put_pixel(x as u32, y as u32, color);
        }
    }
    let color = Rgb([0, 0, 0]);
    draw_text_mut(&mut image, color, (up_width as u32 + 8).try_into().unwrap(), 8, scale, &font, down);
    image
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calc() {
        let img = draw_text("Porn", "Hub");
        img.save("test.png").unwrap();
    }
}