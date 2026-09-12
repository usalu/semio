//! 🏋️ 🏋️ Fem2d play app commands command — `add-combination`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::create_combination;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::FemCombination;
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
#[dsl(keyword = "add-combination")]
pub struct AddCombination {
    pub name: String,
    pub terms: Vec<crate::FemCombinationTerm>,
}

pub fn handle(payload: &AddCombination, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.combinations.iter().map(|c| c.id.clone()), "c");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateCombination(create_combination::CreateCombination { combination: FemCombination { id, name: payload.name.clone(), terms: payload.terms.clone() } })]))
}
