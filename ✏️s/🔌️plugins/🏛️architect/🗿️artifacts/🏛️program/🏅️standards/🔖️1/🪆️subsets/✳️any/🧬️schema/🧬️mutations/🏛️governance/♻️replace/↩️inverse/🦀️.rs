//! ↩️ Inverse (undo) construction for the `replace-governance` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏛️update-governance` per Wave C.

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

pub fn inverse<Op>(_payload: &super::ReplaceGovernance, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_governance::ReplaceGovernance>,
{
    Ok((|| {
    vec![Op::from(super::ReplaceGovernance { new_governance: base.governance.clone() })]

    })())
}
