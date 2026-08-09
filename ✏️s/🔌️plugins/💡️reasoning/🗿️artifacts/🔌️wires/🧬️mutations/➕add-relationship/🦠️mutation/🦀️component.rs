//! ➕ MindmapWires mutation — `AddRelationship` apply delegate.
use crate::artifacts::wires::WiresSnapshot;
use crate::artifacts::wires::mutations::WiresMutation;

pub fn apply(projection: &mut WiresSnapshot, mutation: &WiresMutation) {
    crate::artifacts::wires::mutations::apply_wires_mutation(projection, mutation);
}
