//! ↩️ Concrete set-design-option inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::set_design_option::SetDesignOption;
use crate::standards::v1::subsets::any::schema::{diff::Patch, snapshot::ModelSnapshot};
pub fn inverse<Op>(payload: &SetDesignOption, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_design_option::SetDesignOption>,
{
    let Some(record) = base.design_options.get(&payload.id) else { return Vec::new(); };
    let patch = payload.patch().minimal(record).restoring(record);
    if patch.is_empty() { return Vec::new(); }
    vec![Op::from(SetDesignOption::from_patch(payload.id.clone(), patch))]
}
