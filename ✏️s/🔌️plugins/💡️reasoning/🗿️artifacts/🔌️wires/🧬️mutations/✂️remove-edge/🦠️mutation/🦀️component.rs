//! ✂️ MindmapWires mutation — `RemoveEdge` apply delegate.
use crate::artifacts::wires::MindmapWiresDocument;
use crate::artifacts::wires::mutations::MindmapWiresMutation;

pub fn apply(projection: &mut MindmapWiresDocument, mutation: &MindmapWiresMutation) {
    crate::artifacts::wires::mutations::apply_mindmap_wires_mutation(projection, mutation);
}
