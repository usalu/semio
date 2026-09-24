//! 🗺️ gismap ← dxf — a DXF file read by `s.stdio.semio/v1/drawing`'s own import leaf, then taken as
//! world geometry (`gis_map_snapshot_from_drawing`): circles become positions, closed paths regions,
//! open paths routes, coordinates read as `lon`/`lat`.
//!
//! 🔖 `IoFidelity::Lossy`: text, hatches and blocks carry no map feature; feature ids are assigned.
use crate::standards::v1::subsets::any::schema::gis_map_snapshot_from_drawing;
use crate::GisMapSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{decode_drawing, SemioDrawingFormat};

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisMapSnapshot, store::TextError> {
    let drawing = decode_drawing(bytes, SemioDrawingFormat::Dxf).map_err(|error| store::TextError::new(format!("gismap←dxf: {error}"), dsl::TextSpan::at(1, 1)))?;
    Ok(gis_map_snapshot_from_drawing(&drawing))
}
