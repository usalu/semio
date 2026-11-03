//! ↩️ `change-fire-duration-s` inverse: restores the previous `duration_s`.

use crate::change_fire_duration_s::ChangeFireDurationS;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFireDurationS, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_duration_s::ChangeFireDurationS> {
    Ok(match base.fire_scenarios.iter().find(|row| row.id == payload.fire_scenario_id) {
        Some(row) => vec![Op::from(ChangeFireDurationS { fire_scenario_id: payload.fire_scenario_id.clone(), new_duration_s: row.duration_s })],
        None => Vec::new(),
    })
}
