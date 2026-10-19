//! ↩️ Inverse of `CreateRamp`: the concrete `DeleteRamp` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_92::standards::v1::subsets::any::schema::mutations::delete_ramp::DeleteRamp;
use crate::standards::v1::subsets::any::schema::mutations::create_ramp::CreateRamp;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateRamp, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_92::standards::v1::subsets::any::schema::mutations::delete_ramp::DeleteRamp>,
{
    if base.ramps.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteRamp { id: payload.id.clone() })]
}
