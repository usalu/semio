//! ↩️ Concrete set-element-workset inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::set_element_workset::SetElementWorkset;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &SetElementWorkset, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_element_workset::SetElementWorkset>,
{
    vec![Op::from(SetElementWorkset { id: payload.id.clone(), workset: base.element_worksets.get(&payload.id).map(|membership| membership.target.clone()) })]
}
