//! \u21A9\uFE0F Inverse of \u00B6DeleteSchedule\u00B6: the concrete \u00B6CreateSchedule\u00B6 of the removed definition, none when the schedule is absent.

use super::super::create_schedule::CreateSchedule;
use super::DeleteSchedule;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteSchedule, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.schedules.get(&payload.id) {
        Some(schedule) => vec![ModelMutation::CreateSchedule(CreateSchedule { id: payload.id.clone(), schedule: schedule.clone() })],
        None => Vec::new(),
    }
}
