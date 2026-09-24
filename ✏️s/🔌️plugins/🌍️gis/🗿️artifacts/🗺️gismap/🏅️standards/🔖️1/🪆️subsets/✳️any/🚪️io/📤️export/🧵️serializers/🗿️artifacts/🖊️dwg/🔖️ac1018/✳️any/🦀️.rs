//! 🗺️ gismap → dwg — the map's world drawing (`gis_map_snapshot_to_world_drawing`: every vertex its own
//! `lon`/`lat`, positions as circles of `GIS_WORLD_MARKER_RADIUS`) written as DWG entities by
//! `s.stdio.semio/v1/drawing`'s own export leaf; the sibling import leaf reads it back.
//!
//! 🔖 `IoFidelity::Lossy`: geometry survives feature for feature, feature ids and payloads beyond
//! geometry do not.
use crate::standards::v1::subsets::any::schema::gis_map_snapshot_to_world_drawing;
use crate::GisMapSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{encode_drawing, SemioDrawingFormat};

pub fn register() {}

pub fn serialize_bytes(snapshot: &GisMapSnapshot) -> Result<Vec<u8>, store::TextError> {
    encode_drawing(&gis_map_snapshot_to_world_drawing(snapshot), SemioDrawingFormat::Dwg).map_err(|error| store::TextError::new(format!("gismap→dwg: {error}"), dsl::TextSpan::at(1, 1)))
}
