//! 🖋️ Glyph atlas backed by fontdue rasterization.

use fontdue::Font as FontdueFont;
use std::collections::HashMap;

pub struct GlyphEntry {
    pub atlas_x: u32,
    pub atlas_y: u32,
    pub width: u32,
    pub height: u32,
    pub advance: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
}

pub struct FontAtlas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub font: FontdueFont,
    glyphs: HashMap<char, GlyphEntry>,
    cursor_x: u32,
    cursor_y: u32,
    row_height: u32,
}

impl FontAtlas {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let font = FontdueFont::from_bytes(bytes, fontdue::FontSettings::default())
            .map_err(|err| format!("font load failed: {err}"))?;
        Ok(Self {
            width: 2048,
            height: 2048,
            pixels: vec![0; 2048 * 2048],
            font,
            glyphs: HashMap::new(),
            cursor_x: 1,
            cursor_y: 1,
            row_height: 0,
        })
    }

    pub fn ensure_glyph(&mut self, ch: char) -> &GlyphEntry {
        if !self.glyphs.contains_key(&ch) {
            let (metrics, bitmap) = self.font.rasterize(ch, 16.0);
            let width = metrics.width as u32;
            let height = metrics.height as u32;
            if self.cursor_x + width + 2 >= self.width {
                self.cursor_x = 1;
                self.cursor_y += self.row_height + 2;
                self.row_height = 0;
            }
            let atlas_x = self.cursor_x;
            let atlas_y = self.cursor_y;
            for row in 0..height {
                let dst = ((atlas_y + row) * self.width + atlas_x) as usize;
                let src = (row * width) as usize;
                self.pixels[dst..dst + width as usize]
                    .copy_from_slice(&bitmap[src..src + width as usize]);
            }
            self.glyphs.insert(
                ch,
                GlyphEntry {
                    atlas_x,
                    atlas_y,
                    width,
                    height,
                    advance: metrics.advance_width,
                    bearing_x: metrics.xmin as f32,
                    bearing_y: metrics.ymin as f32,
                },
            );
            self.cursor_x += width + 2;
            self.row_height = self.row_height.max(height);
        }
        self.glyphs.get(&ch).expect("glyph just inserted")
    }

    pub fn measure_text(&mut self, text: &str, size: f32) -> (f32, f32) {
        let scale = size / 16.0;
        let mut width = 0.0f32;
        let mut max_height = 0.0f32;
        for ch in text.chars() {
            let glyph = self.ensure_glyph(ch);
            width += glyph.advance * scale;
            max_height = max_height.max((glyph.height as f32 + glyph.bearing_y) * scale);
        }
        (width, max_height.max(size))
    }
}

pub async fn fetch_font_bytes(url: &str) -> Result<Vec<u8>, String> {
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Uint8Array;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init(url, &opts).map_err(|_| "request failed")?;
        let window = web_sys::window().ok_or("no window")?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| "fetch failed")?;
        let resp: Response = resp_value.dyn_into().map_err(|_| "response cast failed")?;
        if !resp.ok() {
            return Err(format!("font fetch status {}", resp.status()));
        }
        let buffer = JsFuture::from(resp.array_buffer().map_err(|_| "array_buffer failed")?)
            .await
            .map_err(|_| "buffer failed")?;
        let array = Uint8Array::new(&buffer);
        let mut bytes = vec![0u8; array.length() as usize];
        array.copy_to(&mut bytes);
        if url.ends_with(".woff2") {
            return woff2::decode_to_vec(&bytes).map_err(|err| format!("woff2 decode: {err}"));
        }
        Ok(bytes)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = url;
        Err("font fetch only supported on wasm32".into())
    }
}
