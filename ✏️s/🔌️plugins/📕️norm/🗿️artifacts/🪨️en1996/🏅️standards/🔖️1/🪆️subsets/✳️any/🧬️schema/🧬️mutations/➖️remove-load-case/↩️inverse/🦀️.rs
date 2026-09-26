use super::RemoveLoadCase;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
use crate::mutations::insert_load_case;
pub fn inverse(payload: &RemoveLoadCase, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() { Vec::new() } else { vec![En1996Mutation::InsertLoadCase(insert_load_case::InsertLoadCase { wall_index: payload.wall_index, index: payload.index, load_case: base.walls[payload.wall_index].load_cases[payload.index].clone() })] }
}
