//! ↩️ Inverse of `CreateColumn`: the concrete `DeleteColumn` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_65::standards::v1::subsets::any::schema::mutations::delete_column::DeleteColumn;
use crate::standards::v1::subsets::any::schema::mutations::create_column::CreateColumn;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateColumn, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_65::standards::v1::subsets::any::schema::mutations::delete_column::DeleteColumn>,
{
    if base.columns.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteColumn { id: payload.id.clone() })]
}
