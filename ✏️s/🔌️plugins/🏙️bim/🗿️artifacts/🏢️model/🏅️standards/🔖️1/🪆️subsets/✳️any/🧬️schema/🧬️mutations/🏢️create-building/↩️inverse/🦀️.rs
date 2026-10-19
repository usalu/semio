//! ↩️ Inverse of `CreateBuilding`: the concrete `DeleteBuilding` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_59::standards::v1::subsets::any::schema::mutations::delete_building::DeleteBuilding;
use crate::standards::v1::subsets::any::schema::mutations::create_building::CreateBuilding;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateBuilding, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_59::standards::v1::subsets::any::schema::mutations::delete_building::DeleteBuilding>,
{
    if base.buildings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteBuilding { id: payload.id.clone() })]
}
