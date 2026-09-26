//! Inverse for `change-beam-stud-spacing-m`.
use super::ChangeBeamStudSpacingM;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeBeamStudSpacingM, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeBeamStudSpacingM(ChangeBeamStudSpacingM { index: payload.index, new_spacing_m: beam.studs.spacing_m })]
}
