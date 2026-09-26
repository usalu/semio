//! Inverse for `change-beam-construction`.
use super::ChangeBeamConstruction;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeBeamConstruction, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeBeamConstruction(ChangeBeamConstruction { index: payload.index, new_construction: beam.construction.clone() })]
}
