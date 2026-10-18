//! \u21A9\uFE0F Inverse of \u00B6CreateSchedule\u00B6: the concrete \u00B6DeleteSchedule\u00B6 of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_schedule::DeleteSchedule;
use crate::standards::v1::subsets::any::schema::mutations::create_schedule::CreateSchedule;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateSchedule, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_schedule::DeleteSchedule>,
{
    if base.schedules.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteSchedule { id: payload.id.clone() })]
}
