#[cfg(test)]
mod dump {
    #[test]
    #[ignore]
    fn dump_thumb_hex() {
        let png_bytes = std::fs::read("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🖼️rathaus-ahlen-grundriss.png").unwrap();
        let mut reader = png::Decoder::new(std::io::Cursor::new(&png_bytes)).read_info().unwrap();
        let mut buf = vec![0u8; reader.output_buffer_size().unwrap_or(0)];
        let frame = reader.next_frame(&mut buf).unwrap();
        let info = reader.info();
        let palette = info.palette.clone();
        let rgba: Vec<u8> = match frame.color_type {
            png::ColorType::Indexed => {
                let table = palette.as_deref().unwrap();
                buf[..frame.buffer_size()].iter().flat_map(|&i| { let b=i as usize*3; [table[b],table[b+1],table[b+2],255] }).collect()
            }
            png::ColorType::Rgb => buf[..frame.buffer_size()].chunks_exact(3).flat_map(|p| [p[0],p[1],p[2],255]).collect(),
            png::ColorType::Rgba => buf[..frame.buffer_size()].to_vec(),
            _ => panic!("unexpected color type"),
        };
        let full = image::RgbaImage::from_raw(frame.width, frame.height, rgba).unwrap();
        let small = image::imageops::thumbnail(&full, 8, 8);
        let small_rgb = image::DynamicImage::ImageRgba8(small).to_rgb8();
        let hex: String = small_rgb.as_raw().iter().map(|b| format!("{b:02x}")).collect();
        println!("THUMB_HEX_START{}THUMB_HEX_END", hex);
        println!("THUMB_LEN={}", small_rgb.as_raw().len());
    }
}
