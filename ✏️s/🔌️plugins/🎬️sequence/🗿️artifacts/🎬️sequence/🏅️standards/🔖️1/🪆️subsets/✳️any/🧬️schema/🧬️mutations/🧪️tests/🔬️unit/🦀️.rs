//! 🧪️ Laws of the empty sequence parent vocabulary (design §20.15): content edits are child-lane leaves only.

use super::*;

/// ⚖️ LAW: the parent vocabulary declares no kind and no descriptor, and refuses every operation line and record.
#[test]
fn the_parent_vocabulary_is_empty_and_refuses_every_operation() {
    assert!(<SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::kinds().is_empty());
    assert!(<SequenceMutation as protocol::Mutation<SequenceSnapshot>>::DESCRIPTORS.is_empty());
    assert!(<SequenceMutation as protocol::OpText>::parse_op("move-step id=\"step-1\" x=1 y=2").is_err());
    assert!(<SequenceMutation as protocol::OpBinary>::decode_op(&[0]).is_err());
}
