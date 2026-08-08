use crate::artifacts::wires::diff::MindmapWiresDiff;
use crate::artifacts::wires::MindmapWiresDocument;
use crate::artifacts::wires::mutations::MindmapWiresMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &MindmapWiresMutation, base: &MindmapWiresDocument) -> MindmapWiresDiff {
    <MindmapWiresMutation as protocol::Mutation<MindmapWiresDocument>>::diff(mutation, base)
}
