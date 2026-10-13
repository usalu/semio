//! ↩️ Inverse (undo) construction for the `rename-governance` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏛️update-governance` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

pub fn inverse(_payload: &super::RenameGovernance, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::RenameGovernance(super::RenameGovernance { new_framework: base.governance.framework.clone() })]

    })())
}
