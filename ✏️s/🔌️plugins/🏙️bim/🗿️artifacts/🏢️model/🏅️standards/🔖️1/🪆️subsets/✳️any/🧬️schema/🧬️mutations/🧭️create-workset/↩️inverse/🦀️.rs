//! ↩️ Concrete create-workset inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::create_workset::CreateWorkset;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &CreateWorkset, _base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_workset::DeleteWorkset>,
{
    vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_workset::DeleteWorkset { id: payload.id.clone() })]
}
