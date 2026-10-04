//! 🧪️ Laws of the empty wires parent vocabulary (design §20.15): board edits are child-lane graph leaves only.

use super::*;

/// ⚖️ LAW: the parent vocabulary declares no kind and no descriptor, and refuses every operation line and record.
#[test]
fn the_parent_vocabulary_is_empty_and_refuses_every_operation() {
    assert!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::kinds().is_empty());
    assert!(<WiresMutation as protocol::Mutation<WiresSnapshot>>::DESCRIPTORS.is_empty());
    assert!(<WiresMutation as protocol::OpText>::parse_op("move-node nodeId=\"node-1\" newX=1 newY=2").is_err());
    assert!(<WiresMutation as protocol::OpBinary>::decode_op(&[0]).is_err());
}
