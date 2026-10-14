//! ↩️ `change-fire-scenarios` inverse.

use crate::change_fire_scenarios::ChangeFireScenarios;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(_payload: &ChangeFireScenarios, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_scenarios::ChangeFireScenarios> {
    Ok((|| {
    vec![Op::from(ChangeFireScenarios { fire_scenarios: base.fire_scenarios.clone() })]

    })())
}
