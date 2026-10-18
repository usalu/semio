//! 📕️ Pdf, png and package encoding of the layout export job: the pdf objects, page and item operators, the png rows and idat chunks and the zip package entries. See [`super`].

use super::*;

//#region 🧩️Job
impl LayoutExportJob {
    /// 📕️ Which pages the PDF carries: the addressed page, or every page when none is addressed.
    fn pdf_page_indices(&self) -> Vec<usize> {
        match self.target_page {
            Some(index) if self.request.as_ref().unwrap().page_id.is_some() => vec![index],
            _ => (0..self.request.as_ref().unwrap().snapshot.pages.len()).collect(),
        }
    }

    /// 📕️ One unit per page: the same display list the Preview window paints (parent-page
    /// inheritance, overrides, parley-shaped glyphs) becomes this page's item slice.
    pub(super) fn plan_pdf_one(&mut self) -> Result<(), String> {
        let indices = self.pdf_page_indices();
        let Some(&page_index) = indices.get(self.pdf_page_cursor) else {
            self.begin_encoder()?;
            self.stage = ExportStage::Encode;
            return Ok(());
        };
        let snapshot = Arc::clone(&self.request.as_ref().unwrap().snapshot);
        let page = snapshot.pages.get(page_index).ok_or("layout-export-page-missing")?;
        let width = finite_dimension(page.width)?;
        let height = finite_dimension(page.height)?;
        let engine = self.pdf_engine.get_or_insert_with(semio_s_artifact_layout_layout_engine::scene::LayoutEngine::new);
        let list = semio_s_artifact_layout_layout_engine::scene::build_display_list_for_page(engine, &snapshot, page, "", &[], None, false);
        let mut items = 0usize;
        let mut push = |item: PdfItem, pdf_items: &mut Vec<PdfItem>| -> Result<(), String> {
            if pdf_items.len() >= MAX_LAYOUT_EXPORT_DECODED_ITEMS {
                return Err("layout-export-item-limit".into());
            }
            pdf_items.push(item);
            items += 1;
            Ok(())
        };
        for piece in semio_s_artifact_layout_layout_engine::scene::stack_pieces(&list) {
            match piece {
                semio_s_artifact_layout_layout_engine::scene::StackPiece::Stroke(stroke) | semio_s_artifact_layout_layout_engine::scene::StackPiece::FrameStroke(stroke) => {
                    push(PdfItem::Stroke { points: stroke.points.clone(), closed: stroke.closed, color: stroke.color, width: stroke.width, fill: stroke.fill }, &mut self.pdf_items)?;
                }
                semio_s_artifact_layout_layout_engine::scene::StackPiece::Rect(rect) => {
                    push(PdfItem::Rect { x: rect.x, y: rect.y, width: rect.width, height: rect.height, fill: rect.fill.as_ref().map(|color| color.0), stroke: rect.stroke.as_ref().map(|color| color.0) }, &mut self.pdf_items)?;
                }
                semio_s_artifact_layout_layout_engine::scene::StackPiece::Image(image) => {
                    push(PdfItem::Image { x: image.x, y: image.y, width: image.width, height: image.height, rotation: image.rotation, placeholder: image.placeholder, mark: image.preview.clone(), raster: image.proxy_data_url.as_deref().and_then(Self::proxy_png_rgb) }, &mut self.pdf_items)?;
                }
                semio_s_artifact_layout_layout_engine::scene::StackPiece::Text(run) => {
                    for glyph in &run.glyphs {
                        let glyph_id = u16::try_from(glyph.glyph_id).map_err(|_| "layout-export-glyph-range")?;
                        push(PdfItem::Glyph { x: glyph.x, y: glyph.y, size: glyph.font_size, glyph_id, color: glyph.color.0, italic: glyph.italic }, &mut self.pdf_items)?;
                    }
                }
            }
        }
        self.pdf_pages.push(PdfPagePlan { width, height, items });
        self.pdf_page_cursor += 1;
        Ok(())
    }

