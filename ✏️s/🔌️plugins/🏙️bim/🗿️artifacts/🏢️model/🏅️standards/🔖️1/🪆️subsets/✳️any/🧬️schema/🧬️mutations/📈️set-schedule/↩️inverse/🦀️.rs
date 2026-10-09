//! \u21A9\uFE0F Inverse of \u00B6SetSchedule\u00B6: an absolute \u00B6SetSchedule\u00B6 restoring the base value of exactly the fields the forward really changes, none when the schedule is absent or nothing changes.

use super::SetSchedule;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetSchedule, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.schedules.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetSchedule(SetSchedule::from_patch(payload.id.clone(), restore))]
}
