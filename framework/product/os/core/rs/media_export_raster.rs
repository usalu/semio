//! 🖼️ SVG rasterization and 2D media-export registration helpers.

use crate::media_graph::{
    register_os_media_export_handler, OsMediaExportFormat, OsMediaExportResult,
};
use base64::Engine;
use png::{BitDepth, ColorType, Encoder};
use serde_json::Value;

/// @emoji 🖼️ Rasterizes SVG markup to a base64-encoded PNG payload.
pub fn rasterize_svg_to_png_base64(svg: &str, width: u32, height: u32) -> Result<String, String> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).map_err(|error| error.to_string())?;
    let size = tree.size();
    let render_w = if width > 0 {
        width
    } else {
        size.width().ceil().max(1.0) as u32
    };
    let render_h = if height > 0 {
        height
    } else {
        size.height().ceil().max(1.0) as u32
    };
    let mut pixmap = tiny_skia::Pixmap::new(render_w, render_h).ok_or_else(|| "invalid raster dimensions".to_string())?;
    let scale_x = render_w as f32 / size.width().max(1.0);
    let scale_y = render_h as f32 / size.height().max(1.0);
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale_x, scale_y),
        &mut pixmap.as_mut(),
    );
    let png_bytes = encode_rgba_png(pixmap.data(), pixmap.width(), pixmap.height())?;
    Ok(base64::engine::general_purpose::STANDARD.encode(png_bytes))
}

fn encode_rgba_png(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    {
        let mut encoder = Encoder::new(&mut bytes, width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|error| error.to_string())?;
        writer.write_image_data(pixels).map_err(|error| error.to_string())?;
    }
    Ok(bytes)
}

/// @emoji 💾 Registers SVG and PNG export handlers for one 2D resource kind.
pub fn register_2d_svg_png_export_handlers(
    resource_kind: &'static str,
    file_stem: &'static str,
    document_to_svg: fn(&Value) -> Result<(String, u32, u32), String>,
) {
    register_os_media_export_handler(resource_kind, OsMediaExportFormat::Svg, move |doc| {
        let (svg, _width, _height) = document_to_svg(doc)?;
        Ok(OsMediaExportResult {
            data: svg,
            mime_type: "image/svg+xml".into(),
            file_name: format!("{file_stem}.svg"),
        })
    });
    register_os_media_export_handler(resource_kind, OsMediaExportFormat::Png, move |doc| {
        let (svg, width, height) = document_to_svg(doc)?;
        let data = rasterize_svg_to_png_base64(&svg, width, height)?;
        Ok(OsMediaExportResult {
            data,
            mime_type: "image/png".into(),
            file_name: format!("{file_stem}.png"),
        })
    });
}

/// @emoji 💾 Registers OBJ/GLB export handlers for one mesh resource kind.
pub fn register_mesh_obj_glb_export_handlers(
    resource_kind: &'static str,
    file_stem: &'static str,
    mesh_from_document: fn(&Value) -> Result<semio_framework_plugin::MeshData, String>,
) {
    use base64::Engine;
    register_os_media_export_handler(resource_kind, OsMediaExportFormat::Obj, move |doc| {
        let mesh = mesh_from_document(doc)?;
        let (data, mime_type) = semio_framework_plugin::export_mesh_obj(&mesh, file_stem);
        Ok(OsMediaExportResult {
            data,
            mime_type,
            file_name: format!("{file_stem}.obj"),
        })
    });
    register_os_media_export_handler(resource_kind, OsMediaExportFormat::Glb, move |doc| {
        let mesh = mesh_from_document(doc)?;
        let (bytes, mime_type) = semio_framework_plugin::export_mesh_glb_bytes(&mesh);
        Ok(OsMediaExportResult {
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            mime_type,
            file_name: format!("{file_stem}.glb"),
        })
    });
}
