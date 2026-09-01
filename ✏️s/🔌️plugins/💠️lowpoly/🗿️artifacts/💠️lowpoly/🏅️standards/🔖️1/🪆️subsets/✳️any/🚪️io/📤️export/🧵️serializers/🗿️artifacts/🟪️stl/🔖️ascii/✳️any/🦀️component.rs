//! lowpoly -> stl
//!
//! 🐛️ Pre-fix content round-tripped `LowpolySnapshot::encode_pack` bytes (envelope id
//! `lowpoly.lowpoly`) straight into `StlSnapshot::decode_pack`, which unconditionally rejects any
//! envelope id other than its own `stdio.stl` (see that type's `decode_pack_with`) -- this always
//! threw `PackError::Schema("pack envelope mismatch: ...")` at runtime despite compiling and
//! looking real (same defect class fixed for real on the txt/obj/png/ply lowpoly IO leaves --
//! see `../../🧊️obj/🔖️3.0/✳️any/🦀️component.rs`'s doc comment for the shared root cause).
//!
//! Unlike obj/ply, real ASCII STL (`solid … facet … endsolid`) has no per-line "unknown statement"
//! or comment retention slot in `StlSnapshot` to smuggle a carrier payload through, so this cannot
//! be fixed the same way without inventing a second bespoke grammar (explicitly against this
//! ticket's rules) or resolving real mesh geometry -- which needs a store/session handle to follow
//! `LowpolyObject.mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>` to its content, not
//! available to a synchronous `&LowpolySnapshot -> …` function. Left as an HONEST stub (never a
//! silent pack-envelope lie) pending that architecture work -- see this ticket's
//! `📝️io-implementation-result.md` handoff section.
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::stl::StlSnapshot;

pub fn register() {}

pub fn serialize(_snapshot: &LowpolySnapshot) -> Result<StlSnapshot, store::TextError> {
    Err(store::TextError::new("lowpoly->stl: real mesh geometry is unavailable at the LowpolySnapshot layer (mesh is a content-addressed handle, not embedded geometry) -- not implemented", dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    serialize(snapshot).map(|_| Vec::new())
}
