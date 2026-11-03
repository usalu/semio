//! ↩️ `insert-fire-scenario` inverse.

use crate::insert_fire_scenario::InsertFireScenario;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_fire_scenario;

pub fn inverse<Op>(payload: &InsertFireScenario, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fire_scenario::RemoveFireScenario> {
    Ok((|| {
    vec![Op::from(remove_fire_scenario::RemoveFireScenario { id: payload.fire_scenario.id.clone() })]

    })())
}
