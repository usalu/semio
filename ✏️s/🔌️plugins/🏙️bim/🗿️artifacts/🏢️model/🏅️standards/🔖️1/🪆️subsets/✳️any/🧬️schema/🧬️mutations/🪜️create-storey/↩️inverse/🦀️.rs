//! ↩️ Inverse of `CreateStorey`: the concrete `DeleteStorey` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_60::standards::v1::subsets::any::schema::mutations::delete_storey::DeleteStorey;
use crate::standards::v1::subsets::any::schema::mutations::create_storey::CreateStorey;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateStorey, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_60::standards::v1::subsets::any::schema::mutations::delete_storey::DeleteStorey>,
{
    if base.storeys.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteStorey { id: payload.id.clone() })]
}
