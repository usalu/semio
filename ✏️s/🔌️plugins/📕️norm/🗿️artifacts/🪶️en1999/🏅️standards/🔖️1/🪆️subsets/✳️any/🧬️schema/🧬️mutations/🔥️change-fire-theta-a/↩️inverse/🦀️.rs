//! ↩️ `change-fire-theta-a` inverse: restores the previous `theta_a`.

use crate::change_fire_theta_a::ChangeFireThetaA;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFireThetaA, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_theta_a::ChangeFireThetaA> {
    Ok(match base.fire_scenarios.iter().find(|row| row.id == payload.fire_scenario_id) {
        Some(row) => vec![Op::from(ChangeFireThetaA { fire_scenario_id: payload.fire_scenario_id.clone(), new_theta_a: row.theta_a })],
        None => Vec::new(),
    })
}
