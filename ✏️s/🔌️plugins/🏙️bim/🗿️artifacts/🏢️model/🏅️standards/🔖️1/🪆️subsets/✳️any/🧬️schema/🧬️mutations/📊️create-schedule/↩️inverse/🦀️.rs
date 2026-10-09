//! \u21A9\uFE0F Inverse of \u00B6CreateSchedule\u00B6: the concrete \u00B6DeleteSchedule\u00B6 of the id it created, none when the id was already taken.

use super::super::delete_schedule::DeleteSchedule;
use super::CreateSchedule;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateSchedule, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.schedules.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteSchedule(DeleteSchedule { id: payload.id.clone() })]
}
