//! Serialize cad to stdio.png.
//!
//! 🧹️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: the
//! binary `serialize()` this file used to carry packed the CAD document's own opaque
//! `ArtifactPack` bytes into a fabricated 1-pixel-tall RGBA raster (a real correctness bug — the
//! emitted "image" had nothing to do with any actual raster; it also no longer compiled against
//! the real `PngSnapshot` shape, which has no `image` field) and had zero callers (`CadComposer`
//! only ever calls `serialize_text` below) — deleted outright per the master plan's cad extraction
//! row. A real cad→png export needs an actual 3D-to-raster renderer (camera projection,
//! rasterization), which doesn't exist anywhere in this repo — reported as a `stdio_gaps` entry,
//! not worked around here.
use crate::artifacts::cad::CadSnapshot;

//#region Serialize
pub fn register() {}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
