//! \u21A9\uFE0F Inverse of \u00B6DeleteSchedule\u00B6: the concrete \u00B6CreateSchedule\u00B6 of the removed definition, none when the schedule is absent.

use crate::standards::v1::subsets::any::schema::mutations::create_schedule::CreateSchedule;
use crate::standards::v1::subsets::any::schema::mutations::delete_schedule::DeleteSchedule;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteSchedule, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_schedule::CreateSchedule>,
{
    match base.schedules.get(&payload.id) {
        Some(schedule) => vec![Op::from(CreateSchedule { id: payload.id.clone(), schedule: schedule.clone() })],
        None => Vec::new(),
    }
}
