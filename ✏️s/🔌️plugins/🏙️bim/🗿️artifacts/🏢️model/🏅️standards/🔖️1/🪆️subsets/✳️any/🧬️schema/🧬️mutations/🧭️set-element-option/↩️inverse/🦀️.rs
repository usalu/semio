//! ↩️ Concrete set-element-option inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::set_element_option::SetElementOption;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &SetElementOption, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_element_option::SetElementOption>,
{
    vec![Op::from(SetElementOption { id: payload.id.clone(), option: base.element_options.get(&payload.id).map(|membership| membership.target.clone()) })]
}
