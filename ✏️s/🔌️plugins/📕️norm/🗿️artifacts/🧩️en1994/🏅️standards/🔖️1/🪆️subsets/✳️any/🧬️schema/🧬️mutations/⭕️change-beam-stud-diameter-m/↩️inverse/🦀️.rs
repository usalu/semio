//! Inverse for `change-beam-stud-diameter-m`.
use super::ChangeBeamStudDiameterM;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeBeamStudDiameterM, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeBeamStudDiameterM(ChangeBeamStudDiameterM { index: payload.index, new_diameter_m: beam.studs.diameter_m })]
}
