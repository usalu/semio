//! 🖼️ Lightweight SVG builders for simple document exports.

use serde_json::Value;

fn escape_svg_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// @emoji 🖼️ Wraps SVG body markup with explicit dimensions.
pub fn wrap_svg(width: u32, height: u32, body: &str) -> (String, u32, u32) {
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">{body}</svg>"#
    );
    (svg, width, height)
}

/// @emoji 🏷️ Builds a title-card SVG from a document JSON value.
pub fn title_card_svg(value: &Value, label: &str, width: u32, height: u32) -> Result<(String, u32, u32), String> {
    let title = value
        .get("title")
        .and_then(|entry| entry.as_str())
        .or_else(|| value.get("id").and_then(|entry| entry.as_str()))
        .unwrap_or(label);
    let body = format!(
        "<rect width=\"100%\" height=\"100%\" fill=\"white\"/><text x=\"32\" y=\"64\" font-size=\"32\" fill=\"#111827\">{}</text>",
        escape_svg_text(title)
    );
    Ok(wrap_svg(width, height, &body))
}

/// @emoji 📄 Serializes page-like rectangles from a `pages` array.
pub fn pages_rects_svg(value: &Value, fallback_label: &str) -> Result<(String, u32, u32), String> {
    let pages = value
        .get("pages")
        .and_then(|entry| entry.as_array())
        .cloned()
        .unwrap_or_default();
    if pages.is_empty() {
        return title_card_svg(value, fallback_label, 1024, 768);
    }
    let mut max_x = 0.0f64;
    let mut max_y = 0.0f64;
    let mut body = String::new();
    for (index, page) in pages.iter().enumerate() {
        let width = page.get("width").and_then(|entry| entry.as_f64()).unwrap_or(800.0);
        let height = page.get("height").and_then(|entry| entry.as_f64()).unwrap_or(600.0);
        let x = page.get("x").and_then(|entry| entry.as_f64()).unwrap_or((index as f64) * (width + 24.0));
        let y = page.get("y").and_then(|entry| entry.as_f64()).unwrap_or(0.0);
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
        body.push_str(&format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" fill=\"white\" stroke=\"#94a3b8\" stroke-width=\"2\"/>"
        ));
    }
    Ok(wrap_svg(max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32, &body))
}

/// @emoji 🗺️ Serializes point features from common GIS fixture fields.
pub fn map_points_svg(value: &Value, fallback_label: &str) -> Result<(String, u32, u32), String> {
    let positions = value
        .get("positions")
        .or_else(|| value.get("points"))
        .and_then(|entry| entry.as_array())
        .cloned()
        .unwrap_or_default();
    if positions.is_empty() {
        return title_card_svg(value, fallback_label, 1024, 768);
    }
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;
    for position in &positions {
        let Some(coords) = position.as_array() else { continue };
        let x = coords.first().and_then(|entry| entry.as_f64()).unwrap_or(0.0);
        let y = coords.get(1).and_then(|entry| entry.as_f64()).unwrap_or(0.0);
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    let pad = 32.0;
    let width = ((max_x - min_x) + pad * 2.0).max(256.0).round() as u32;
    let height = ((max_y - min_y) + pad * 2.0).max(256.0).round() as u32;
    let shifted = positions
        .iter()
        .filter_map(|position| position.as_array())
        .map(|coords| {
            let x = coords.first().and_then(|entry| entry.as_f64()).unwrap_or(0.0) - min_x + pad;
            let y = coords.get(1).and_then(|entry| entry.as_f64()).unwrap_or(0.0) - min_y + pad;
            format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"6\" fill=\"#2563eb\"/>")
        })
        .collect::<Vec<_>>()
        .join("");
    if shifted.is_empty() {
        return title_card_svg(value, fallback_label, 1024, 768);
    }
    Ok(wrap_svg(width, height, &shifted))
}
