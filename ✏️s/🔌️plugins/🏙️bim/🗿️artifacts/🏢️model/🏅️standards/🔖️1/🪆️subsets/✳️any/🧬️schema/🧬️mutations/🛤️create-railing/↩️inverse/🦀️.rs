//! ↩️ Inverse of `CreateRailing`: the concrete `DeleteRailing` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_62::standards::v1::subsets::any::schema::mutations::delete_railing::DeleteRailing;
use crate::standards::v1::subsets::any::schema::mutations::create_railing::CreateRailing;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateRailing, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_62::standards::v1::subsets::any::schema::mutations::delete_railing::DeleteRailing>,
{
    if base.railings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteRailing { id: payload.id.clone() })]
}
