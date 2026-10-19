//! 🏋️ 🏋️ FEM 3D app commands command — `add-member-udl`.

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

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
#[dsl(keyword = "add-member-udl")]
pub struct AddMemberUdl {
    pub element_id: String,
    pub wx: f64,
    pub wy: f64,
    pub wz: f64,
    pub case_id: Option<String>,
}

pub fn handle(payload: &AddMemberUdl, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let load_id = doc.operation()?.entity_id("l", "loads")?;
    let load = FemLoad::MemberUdl { id: load_id, element_id: payload.element_id.clone(), wx: payload.wx, wy: payload.wy, wz: payload.wz };
    Ok(Emit::mutations(vec![super::add_nodal_load::add_load_mutation(doc, payload.case_id.as_deref(), load)?]))
}
