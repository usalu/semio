//! Serialize cad to stdio.step.
//!
//! 🧹️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: the
//! binary `serialize()` this file used to carry reinterpreted the CAD document's own opaque
//! `ArtifactPack` bytes as fabricated `BrepVertex` triples (a real correctness bug — the emitted
//! "geometry" had nothing to do with any actual solid) and had zero callers (`CadComposer` only
//! ever calls `serialize_text` below) — deleted outright per the master plan's cad extraction row.
//! Real geometry-exact STEP export lives at `⚙️engine/🦀️.rs`'s `export_solids_as`
//! (sources the live kernel solids' real AP214 text, then round-trips it through stdio's own
//! `SemioBrepFromStep`/`SemioBrepToStep` `semio/brep` bridge).
use crate::artifacts::cad::CadSnapshot;

//#region Serialize
pub fn register() {}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