    pub(super) fn begin_pdf(&mut self) -> Result<(), String> {
        self.output.append(b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n")?;
        self.pdf_section = PdfSection::Catalog;
        self.pdf_cursor = 0;
        self.pdf_item_cursor = 0;
        Ok(())
    }

    fn pdf_object_offset(&mut self, id: usize) -> Result<(), String> {
        if self.pdf_offsets.len() + 1 != id {
            return Err("layout-export-pdf-object-order".into());
        }
        self.pdf_offsets.push(u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?);
        Ok(())
    }

    fn pdf_object(&mut self, id: usize, body: &[u8]) -> Result<(), String> {
        self.pdf_object_offset(id)?;
        self.output.append(format!("{id} 0 obj\n").as_bytes())?;
        self.output.append(body)?;
        self.output.append(b"\nendobj\n")
    }

    fn pdf_page_object_id(page: usize) -> usize {
        7 + 3 * page
    }


fn proxy_png_rgb(data_url: &str) -> Option<PdfRaster> {
    let payload = data_url.strip_prefix("data:image/png;base64,")?;
    let bytes = decode_base64(payload).ok()?;
    let snapshot = semio_framework_pixels::decode_png(&bytes).ok()?;
    let pixels = u64::from(snapshot.width).checked_mul(u64::from(snapshot.height))?;
    if pixels == 0 || pixels > 65_536 {
        return None;
    }
    let expected = (pixels as usize).checked_mul(4)?;
    if snapshot.pixels.len() != expected {
        return None;
    }
    let mut rgb = Vec::with_capacity(expected / 4 * 3);
    for pixel in snapshot.pixels.chunks_exact(4) {
        let alpha = f32::from(pixel[3]) / 255.0;
        for channel in 0..3 {
            rgb.push(((f32::from(pixel[channel]) / 255.0 * alpha + (1.0 - alpha)) * 255.0).round() as u8);
        }
    }
    Some(PdfRaster { width: snapshot.width, height: snapshot.height, rgb })
}

pub(super) fn pdf_image_matrix(x: f32, y: f32, width: f32, height: f32, rotation: f32, page_height: f32) -> String {
    if rotation.abs() < 1.0e-4 {
        return format!("{width:.3} 0 0 {height:.3} {x:.3} {:.3}", page_height - y - height);
    }
    let cx = x + width * 0.5;
    let cy = y + height * 0.5;
    let (sin, cos) = rotation.sin_cos();
    let map = |px: f32, py: f32| {
        let dx = px - cx;
        let dy = py - cy;
        let sx = cx + dx * cos - dy * sin;
        let sy = cy + dx * sin + dy * cos;
        (sx, page_height - sy)
    };
    let bottom_left = map(x, y + height);
    let bottom_right = map(x + width, y + height);
    let top_left = map(x, y);
    format!(
        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}",
        bottom_right.0 - bottom_left.0,
        bottom_right.1 - bottom_left.1,
        top_left.0 - bottom_left.0,
        top_left.1 - bottom_left.1,
        bottom_left.0,
        bottom_left.1
    )
}

fn pdf_inline_image(x: f32, y: f32, width: f32, height: f32, rotation: f32, page_height: f32, raster: &PdfRaster) -> String {
    let mut hex = String::with_capacity(raster.rgb.len() * 2);
    for byte in &raster.rgb {
        hex.push_str(&format!("{byte:02X}"));
    }
    let matrix = Self::pdf_image_matrix(x, y, width, height, rotation, page_height);
    format!("q {matrix} cm\nBI\n/W {} /H {} /CS /RGB /BPC 8 /F /AHx\nID\n{hex}>\nEI\nQ\n", raster.width, raster.height)
}

fn pdf_preview_ops(mark: &str, x: f32, y: f32, width: f32, height: f32, page_height: f32) -> String {
    let inset = 4.0_f32.min(width * 0.2).min(height * 0.2);
    if mark.is_empty() || inset < 0.5 {
        return String::new();
    }
    let left = x + inset;
    let right = x + width - inset;
    let top = page_height - (y + inset);
    let bottom = page_height - (y + height - inset);
    let line = |x1: f32, y1: f32, x2: f32, y2: f32| format!("q 0.2000 0.2500 0.3000 RG 1.5 w {x1:.3} {y1:.3} m {x2:.3} {y2:.3} l S Q\n");
    match mark {
        "page" => format!("q 0.2000 0.2500 0.3000 RG 1.5 w {left:.3} {bottom:.3} {:.3} {:.3} re S Q\n", right - left, top - bottom),
        "stroke" => line(left, top, right, bottom),
        "map" => {
            let mid_x = (left + right) * 0.5;
            let mid_y = (top + bottom) * 0.5;
            format!("{}{}", line(mid_x, top, mid_x, bottom), line(left, mid_y, right, mid_y))
        }
        "curve" => {
            let mid_x = (left + right) * 0.5;
            format!("{}{}", line(left, bottom, mid_x, top), line(mid_x, top, right, bottom))
        }
        "grid" => {
            let step_x = (right - left) / 3.0;
            let step_y = (top - bottom) / 3.0;
            let mut ops = String::new();
            for step in 1..3 {
                let y = bottom + step_y * step as f32;
                let x = left + step_x * step as f32;
                ops.push_str(&line(left, y, right, y));
                ops.push_str(&line(x, bottom, x, top));
            }
            ops
        }
        _ => String::new(),
    }
}

