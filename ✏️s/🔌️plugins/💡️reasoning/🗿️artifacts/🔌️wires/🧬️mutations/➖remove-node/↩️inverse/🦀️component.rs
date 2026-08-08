use crate::artifacts::wires::MindmapWiresDocument;
use crate::artifacts::wires::mutations::MindmapWiresMutation;

pub fn inverse(base: &MindmapWiresDocument, mutation: &MindmapWiresMutation) -> Vec<MindmapWiresMutation> {
    <MindmapWiresMutation as protocol::Mutation<MindmapWiresDocument>>::inverse(mutation, base)
}
