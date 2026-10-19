//! 🩹️ FEM 3D material field commands lower to durable typed field intent.

use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::change_material::ChangeMaterial;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dMaterialPatch;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem3dSnapshot = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️PatchMaterial
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-material")]
pub struct PatchMaterial {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchMaterial, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let material = doc.snapshot.materials.iter().find(|material| material.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.material-missing"))?;
    let mut patch = Fem3dMaterialPatch::default();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.material-value")) };
    match payload.field.as_str() {
        "name" => patch.name = Some(payload.value.clone()),
        "e" => patch.e = Some(number()?),
        "g" => patch.g = Some(number()?),
        "nu" => patch.nu = Some(number()?),
        "rho" => patch.rho = Some(number()?),
        _ => return Err(Fault::from("fem3d.patch.material-field")),
    }
    if patch.candidate(material) == *material {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem3dMutation::ChangeMaterial(ChangeMaterial { id: payload.id.clone(), patch })]))
}
//#endregion 🔖️PatchMaterial

//#region 🧪️Tests
//#endregion 🧪️Tests
