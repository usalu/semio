//! Inverse for `change-beam-stud-fu-pa`.
use super::ChangeBeamStudFUPa;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeBeamStudFUPa, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeBeamStudFUPa(ChangeBeamStudFUPa { index: payload.index, new_f_u_pa: beam.studs.f_u_pa })]
}
