//! 🏋️ 🏋️ Fem2d play app commands command — `add-combination`.

use crate::artifacts::fem2d::mutations::create_combination;
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::FemCombination;
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-combination")]
pub struct AddCombination {
    pub name: String,
    pub terms: Vec<crate::artifacts::fem2d::FemCombinationTerm>,
}

pub fn handle(payload: &AddCombination, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.combinations.iter().map(|c| c.id.clone()), "c");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateCombination(create_combination::mutation::CreateCombination { combination: FemCombination { id, name: payload.name.clone(), terms: payload.terms.clone() } })]))
}
