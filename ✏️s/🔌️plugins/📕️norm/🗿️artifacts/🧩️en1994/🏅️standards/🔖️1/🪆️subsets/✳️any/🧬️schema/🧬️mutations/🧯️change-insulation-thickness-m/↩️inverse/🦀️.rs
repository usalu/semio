//! Inverse for `change-insulation-thickness-m`.
use super::ChangeInsulationThicknessM;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(_payload: &ChangeInsulationThicknessM, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeInsulationThicknessM(ChangeInsulationThicknessM { new_insulation_thickness_m: base.insulation_thickness_m })]
}
