//! 📷️ generation3d ← `s.stdio.png@1.2` — refused honestly.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf discarded the bytes and returned an
//! empty document, which read as a successful import of a blank model.
//!
//! A PNG is a raster image. This artifact's document is a BRep flow graph whose geometry is what
//! that graph evaluates to, and no operator in it reads pixels — the closest thing, an
//! `InputImage` widget, feeds image-processing nodes this artifact does not host. So there is no
//! import, and the typed error says which side is missing rather than pretending.
//!
//! 🖼️ The paired EXPORT direction is not this artifact's either: generation2d owns the `stdio.png`
//! export claim for the procedural plugin (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3 —
//! see `🚪️io/🦀️.rs`'s `🚪️IoRegistry` region), which is why `export_stdio_kinds` omits it while
//! `import_stdio_kinds` still lists png so a dropped file reaches this refusal instead of vanishing.
use crate::standards::v1::subsets::any::io::mesh_bridge::io_error;
use crate::Generation3dSnapshot;
use semio_s_artifact_stdio_png::PngSnapshot;

/// 🚫️ Why a raster image cannot become a generation3d document, in one sentence the UI can show.
const NO_GEOMETRY: &str = "generation3d←png: PNG is a raster image and a generation3d document is a BRep flow graph — no operator in it reads pixels, so there is no geometry to import. Import a mesh format (stl, obj, gltf, ply, dwg) or this artifact's own txt/json instead.";

pub fn register() {}

pub fn deserialize(_from: &PngSnapshot) -> Result<Generation3dSnapshot, store::TextError> {
    Err(io_error(NO_GEOMETRY))
}

pub fn deserialize_bytes(_bytes: &[u8]) -> Result<Generation3dSnapshot, store::TextError> {
    Err(io_error(NO_GEOMETRY))
}
