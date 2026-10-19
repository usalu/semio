//! ↩️ Concrete create-option-group inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::create_option_group::CreateOptionGroup;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &CreateOptionGroup, _base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_option_group::DeleteOptionGroup>,
{
    vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_option_group::DeleteOptionGroup { id: payload.id.clone() })]
}
