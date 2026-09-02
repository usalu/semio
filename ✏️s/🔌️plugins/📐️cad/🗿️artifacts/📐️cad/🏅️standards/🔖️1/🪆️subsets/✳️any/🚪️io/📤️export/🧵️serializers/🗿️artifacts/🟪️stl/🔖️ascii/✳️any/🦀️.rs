//! Serialize cad to stdio.stl.
//!
//! 🧹️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: the
//! binary `serialize()` this file used to carry reinterpreted the CAD document's own opaque
//! `ArtifactPack` bytes as fabricated `f32` vertex triples (a real correctness bug, not a
//! duplication — the emitted "geometry" had nothing to do with any actual solid) and had zero
//! callers (`CadComposer`/`CadComposer`'s export table only ever call `serialize_text` below) —
//! deleted outright per the master plan's cad extraction row. Real geometry-aware STL export lives
//! at `⚙️engine/🦀️.rs`'s `export_solids_as` (tessellates the live kernel solids into a
//! real `semio/mesh` snapshot and calls stdio's own `SemioMeshToStl` codec).
use crate::artifacts::cad::CadSnapshot;

//#region Serialize
pub fn register() {}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