    fn pdf_item_ops(item: &PdfItem, page_height: f32) -> String {
        let color = |rgba: [f32; 4]| format!("{:.4} {:.4} {:.4}", rgba[0].clamp(0.0, 1.0), rgba[1].clamp(0.0, 1.0), rgba[2].clamp(0.0, 1.0));
        match item {
            PdfItem::Stroke { points, closed, color: stroke_color, width, fill } => {
                if points.len() < 2 {
                    return String::new();
                }
                let mut path = String::new();
                for (index, (x, y)) in points.iter().enumerate() {
                    let verb = if index == 0 { "m" } else { "l" };
                    path.push_str(&format!("{x:.3} {:.3} {verb} ", page_height - y));
                }
                if *closed {
                    path.push_str("h ");
                }
                let mut ops = String::new();
                if let Some(fill) = fill.filter(|rgba| rgba[3] > 0.001) {
                    ops.push_str(&format!("q {} rg {path}f Q\n", color(fill)));
                }
                ops.push_str(&format!("q {} RG {width:.3} w {path}S Q\n", color(*stroke_color)));
                ops
            }
            PdfItem::Rect { x, y, width, height, fill, stroke } => {
                let mut ops = String::new();
                let path = format!("{:.3} {:.3} {:.3} {:.3} re", x, page_height - y - height, width, height);
                if let Some(fill) = fill.filter(|rgba| rgba[3] > 0.001) {
                    ops.push_str(&format!("q {} rg {path} f Q\n", color(fill)));
                }
                if let Some(stroke) = stroke.filter(|rgba| rgba[3] > 0.001) {
                    ops.push_str(&format!("q {} RG 1 w {path} S Q\n", color(stroke)));
                }
                ops
            }
            PdfItem::Image { x, y, width, height, rotation, placeholder, mark, raster } => {
                if let Some(raster) = raster {
                    return Self::pdf_inline_image(*x, *y, *width, *height, *rotation, page_height, raster);
                }
                let path = format!("{:.3} {:.3} {:.3} {:.3} re", x, page_height - y - height, width, height);
                let body = if *placeholder {
                    format!("q 0.92 0.88 0.84 rg {path} f Q\nq 0.75 0.35 0.2 RG 1 w {path} S Q\n")
                } else {
                    format!("q 0.85 0.85 0.85 rg {path} f Q\n")
                };
                format!("{body}{}", Self::pdf_preview_ops(mark, *x, *y, *width, *height, page_height))
            }
            PdfItem::Glyph { x, y, size, glyph_id, color: fill, italic } => {
                let tinted = fill[0] > 0.001 || fill[1] > 0.001 || fill[2] > 0.001;
                let open = if tinted { format!("q {} rg ", color(*fill)) } else { String::new() };
                let close = if tinted { " Q" } else { "" };
                let tm = if *italic { format!("1 0 0.250 1 {:.3} {:.3}", x, page_height - y) } else { format!("1 0 0 1 {:.3} {:.3}", x, page_height - y) };
                format!("{open}BT /F1 {:.3} Tf {tm} Tm <{:04X}> Tj ET{close}\n", size, glyph_id)
            }
        }
    }

