//! ↔️edges-move `SequenceMutation` apply leaf.
use crate::artifacts::sequence::SequenceSnapshot;
use crate::artifacts::sequence::mutations::SequenceMutation;

pub fn apply(projection: &mut SequenceSnapshot, mutation: &SequenceMutation) {
    *projection = crate::artifacts::sequence::mutations::apply_sequence_mutation(projection, mutation);
}
