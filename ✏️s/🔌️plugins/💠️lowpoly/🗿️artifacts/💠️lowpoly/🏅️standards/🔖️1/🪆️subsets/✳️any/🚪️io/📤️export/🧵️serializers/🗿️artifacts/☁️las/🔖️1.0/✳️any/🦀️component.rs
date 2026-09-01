//! lowpoly -> las
//!
//! 🐛️ Same pre-fix pack-envelope-mismatch defect class as the stl leaf (see that leaf's doc
//! comment for the shared root cause) -- always errored at runtime despite compiling and looking
//! real. `LasSnapshot` is a real LIDAR point-cloud document; synthesizing one needs real point
//! positions, which would have to come from resolved mesh vertex geometry, unavailable at the
//! `&LowpolySnapshot -> …` layer (see the stl leaf's doc comment for why). Left as an HONEST stub
//! pending that architecture work -- see this ticket's `📝️io-implementation-result.md` handoff.
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::las::LasSnapshot;

pub fn register() {}

pub fn serialize(_snapshot: &LowpolySnapshot) -> Result<LasSnapshot, store::TextError> {
    Err(store::TextError::new("lowpoly->las: real point-cloud geometry is unavailable at the LowpolySnapshot layer (mesh is a content-addressed handle, not embedded geometry) -- not implemented", dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    serialize(snapshot).map(|_| Vec::new())
}
