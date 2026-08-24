//! Serialize cad to stdio.ifc.
//!
//! 🧹️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: the
//! binary `serialize()` this file used to carry reinterpreted the CAD document's own opaque
//! `ArtifactPack` bytes as fabricated `IFCCARTESIANPOINT` entities (a real correctness bug — the
//! emitted "points" had nothing to do with any actual geometry) and had zero callers (`CadComposer`
//! only ever calls `serialize_text` below) — deleted outright per the master plan's cad extraction
//! row. No real cad↔ifc bridge exists (cad has no `model`-subset spatial-tree representation to
//! source one from) — reported as a `stdio_gaps` entry, not worked around here.
use crate::artifacts::cad::CadSnapshot;

//#region Serialize
pub fn register() {}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
