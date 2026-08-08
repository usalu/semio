//! ➖edges-remove `SequenceMutation` inverse leaf.
use crate::artifacts::sequence::SequenceFixture;
use crate::artifacts::sequence::mutations::SequenceMutation;
use protocol::Mutation;

pub fn inverse(base: &SequenceFixture, mutation: &SequenceMutation) -> Vec<SequenceMutation> {
    <SequenceMutation as Mutation<SequenceFixture>>::inverse(mutation, base)
}
