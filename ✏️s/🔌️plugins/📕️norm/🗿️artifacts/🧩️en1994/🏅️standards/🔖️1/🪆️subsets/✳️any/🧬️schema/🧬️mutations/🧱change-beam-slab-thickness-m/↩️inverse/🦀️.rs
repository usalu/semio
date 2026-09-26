//! Inverse for `change-beam-slab-thickness-m`.
use super::ChangeBeamSlabThicknessM;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeBeamSlabThicknessM, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeBeamSlabThicknessM(ChangeBeamSlabThicknessM { index: payload.index, new_slab_thickness_m: beam.slab_thickness_m })]
}
