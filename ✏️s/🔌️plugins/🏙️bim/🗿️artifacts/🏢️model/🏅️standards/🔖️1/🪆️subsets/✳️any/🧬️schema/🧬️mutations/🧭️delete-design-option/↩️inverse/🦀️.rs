//! ↩️ Concrete delete-design-option inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::delete_design_option::DeleteDesignOption;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &DeleteDesignOption, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_design_option::CreateDesignOption> + From<crate::standards::v1::subsets::any::schema::mutations::set_element_option::SetElementOption>,
{
    let Some(record) = base.design_options.get(&payload.id) else { return Vec::new(); };
    let mut result = Vec::new();
    for (id, member) in &base.element_options { if member.target == payload.id { result.push(Op::from(crate::standards::v1::subsets::any::schema::mutations::set_element_option::SetElementOption { id: id.clone(), option: Some(member.target.clone()) })); } }
    result.push(Op::from(crate::standards::v1::subsets::any::schema::mutations::create_design_option::CreateDesignOption { id: payload.id.clone(), design_option: record.clone() }));
    result
}
