//! 🗺️ gismap → pdf — the map's page drawing (`gis_map_snapshot_to_drawing`: markers, routes and
//! regions, north up) written as a one-page vector PDF 1.4 by `s.stdio.semio/v1/drawing`'s own export leaf.
//!
//! 🔖 `IoFidelity::Lossy`: a picture of the map — coordinates are shifted onto the page and feature
//! ids and payloads beyond geometry are not carried.
use crate::standards::v1::subsets::any::schema::gis_map_snapshot_to_drawing;
use crate::GisMapSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{encode_drawing, SemioDrawingFormat};

pub fn register() {}

pub fn serialize_bytes(snapshot: &GisMapSnapshot) -> Result<Vec<u8>, store::TextError> {
    encode_drawing(&gis_map_snapshot_to_drawing(snapshot), SemioDrawingFormat::Pdf { version: "1.4" }).map_err(|error| store::TextError::new(format!("gismap→pdf: {error}"), dsl::TextSpan::at(1, 1)))
}
