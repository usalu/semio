//! 🏋️ 🏋️ FEM 3D app commands command — `add-load-case`.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::mutations::{add_load, change_load_case_self_weight, create_combination, create_load_case};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemLoad, FemLoadCase};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-load-case")]
pub struct AddLoadCase {
    pub name: String,
    pub self_weight: bool,
}

pub fn handle(payload: &AddLoadCase, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.load_cases.iter().map(|lc| lc.id.clone()), "case-");
    Ok(Emit::mutations(vec![Fem3dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case: FemLoadCase { id, name: payload.name.clone(), loads: Vec::new(), self_weight: payload.self_weight } })]))
}
