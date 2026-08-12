//! ↩️ Inverse (undo) construction for the `create-human-factor-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧠human-factors` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateHumanFactorRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteHumanFactorRequirement(super::super::delete_human_factor_requirement::mutation::DeleteHumanFactorRequirement { id: payload.human_factor_requirement.header.id.clone() })]
}
