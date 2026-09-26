use super::InsertLoadCase;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
use crate::mutations::remove_load_case;
pub fn inverse(payload: &InsertLoadCase, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::RemoveLoadCase(remove_load_case::RemoveLoadCase { wall_index: payload.wall_index, index: payload.index })]
}
