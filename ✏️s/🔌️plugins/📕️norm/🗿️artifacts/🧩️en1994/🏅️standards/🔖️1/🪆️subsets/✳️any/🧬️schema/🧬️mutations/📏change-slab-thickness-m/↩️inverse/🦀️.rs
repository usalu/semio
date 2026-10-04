//! Inverse for `change-slab-thickness-m`.
use super::ChangeSlabThicknessM;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeSlabThicknessM, base: &En1994Snapshot) -> Result<Vec<En1994Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(slab) = base.slabs.get(payload.index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeSlabThicknessM(ChangeSlabThicknessM { index: payload.index, new_concrete_thickness_m: slab.concrete_thickness_m })]

    })())
}
