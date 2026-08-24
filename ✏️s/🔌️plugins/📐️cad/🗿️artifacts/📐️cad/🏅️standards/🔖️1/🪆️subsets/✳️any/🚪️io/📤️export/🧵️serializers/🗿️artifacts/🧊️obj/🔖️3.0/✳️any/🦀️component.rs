//! Serialize cad to stdio.obj.
//!
//! 🧹️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: the
//! binary `serialize()` this file used to carry reinterpreted the CAD document's own opaque
//! `ArtifactPack` bytes as fabricated `f32` vertex triples (a real correctness bug — the emitted
//! "geometry" had nothing to do with any actual solid; it also no longer compiled against the real
//! `ObjSnapshot` shape) and had zero callers (`CadComposer` only ever calls `serialize_text`
//! below) — deleted outright per the master plan's cad extraction row. Real geometry-aware OBJ
//! export lives at `⚙️engine/🦀️component.rs`'s `export_solids_as` (tessellates the live kernel
//! solids into a real `semio/mesh` snapshot and calls stdio's own `SemioMeshToObj` codec).
use crate::artifacts::cad::CadSnapshot;

//#region Serialize
pub fn register() {}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
