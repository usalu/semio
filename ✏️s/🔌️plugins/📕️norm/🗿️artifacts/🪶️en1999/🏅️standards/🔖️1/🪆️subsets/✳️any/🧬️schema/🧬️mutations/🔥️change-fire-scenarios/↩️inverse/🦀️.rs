//! ↩️ `change-fire-scenarios` inverse.

use crate::mutations::change_fire_scenarios::ChangeFireScenarios;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(_payload: &ChangeFireScenarios, base: &En1999Snapshot) -> Result<Vec<En1999Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1999Mutation::ChangeFireScenarios(ChangeFireScenarios { fire_scenarios: base.fire_scenarios.clone() })]

    })())
}
