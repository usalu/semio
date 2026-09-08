//! Serialize layout to stdio.dwg.
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_dwg::schema::snapshot::decode_dwg;
use semio_s_artifact_stdio_dwg::DwgSnapshot;

pub fn register() {}

/// 📐️ Renders authored layout geometry to SVG before converting its paths to DWG.
pub fn serialize(from: &LayoutSnapshot) -> Result<DwgSnapshot, store::PackError> {
    let drawing = crate::io::layout_snapshot_to_semio_drawing(from);
    let text = crate::io::compose_svg_from_drawing(&drawing).map_err(store::PackError::Schema)?;
    let bytes = semio_framework_os::svg_to_polylines(&text).and_then(|paths| semio_s_artifact_stdio_dwg::standards::v_ac1024::subsets::any::io::polylines_to_dwg_bytes(paths.iter().map(|path| (path.layer.as_str(), path.vertices.as_slice(), path.closed)))).map_err(store::PackError::Schema)?;
    decode_dwg(&bytes).map_err(store::PackError::Schema)
}
