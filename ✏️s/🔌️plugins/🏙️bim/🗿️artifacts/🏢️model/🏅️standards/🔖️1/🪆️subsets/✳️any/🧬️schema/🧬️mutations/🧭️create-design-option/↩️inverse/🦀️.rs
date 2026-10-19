//! ↩️ Concrete create-design-option inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::create_design_option::CreateDesignOption;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &CreateDesignOption, _base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_design_option::DeleteDesignOption>,
{
    vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_design_option::DeleteDesignOption { id: payload.id.clone() })]
}
