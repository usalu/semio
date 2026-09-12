//! 🏋️ 🏋️ Fem2d play app commands command — `add-load-case`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::create_load_case;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::FemLoadCase;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-load-case")]
pub struct AddLoadCase {
    pub name: String,
    pub self_weight: bool,
}

pub fn handle(payload: &AddLoadCase, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.load_cases.iter().map(|lc| lc.id.clone()), "case-");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateLoadCase(create_load_case::CreateLoadCase { load_case: FemLoadCase { id, name: payload.name.clone(), loads: Vec::new(), self_weight: payload.self_weight } })]))
}
