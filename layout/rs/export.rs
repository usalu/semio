use std::io::{Cursor, Write};

use image::{ImageBuffer, Rgba};
use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::display::DisplayList;
use crate::document::LayoutDocument;
use crate::engine::{build_display_list_for_page, display_list_to_scene};

pub fn export_display_list_svg(list: &DisplayList) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        list.page_width, list.page_height, list.page_width, list.page_height
    ));
    out.push('\n');
    out.push_str(&format!(r#"<rect width="{}" height="{}" fill="white"/>"#, list.page_width, list.page_height));
    out.push('\n');
    for rect in &list.rects {
        if let Some(fill) = &rect.fill {
            out.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="rgba({},{},{},{})"/>"#,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                (fill.0[0] * 255.0) as u8,
                (fill.0[1] * 255.0) as u8,
                (fill.0[2] * 255.0) as u8,
                fill.0[3]
            ));
            out.push('\n');
        }
        if let Some(stroke) = &rect.stroke {
            out.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="none" stroke="rgba({},{},{},{})" stroke-width="1"/>"#,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                (stroke.0[0] * 255.0) as u8,
                (stroke.0[1] * 255.0) as u8,
                (stroke.0[2] * 255.0) as u8,
                stroke.0[3]
            ));
            out.push('\n');
        }
    }
    for image in &list.images {
        let fill = if image.placeholder { "rgba(235,225,215,1)" } else { "rgba(220,220,220,1)" };
        out.push_str(&format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#,
            image.x, image.y, image.width, image.height, fill
        ));
        out.push('\n');
    }
    for run in &list.text_runs {
        for glyph in &run.glyphs {
            out.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#,
                glyph.x,
                glyph.y,
                glyph.font_size * 0.45,
                glyph.font_size
            ));
            out.push('\n');
        }
    }
    out.push_str("</svg>");
    out
}

pub fn export_document_svg(doc: &LayoutDocument, page_id: &str) -> Result<String, String> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| format!("page {page_id} not found"))?;
    let list = build_display_list_for_page(doc, page, page_id, &[], false);
    Ok(export_display_list_svg(&list))
}

pub fn export_document_pdf(doc: &LayoutDocument, page_id: &str) -> Result<Vec<u8>, String> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| format!("page {page_id} not found"))?;
    let list = build_display_list_for_page(doc, page, page_id, &[], false);
    let mut body = String::new();
    body.push_str("BT\n/F1 12 Tf\n");
    body.push_str(&format!("{} {} {} {} re\nf\n", 0, 0, page.width, page.height));
    for rect in &list.rects {
        if rect.fill.is_some() {
            body.push_str(&format!("{} {} {} {} re\nf\n", rect.x, rect.y, rect.width, rect.height));
        }
    }
    body.push_str("ET\n");
    let objects = vec![
        "1 0 obj<< /Type /Catalog /Pages 2 0 R >>endobj\n".to_string(),
        format!("2 0 obj<< /Type /Pages /Kids [3 0 R] /Count 1 >>endobj\n"),
        format!(
            "3 0 obj<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 4 0 R /Resources<< /Font<< /F1 5 0 R >> >> >>endobj\n",
            page.width, page.height
        ),
        format!("4 0 obj<< /Length {} >>stream\n{}endstream\nendobj\n", body.len(), body),
        "5 0 obj<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>endobj\n".to_string(),
    ];
    let mut pdf = String::from("%PDF-1.4\n");
    let mut offsets = vec![0usize];
    for object in &objects {
        offsets.push(pdf.len());
        pdf.push_str(object);
    }
    let xref_start = pdf.len();
    pdf.push_str(&format!("xref\n0 {}\n", objects.len() + 1));
    pdf.push_str("0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        pdf.push_str(&format!("{:010} 00000 n \n", offset));
    }
    pdf.push_str(&format!("trailer<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n", objects.len() + 1, xref_start));
    Ok(pdf.into_bytes())
}

pub fn export_document_png_cpu(doc: &LayoutDocument, page_id: &str) -> Result<Vec<u8>, String> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| format!("page {page_id} not found"))?;
    let list = build_display_list_for_page(doc, page, page_id, &[], false);
    let width = list.page_width.max(1.0) as u32;
    let height = list.page_height.max(1.0) as u32;
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));
    for rect in &list.rects {
        if let Some(fill) = &rect.fill {
            let color = Rgba([
                (fill.0[0] * 255.0) as u8,
                (fill.0[1] * 255.0) as u8,
                (fill.0[2] * 255.0) as u8,
                (fill.0[3] * 255.0) as u8,
            ]);
            let x0 = rect.x.max(0.0) as u32;
            let y0 = rect.y.max(0.0) as u32;
            let x1 = (rect.x + rect.width).min(width as f32) as u32;
            let y1 = (rect.y + rect.height).min(height as f32) as u32;
            for y in y0..y1 {
                for x in x0..x1 {
                    img.put_pixel(x, y, color);
                }
            }
        }
    }
    let mut bytes = Vec::new();
    {
        let encoder = png::Encoder::new(&mut bytes, width, height);
        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
        writer.write_image_data(img.as_raw()).map_err(|e| e.to_string())?;
    }
    Ok(bytes)
}

pub fn export_package_zip(doc_json: &str, preflight_json: &str) -> Result<Vec<u8>, String> {
    let doc: LayoutDocument = serde_json::from_str(doc_json).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.start_file("document.json", options).map_err(|e| e.to_string())?;
    zip.write_all(doc_json.as_bytes()).map_err(|e| e.to_string())?;
    zip.start_file("preflight-report.json", options).map_err(|e| e.to_string())?;
    zip.write_all(preflight_json.as_bytes()).map_err(|e| e.to_string())?;
    let manifest_links: Vec<serde_json::Value> = doc
        .links
        .iter()
        .map(|link| {
            let hash = if link.hash.is_empty() {
                format!("sha256:{:x}", Sha256::digest(link.path.as_bytes()))
            } else {
                link.hash.clone()
            };
            serde_json::json!({
                "id": link.id,
                "path": link.path,
                "hash": hash,
                "state": link.state,
                "missing": link.state.as_deref() == Some("missing"),
            })
        })
        .collect();
    let manifest = serde_json::json!({
        "schema": "layout.package-manifest/v1",
        "document": "document.json",
        "preflight": "preflight-report.json",
        "links": manifest_links,
        "generatedAt": "2026-07-02T00:00:00Z",
    });
    zip.start_file("package-manifest.json", options).map_err(|e| e.to_string())?;
    zip.write_all(manifest.to_string().as_bytes()).map_err(|e| e.to_string())?;
    let data = zip.finish().map_err(|e| e.to_string())?;
    Ok(data.into_inner())
}

pub fn scene_png_from_display_list(list: &DisplayList) -> Result<Vec<u8>, String> {
    let _scene = display_list_to_scene(list, false, (0.0, 0.0, 1.0));
    let width = list.page_width.max(1.0) as u32;
    let height = list.page_height.max(1.0) as u32;
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));
    let mut bytes = Vec::new();
    {
        let encoder = png::Encoder::new(&mut bytes, width, height);
        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
        writer.write_image_data(img.as_raw()).map_err(|e| e.to_string())?;
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svg_contains_root() {
        let list = DisplayList {
            page_id: "p".into(),
            page_width: 100.0,
            page_height: 100.0,
            rects: vec![],
            text_runs: vec![],
            images: vec![],
            guides: vec![],
        };
        let svg = export_display_list_svg(&list);
        assert!(svg.contains("<svg"));
    }
}
