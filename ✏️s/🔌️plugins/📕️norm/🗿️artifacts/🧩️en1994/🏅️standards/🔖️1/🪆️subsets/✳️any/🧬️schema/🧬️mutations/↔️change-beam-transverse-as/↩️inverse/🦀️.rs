//! Inverse for `change-beam-transverse-as`.
use super::ChangeBeamTransverseAs;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeBeamTransverseAs, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeBeamTransverseAs(ChangeBeamTransverseAs { index: payload.index, new_transverse_as_m2_per_m: beam.transverse_as_m2_per_m })]
}
