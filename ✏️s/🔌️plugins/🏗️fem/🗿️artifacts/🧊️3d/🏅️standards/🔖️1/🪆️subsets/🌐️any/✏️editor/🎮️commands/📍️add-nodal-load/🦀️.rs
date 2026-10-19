//! 🏋️ 🏋️ FEM 3D app commands command — `add-nodal-load`.

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::add_load;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_load_case;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

/// 🏋️ Resolves a named case exactly, or seeds one case from the admitted command authority.
pub(super) fn add_load_mutation(doc: &ArtifactView<'_, Fem3dSnapshot>, case_id: Option<&str>, load: FemLoad) -> Result<Fem3dMutation, Fault> {
    let target = case_id.or_else(|| doc.snapshot.load_cases.first().map(|case| case.id.as_str()));
    Ok(match target {
        Some(id) => Fem3dMutation::AddLoad(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::add_load::AddLoad { case_id: id.into(), load: Box::new(load), index: None }),
        None => Fem3dMutation::CreateLoadCase(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase { load_case: FemLoadCase { id: doc.operation()?.entity_id("case", "load_cases")?, name: "Load Case 1".into(), loads: vec![load], self_weight: false }, index: None }),
    })
}

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

// #region 🧪️Tests

// #endregion 🧪️Tests

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-nodal-load")]
pub struct AddNodalLoad {
    pub node_id: String,
    pub dof: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof,
    pub value: f64,
    pub case_id: Option<String>,
}

pub fn handle(payload: &AddNodalLoad, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let load_id = doc.operation()?.entity_id("l", "loads")?;
    let load = FemLoad::Nodal { id: load_id, node_id: payload.node_id.clone(), dof: payload.dof, value: payload.value };
    Ok(Emit::mutations(vec![add_load_mutation(doc, payload.case_id.as_deref(), load)?]))
}

