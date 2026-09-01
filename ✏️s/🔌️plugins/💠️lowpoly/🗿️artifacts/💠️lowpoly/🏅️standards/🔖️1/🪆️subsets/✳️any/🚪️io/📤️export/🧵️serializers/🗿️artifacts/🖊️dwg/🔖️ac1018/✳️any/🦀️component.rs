//! lowpoly -> dwg
//!
//! 🐛️ Same pre-fix pack-envelope-mismatch defect class as the stl leaf (see that leaf's doc
//! comment for the shared root cause) -- always errored at runtime despite compiling and looking
//! real. `DwgSnapshot` is a full binary CAD document; synthesizing a valid one needs real mesh/CAD
//! geometry, unavailable at the `&LowpolySnapshot -> …` layer (see the stl leaf's doc comment for
//! why). Left as an HONEST stub pending that architecture work -- see this ticket's
//! `📝️io-implementation-result.md` handoff.
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::dwg::DwgSnapshot;

pub fn register() {}

pub fn serialize(_snapshot: &LowpolySnapshot) -> Result<DwgSnapshot, store::TextError> {
    Err(store::TextError::new("lowpoly->dwg: real mesh/CAD geometry is unavailable at the LowpolySnapshot layer (mesh is a content-addressed handle, not embedded geometry) -- not implemented", dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    serialize(snapshot).map(|_| Vec::new())
}
