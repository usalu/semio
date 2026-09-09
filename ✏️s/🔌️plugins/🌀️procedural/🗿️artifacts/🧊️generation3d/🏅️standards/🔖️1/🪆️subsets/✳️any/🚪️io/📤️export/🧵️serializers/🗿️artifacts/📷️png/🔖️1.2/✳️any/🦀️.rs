//! 📷️ generation3d → `s.stdio.png@1.2` — not this artifact's claim.
//!
//! 🖼️ generation2d owns the procedural plugin's `stdio.png` EXPORT claim
//! (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3, the documented tie-break recorded in
//! `🚪️io/🦀️.rs`'s `🚪️IoRegistry` region), which is why `export_stdio_kinds` omits png and no
//! `compose_export_png` entry exists here. This leaf is therefore unreachable from the composer
//! registry by design.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END it nonetheless returned
//! `print_dsl(snapshot).into_bytes()` — the artifact's own DSL text, mislabelled `.png`. Nothing
//! called it, but a function that answers "here are your PNG bytes" with a text document is a trap
//! for the next caller, so it now states the ownership rule instead of fabricating an answer.
use crate::standards::v1::subsets::any::io::mesh_bridge::io_error;
use crate::Generation3dSnapshot;

/// 🚫️ Why png export does not live here, in one sentence the UI can show verbatim.
const NOT_OWNED: &str = "generation3d→png: the procedural plugin's png export claim belongs to s.procedural.generation2d, not to this artifact. Export a mesh format (stl, obj, gltf, ply, dwg, las) or this artifact's own txt/json instead.";

pub fn register() {}

pub fn serialize_bytes(_snapshot: &Generation3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Err(io_error(NOT_OWNED))
}
