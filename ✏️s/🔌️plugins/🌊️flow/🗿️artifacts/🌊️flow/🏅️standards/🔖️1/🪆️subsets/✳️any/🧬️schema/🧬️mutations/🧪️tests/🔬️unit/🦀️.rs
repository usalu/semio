//! 🧪️ Laws of the empty flow parent vocabulary (design §20.15): content edits are child-lane leaves only.

use super::*;

/// ⚖️ LAW: the parent vocabulary declares no kind and no descriptor, and refuses every operation line and record.
#[test]
fn the_parent_vocabulary_is_empty_and_refuses_every_operation() {
    assert!(<FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::kinds().is_empty());
    assert!(<FlowMutation as protocol::Mutation<FlowSnapshot>>::DESCRIPTORS.is_empty());
    assert!(<FlowMutation as protocol::OpText>::parse_op("create-widget index=0").is_err());
    assert!(<FlowMutation as protocol::OpBinary>::decode_op(&[0]).is_err());
}
