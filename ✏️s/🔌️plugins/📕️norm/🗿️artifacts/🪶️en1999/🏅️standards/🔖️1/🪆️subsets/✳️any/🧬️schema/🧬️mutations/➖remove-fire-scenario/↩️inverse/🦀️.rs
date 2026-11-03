//! ↩️ `remove-fire-scenario` inverse.

use crate::remove_fire_scenario::RemoveFireScenario;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_fire_scenario;

pub fn inverse<Op>(payload: &RemoveFireScenario, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fire_scenario::InsertFireScenario> {
    Ok((|| {
    if let Some((index, row)) = base.fire_scenarios.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_fire_scenario::InsertFireScenario { index: index as u32, fire_scenario: row.clone() })]
    } else { Vec::new() }

    })())
}
