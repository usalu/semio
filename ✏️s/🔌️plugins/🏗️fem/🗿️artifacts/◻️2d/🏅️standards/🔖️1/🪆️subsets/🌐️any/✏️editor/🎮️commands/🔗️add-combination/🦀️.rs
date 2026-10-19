//! 🏋️ 🏋️ Fem2d play app commands command — `add-combination`.

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::create_combination;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombination;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-combination")]
pub struct AddCombination {
    pub name: String,
    pub terms: Vec<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombinationTerm>,
}

pub fn handle(payload: &AddCombination, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let id = doc.operation()?.entity_id("c", "combinations")?;
    Ok(Emit::mutations(vec![Fem2dMutation::CreateCombination(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::create_combination::CreateCombination { combination: FemCombination { id, name: payload.name.clone(), terms: payload.terms.clone() }, index: None })]))
}
