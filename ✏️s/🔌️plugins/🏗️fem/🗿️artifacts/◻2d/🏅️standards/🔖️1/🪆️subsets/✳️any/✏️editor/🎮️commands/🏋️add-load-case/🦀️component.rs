//! 🏋️ 🏋️ Fem2d play app commands command — `add-load-case`.

use crate::artifacts::fem2d::mutations::create_load_case;
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::FemLoadCase;
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️AddNodalLoad
//#endregion 🔖️AddNodalLoad

//#region 🔖️AddMemberUdl
//#endregion 🔖️AddMemberUdl

//#region 🔖️AddAreaLoad
//#endregion 🔖️AddAreaLoad

//#region 🔖️AddLoadCase
//#endregion 🔖️AddLoadCase

//#region 🔖️AddCombination
//#endregion 🔖️AddCombination

//#region 🔖️SetSelfWeight
//#endregion 🔖️SetSelfWeight

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-load-case")]
pub struct AddLoadCase {
    pub name: String,
    pub self_weight: bool,
}

pub async fn handle(payload: &AddLoadCase, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.load_cases.iter().map(|lc| lc.id.clone()), "case-");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case: FemLoadCase { id, name: payload.name.clone(), loads: Vec::new(), self_weight: payload.self_weight } })]))
}
