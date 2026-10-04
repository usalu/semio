//! ↩️ Inverse (undo) construction for the `rename-governance` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏛️update-governance` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

pub fn inverse(_payload: &super::RenameGovernance, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::RenameGovernance(super::RenameGovernance { new_framework: base.governance.framework.clone() })]

    })())
}
