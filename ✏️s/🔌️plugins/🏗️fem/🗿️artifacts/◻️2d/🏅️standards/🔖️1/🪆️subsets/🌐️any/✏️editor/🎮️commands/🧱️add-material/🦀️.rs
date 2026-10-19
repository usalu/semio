//! 🧱️ 🧱️ Fem2d play app commands command — `add-material`.

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_fem_material::planar::FemMaterialValue;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️AddNode
//#endregion 🔖️AddNode

//#region 🔖️AddBar
//#endregion 🔖️AddBar

//#region 🔖️AddBeam
//#endregion 🔖️AddBeam

//#region 🔖️AddMaterial
//#endregion 🔖️AddMaterial

//#region 🔖️AddSection
//#endregion 🔖️AddSection

//#region 🔖️AddSupport
//#endregion 🔖️AddSupport

//#region 🔖️AddRegion
//#endregion 🔖️AddRegion

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-material")]
pub struct AddMaterial {
    pub name: String,
    pub e: f64,
}

pub fn handle(payload: &AddMaterial, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let id = doc.operation()?.entity_id("m", "materials")?;
    Ok(Emit::mutations(vec![Fem2dMutation::CreateMaterial(semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial { material: FemMaterialValue { id, name: payload.name.clone(), e: payload.e, nu: 0.3, rho: 7850.0 }, index: None })]))
}