    pub(super) fn encode_pdf_one(&mut self) -> Result<(), String> {
        const FONT: &[u8] = semio_s_artifact_layout_layout_engine::scene::LAYOUT_SANS;
        let page_count = self.pdf_pages.len();
        match self.pdf_section {
            PdfSection::Header => return Err("layout-export-pdf-header-state".into()),
            PdfSection::Catalog => {
                self.pdf_object(1, b"<< /Type /Catalog /Pages 2 0 R >>")?;
                self.pdf_section = PdfSection::Pages;
            }
            PdfSection::Pages => {
                self.pdf_object_offset(2)?;
                self.output.append(b"2 0 obj\n<< /Type /Pages /Kids [")?;
                self.pdf_cursor = 0;
                self.pdf_section = PdfSection::PageKids;
            }
            PdfSection::PageKids => {
                if self.pdf_cursor < page_count {
                    self.output.append(format!("{} 0 R ", Self::pdf_page_object_id(self.pdf_cursor)).as_bytes())?;
                    self.pdf_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::PagesEnd;
                }
            }
            PdfSection::PagesEnd => {
                self.output.append(format!("] /Count {page_count} >>\nendobj\n").as_bytes())?;
                self.pdf_section = PdfSection::FontType0;
            }
            PdfSection::FontType0 => {
                self.pdf_object(3, b"<< /Type /Font /Subtype /Type0 /BaseFont /LayoutSans /Encoding /Identity-H /DescendantFonts [4 0 R] >>")?;
                self.pdf_section = PdfSection::FontCid;
            }
            PdfSection::FontCid => {
                self.pdf_object(4, b"<< /Type /Font /Subtype /CIDFontType2 /BaseFont /LayoutSans /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> /FontDescriptor 5 0 R /DW 600 /CIDToGIDMap /Identity >>")?;
                self.pdf_section = PdfSection::FontDescriptor;
            }
            PdfSection::FontDescriptor => {
                self.pdf_object(5, b"<< /Type /FontDescriptor /FontName /LayoutSans /Flags 32 /FontBBox [-200 -300 1200 1000] /ItalicAngle 0 /Ascent 900 /Descent -250 /CapHeight 700 /StemV 80 /FontFile2 6 0 R >>")?;
                self.pdf_section = PdfSection::FontFileBegin;
            }
            PdfSection::FontFileBegin => {
                self.pdf_object_offset(6)?;
                self.output.append(format!("6 0 obj\n<< /Length {} /Length1 {} >>\nstream\n", FONT.len(), FONT.len()).as_bytes())?;
                self.pdf_font_cursor = 0;
                self.pdf_section = PdfSection::FontFileBytes;
            }
            PdfSection::FontFileBytes => {
                let end = (self.pdf_font_cursor + OUTPUT_CHUNK_BYTES).min(FONT.len());
                if self.pdf_font_cursor < end {
                    self.output.append(&FONT[self.pdf_font_cursor..end])?;
                    self.pdf_font_cursor = end;
                } else {
                    self.pdf_section = PdfSection::FontFileEnd;
                }
            }
            PdfSection::FontFileEnd => {
                self.output.append(b"\nendstream\nendobj\n")?;
                self.pdf_cursor = 0;
                self.pdf_item_cursor = 0;
                self.pdf_section = if page_count == 0 { PdfSection::XrefBegin } else { PdfSection::PageObject };
            }
            PdfSection::PageObject => {
                let page = self.pdf_pages.get(self.pdf_cursor).copied().ok_or("layout-export-page-missing")?;
                let id = Self::pdf_page_object_id(self.pdf_cursor);
                let body = format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font << /F1 3 0 R >> >> /Contents {} 0 R >>", page.width, page.height, id + 1);
                self.pdf_object(id, body.as_bytes())?;
                self.pdf_section = PdfSection::ContentBegin;
            }
            PdfSection::ContentBegin => {
                let id = Self::pdf_page_object_id(self.pdf_cursor) + 1;
                self.pdf_object_offset(id)?;
                self.output.append(format!("{id} 0 obj\n<< /Length {} 0 R >>\nstream\n", id + 1).as_bytes())?;
                self.pdf_content_start = self.output.len;
                self.pdf_section = PdfSection::ContentItem;
            }
            PdfSection::ContentItem => {
                let page = self.pdf_pages.get(self.pdf_cursor).copied().ok_or("layout-export-page-missing")?;
                let first: usize = self.pdf_pages[..self.pdf_cursor].iter().map(|plan| plan.items).sum();
                if self.pdf_item_cursor < page.items {
                    let item = self.pdf_items.get(first + self.pdf_item_cursor).ok_or("layout-export-item-missing")?;
                    let ops = Self::pdf_item_ops(item, page.height as f32);
                    self.output.append(ops.as_bytes())?;
                    self.pdf_item_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::ContentEnd;
                }
            }
            PdfSection::ContentEnd => {
                self.pdf_xref_offset = self.output.len - self.pdf_content_start;
                self.output.append(b"\nendstream\nendobj\n")?;
                self.pdf_section = PdfSection::ContentLength;
            }
            PdfSection::ContentLength => {
                let id = Self::pdf_page_object_id(self.pdf_cursor) + 2;
                let length = self.pdf_xref_offset;
                self.pdf_object(id, format!("{length}").as_bytes())?;
                self.pdf_cursor += 1;
                self.pdf_item_cursor = 0;
                self.pdf_section = if self.pdf_cursor < page_count { PdfSection::PageObject } else { PdfSection::XrefBegin };
            }
            PdfSection::XrefBegin => {
                self.pdf_xref_offset = self.output.len;
                self.output.append(format!("xref\n0 {}\n0000000000 65535 f \n", self.pdf_offsets.len() + 1).as_bytes())?;
                self.pdf_cursor = 0;
                self.pdf_section = PdfSection::XrefEntries;
            }
            PdfSection::XrefEntries => {
                if let Some(offset) = self.pdf_offsets.get(self.pdf_cursor).copied() {
                    self.output.append(format!("{offset:010} 00000 n \n").as_bytes())?;
                    self.pdf_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::Trailer;
                }
            }
            PdfSection::Trailer => {
                self.output.append(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n", self.pdf_offsets.len() + 1, self.pdf_xref_offset).as_bytes())?;
                self.pdf_section = PdfSection::Complete;
            }
            PdfSection::Complete => self.stage = ExportStage::Base64,
        }
        Ok(())
    }

    pub(super) fn begin_png(&mut self) -> Result<(), String> {
        let page = self.page()?;
        let width = finite_dimension(page.width)?;
        let height = finite_dimension(page.height)?;
        self.output.append(b"\x89PNG\r\n\x1a\n")?;
        let mut ihdr = Vec::with_capacity(13);
        ihdr.extend_from_slice(&width.to_be_bytes());
        ihdr.extend_from_slice(&height.to_be_bytes());
        ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
        self.png_chunk(b"IHDR", &ihdr)?;
        let row_bytes = width as usize * 4 + 1;
        let idat_len = 2usize.checked_add(height as usize * (row_bytes + 5)).and_then(|value| value.checked_add(4)).ok_or("layout-export-output-limit")?;
        self.output.append(&u32::try_from(idat_len).map_err(|_| "layout-export-output-limit")?.to_be_bytes())?;
        self.output.append(b"IDAT")?;
        self.png_idat_crc = crc32_update(0, b"IDAT");
        self.append_idat(&[0x78, 0x01])
    }

    fn png_chunk(&mut self, kind: &[u8; 4], data: &[u8]) -> Result<(), String> {
        self.output.append(&u32::try_from(data.len()).map_err(|_| "layout-export-output-limit")?.to_be_bytes())?;
        self.output.append(kind)?;
        self.output.append(data)?;
        self.output.append(&crc32_update(crc32_update(0, kind), data).to_be_bytes())
    }

    fn append_idat(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.png_idat_crc = crc32_update(self.png_idat_crc, bytes);
        self.output.append(bytes)
    }

    pub(super) fn encode_png_one(&mut self) -> Result<(), String> {
        let (width, height) = {
            let page = self.page()?;
            (finite_dimension(page.width)?, finite_dimension(page.height)?)
        };
        if self.encode_cursor < height as usize {
            let y = self.encode_cursor as u32;
            match self.png_row_section {
                PngRowSection::Initialize => {
                    let row_len = width as usize * 4 + 1;
                    if self.png_row.is_empty() {
                        self.png_row = Vec::with_capacity(row_len);
                        self.png_row.push(0);
                    }
                    let end = self.png_row.len().saturating_add(OUTPUT_CHUNK_BYTES).min(row_len);
                    self.png_row.resize(end, 255);
                    if end == row_len {
                        self.png_rect_cursor = 0;
                        self.png_fill_x = 0;
                        self.png_row_section = PngRowSection::Fill;
                    }
                    return Ok(());
                }
                PngRowSection::Fill => {
                    if let Some(rect) = self.rects.get(self.png_rect_cursor) {
                        if y < rect.y || y >= rect.y + rect.height {
                            self.png_rect_cursor += 1;
                            self.png_fill_x = 0;
                            return Ok(());
                        }
                        let start = self.png_fill_x.max(rect.x);
                        let rect_end = rect.x + rect.width;
                        let end = start.saturating_add(PNG_PIXELS_PER_UNIT).min(rect_end);
                        for x in start..end {
                            let index = x as usize * 4 + 1;
                            self.png_row[index..index + 4].copy_from_slice(&rect.rgba);
                        }
                        if end == rect_end {
                            self.png_rect_cursor += 1;
                            self.png_fill_x = 0;
                        } else {
                            self.png_fill_x = end;
                        }
                    } else {
                        self.png_row_section = PngRowSection::Header;
                    }
                    return Ok(());
                }
                PngRowSection::Header => {
                    let len = u16::try_from(self.png_row.len()).map_err(|_| "layout-export-png-row-limit")?;
                    let mut header = [0u8; 5];
                    header[0] = u8::from(self.encode_cursor + 1 == height as usize);
                    header[1..3].copy_from_slice(&len.to_le_bytes());
                    header[3..5].copy_from_slice(&(!len).to_le_bytes());
                    self.append_idat(&header)?;
                    self.png_row_byte_cursor = 0;
                    self.png_row_section = PngRowSection::Data;
                    return Ok(());
                }
                PngRowSection::Data => {
                    let end = self.png_row_byte_cursor.saturating_add(OUTPUT_CHUNK_BYTES).min(self.png_row.len());
                    if self.png_row_byte_cursor < end {
                        let bytes = &self.png_row[self.png_row_byte_cursor..end];
                        for byte in bytes {
                            self.png_adler_s1 = (self.png_adler_s1 + u32::from(*byte)) % 65_521;
                            self.png_adler_s2 = (self.png_adler_s2 + self.png_adler_s1) % 65_521;
                        }
                        self.png_idat_crc = crc32_update(self.png_idat_crc, bytes);
                        self.output.append(bytes)?;
                        self.png_row_byte_cursor = end;
                        return Ok(());
                    }
                    self.png_row.clear();
                    self.png_row_section = PngRowSection::Initialize;
                    self.png_row_byte_cursor = 0;
                    self.png_rect_cursor = 0;
                    self.encode_cursor += 1;
                    return Ok(());
                }
            }
        }
        let adler = (self.png_adler_s2 << 16) | self.png_adler_s1;
        self.append_idat(&adler.to_be_bytes())?;
        self.output.append(&self.png_idat_crc.to_be_bytes())?;
        self.png_chunk(b"IEND", &[])?;
        self.stage = ExportStage::Base64;
        Ok(())
    }

    pub(super) fn begin_zip_entry(&mut self, name: &str) -> Result<(), String> {
        if self.zip.current_name.is_some() || self.zip.entries.len() >= MAX_LAYOUT_EXPORT_FILES {
            return Err("layout-export-package-entry-state".into());
        }
        let mut header = Vec::with_capacity(30 + name.len());
        push_u32(&mut header, 0x04034b50);
        push_u16(&mut header, 20);
        push_u16(&mut header, 0x0008);
        push_u16(&mut header, 0);
        push_u16(&mut header, 0);
        push_u16(&mut header, 0);
        push_u32(&mut header, 0);
        push_u32(&mut header, 0);
        push_u32(&mut header, 0);
        push_u16(&mut header, u16::try_from(name.len()).map_err(|_| "layout-export-package-name-limit")?);
        push_u16(&mut header, 0);
        header.extend_from_slice(name.as_bytes());
        self.zip.current_offset = u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?;
        self.output.append(&header)?;
        self.zip.current_name = Some(name.into());
        self.zip.current_crc = 0;
        self.zip.current_size = 0;
        Ok(())
    }

    fn append_zip_data(&mut self, bytes: &[u8]) -> Result<(), String> {
        if self.zip.current_name.is_none() {
            return Err("layout-export-package-no-entry".into());
        }
        self.zip.current_crc = crc32_update(self.zip.current_crc, bytes);
        self.zip.current_size = self.zip.current_size.checked_add(u32::try_from(bytes.len()).map_err(|_| "layout-export-output-limit")?).ok_or("layout-export-output-limit")?;
        self.output.append(bytes)
    }

    fn end_zip_entry(&mut self) -> Result<(), String> {
        let name = self.zip.current_name.take().ok_or("layout-export-package-no-entry")?;
        let mut descriptor = Vec::with_capacity(16);
        push_u32(&mut descriptor, 0x08074b50);
        push_u32(&mut descriptor, self.zip.current_crc);
        push_u32(&mut descriptor, self.zip.current_size);
        push_u32(&mut descriptor, self.zip.current_size);
        self.output.append(&descriptor)?;
        self.zip.entries.push(ZipEntry { name, crc: self.zip.current_crc, size: self.zip.current_size, offset: self.zip.current_offset });
        Ok(())
    }

    pub(super) fn encode_package_one(&mut self) -> Result<(), String> {
        match self.package_section {
            PackageSection::Begin => {
                self.package_json = Some(TypedJsonCursor::document());
                self.package_section = PackageSection::Scalars;
            }
            PackageSection::Scalars => {
                let (bytes, done) = self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.as_ref().unwrap().snapshot)?;
                if !bytes.is_empty() {
                    self.append_zip_data(&bytes)?;
                }
                if done {
                    self.package_json = None;
                    self.end_zip_entry()?;
                    self.begin_zip_entry("preflight-report.json")?;
                    self.package_byte_cursor = 0;
                    if self.request.as_ref().unwrap().preflight_json.is_none() {
                        self.package_json = Some(TypedJsonCursor::preflight());
                    }
                    self.package_section = PackageSection::Preflight;
                }
            }
            PackageSection::Preflight => {
                if let Some(preflight) = &self.request.as_ref().unwrap().preflight_json {
                    let end = self.package_byte_cursor.saturating_add(OUTPUT_CHUNK_BYTES).min(preflight.len());
                    if self.package_byte_cursor < end {
                        let bytes = preflight.as_bytes()[self.package_byte_cursor..end].to_vec();
                        self.append_zip_data(&bytes)?;
                        self.package_byte_cursor = end;
                    } else {
                        self.finish_preflight_entry()?;
                    }
                    return Ok(());
                }
                let (bytes, done) = self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.as_ref().unwrap().snapshot)?;
                if !bytes.is_empty() {
                    self.append_zip_data(&bytes)?;
                }
                if done {
                    self.package_json = None;
                    self.finish_preflight_entry()?;
                }
            }
            PackageSection::Manifest => {
                let (bytes, done) = self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.as_ref().unwrap().snapshot)?;
                if !bytes.is_empty() {
                    self.append_zip_data(&bytes)?;
                }
                if done {
                    self.package_json = None;
                    self.end_zip_entry()?;
                    self.package_index = 0;
                    self.zip_central_offset = u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?;
                    self.package_section = PackageSection::CentralDirectory;
                }
            }
            PackageSection::CentralDirectory => {
                if let Some(entry) = self.zip.entries.get(self.package_index).cloned() {
                    let mut central = Vec::with_capacity(46 + entry.name.len());
                    push_u32(&mut central, 0x02014b50);
                    push_u16(&mut central, 20);
                    push_u16(&mut central, 20);
                    push_u16(&mut central, 0x0008);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u32(&mut central, entry.crc);
                    push_u32(&mut central, entry.size);
                    push_u32(&mut central, entry.size);
                    push_u16(&mut central, u16::try_from(entry.name.len()).map_err(|_| "layout-export-package-name-limit")?);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u32(&mut central, 0);
                    push_u32(&mut central, entry.offset);
                    central.extend_from_slice(entry.name.as_bytes());
                    self.output.append(&central)?;
                    self.package_index += 1;
                } else {
                    let end = u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?;
                    let count = u16::try_from(self.zip.entries.len()).map_err(|_| "layout-export-package-entry-limit")?;
                    let mut eocd = Vec::with_capacity(22);
                    push_u32(&mut eocd, 0x06054b50);
                    push_u16(&mut eocd, 0);
                    push_u16(&mut eocd, 0);
                    push_u16(&mut eocd, count);
                    push_u16(&mut eocd, count);
                    push_u32(&mut eocd, end - self.zip_central_offset);
                    push_u32(&mut eocd, self.zip_central_offset);
                    push_u16(&mut eocd, 0);
                    self.output.append(&eocd)?;
                    self.package_section = PackageSection::Complete;
                }
            }
            PackageSection::Complete => self.stage = ExportStage::Base64,
        }
        Ok(())
    }

    fn finish_preflight_entry(&mut self) -> Result<(), String> {
        self.end_zip_entry()?;
        self.begin_zip_entry("package-manifest.json")?;
        self.package_json = Some(TypedJsonCursor::manifest());
        self.package_index = 0;
        self.package_section = PackageSection::Manifest;
        Ok(())
    }

}
//#endregion 🧩️Job
