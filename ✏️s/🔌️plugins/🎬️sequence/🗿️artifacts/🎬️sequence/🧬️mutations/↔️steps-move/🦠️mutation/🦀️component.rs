//! ↔️steps-move `SequenceMutation` apply leaf.
use crate::artifacts::sequence::SequenceFixture;
use crate::artifacts::sequence::mutations::SequenceMutation;

pub fn apply(projection: &mut SequenceFixture, mutation: &SequenceMutation) {
    *projection = crate::artifacts::sequence::mutations::apply_sequence_mutation(projection, mutation);
}
