//! ↩️ Inverse of `CreateZone`: the concrete `DeleteZone` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_99::standards::v1::subsets::any::schema::mutations::delete_zone::DeleteZone;
use crate::standards::v1::subsets::any::schema::mutations::create_zone::CreateZone;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateZone, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_99::standards::v1::subsets::any::schema::mutations::delete_zone::DeleteZone>,
{
    if semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id).is_some() {
        return Vec::new();
    }
    vec![Op::from(DeleteZone { id: payload.id.clone() })]
}
