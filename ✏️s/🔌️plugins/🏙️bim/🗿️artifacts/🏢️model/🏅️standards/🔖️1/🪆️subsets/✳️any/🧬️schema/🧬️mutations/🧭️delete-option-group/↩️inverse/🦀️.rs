//! ↩️ Concrete delete-option-group inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::delete_option_group::DeleteOptionGroup;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &DeleteOptionGroup, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_option_group::CreateOptionGroup>,
{
    let Some(record) = base.option_groups.get(&payload.id) else { return Vec::new(); };
    let mut result = Vec::new();
    result.push(Op::from(crate::standards::v1::subsets::any::schema::mutations::create_option_group::CreateOptionGroup { id: payload.id.clone(), option_group: record.clone() }));
    result
}
