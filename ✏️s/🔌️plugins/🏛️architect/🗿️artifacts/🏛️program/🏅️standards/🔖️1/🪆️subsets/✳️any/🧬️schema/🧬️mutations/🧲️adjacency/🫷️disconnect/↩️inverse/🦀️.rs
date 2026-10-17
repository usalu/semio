//! ↩️ Inverse (undo) construction for the `disconnect-adjacency` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧹clear-adjacency` per Wave C.

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo by reconnecting the captured edge. Missing target ⇒ nothing to undo.
pub fn inverse<Op>(payload: &super::DisconnectAdjacency, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_adjacency::ConnectAdjacency>,
{
    Ok((|| {
    match base.adjacencies.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::connect_adjacency::ConnectAdjacency { adjacency: base.adjacencies[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
