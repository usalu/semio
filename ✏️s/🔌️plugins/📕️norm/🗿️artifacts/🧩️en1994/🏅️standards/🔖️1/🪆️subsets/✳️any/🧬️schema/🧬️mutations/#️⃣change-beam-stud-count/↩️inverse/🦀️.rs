//! Inverse for `change-beam-stud-count`.
use super::ChangeBeamStudCount;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeBeamStudCount, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeBeamStudCount(ChangeBeamStudCount { index: payload.index, new_total_count: beam.studs.total_count })]
}
