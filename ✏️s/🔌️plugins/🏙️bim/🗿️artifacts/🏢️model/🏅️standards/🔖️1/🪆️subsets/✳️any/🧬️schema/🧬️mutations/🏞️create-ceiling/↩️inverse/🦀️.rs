//! ↩️ Inverse of `CreateCeiling`: the concrete `DeleteCeiling` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_90::standards::v1::subsets::any::schema::mutations::delete_ceiling::DeleteCeiling;
use crate::standards::v1::subsets::any::schema::mutations::create_ceiling::CreateCeiling;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateCeiling, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_90::standards::v1::subsets::any::schema::mutations::delete_ceiling::DeleteCeiling>,
{
    if base.ceilings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteCeiling { id: payload.id.clone() })]
}
