//! ➖edges-remove `SequenceMutation` inverse leaf.
use crate::artifacts::sequence::SequenceSnapshot;
use crate::artifacts::sequence::mutations::SequenceMutation;
use protocol::Mutation;

pub fn inverse(base: &SequenceSnapshot, mutation: &SequenceMutation) -> Vec<SequenceMutation> {
    <SequenceMutation as Mutation<SequenceSnapshot>>::inverse(mutation, base)
}
