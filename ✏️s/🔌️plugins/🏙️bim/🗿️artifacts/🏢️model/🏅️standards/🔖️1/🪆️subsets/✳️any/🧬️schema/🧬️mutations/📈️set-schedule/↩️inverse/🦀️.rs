//! ↩️ Inverse of \u00B6SetSchedule\u00B6: an absolute \u00B6SetSchedule\u00B6 restoring the base value of exactly the fields the forward really changes, none when the schedule is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_schedule::SetSchedule;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Patch;

pub fn inverse<Op>(payload: &SetSchedule, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_schedule::SetSchedule>,
{
    let Some(record) = base.schedules.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetSchedule::from_patch(payload.id.clone(), restore))]
}
