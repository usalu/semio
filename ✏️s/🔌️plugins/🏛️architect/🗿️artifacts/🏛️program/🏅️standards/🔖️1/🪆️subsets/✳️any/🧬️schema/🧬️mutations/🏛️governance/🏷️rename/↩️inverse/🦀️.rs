//! ↩️ Inverse (undo) construction for the `rename-governance` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏛️update-governance` per Wave C.

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

pub fn inverse<Op>(_payload: &super::RenameGovernance, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_governance::RenameGovernance>,
{
    Ok((|| {
    vec![Op::from(super::RenameGovernance { new_framework: base.governance.framework.clone() })]

    })())
}
