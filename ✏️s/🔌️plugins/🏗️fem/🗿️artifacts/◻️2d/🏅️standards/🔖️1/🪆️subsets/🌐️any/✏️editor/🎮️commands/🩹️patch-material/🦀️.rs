//! 🩹️ FEM 2D material field commands lower to durable typed field intent.

use semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::change_material::ChangeMaterial;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dMaterialPatch;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️PatchMaterial
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-material")]
pub struct PatchMaterial {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Parses one property; the semantic mutation validates its range against the replay base.
pub fn handle(payload: &PatchMaterial, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let material = doc.snapshot.materials.iter().find(|material| material.id.as_str() == payload.id.as_str()).ok_or_else(|| Fault::from("fem2d.patch.material-missing"))?;
    let material = semio_s_fem_material::planar::FemMaterialValue { id: material.id.to_string(), name: material.name.to_string(), e: material.e, nu: material.nu, rho: material.rho };
    let mut patch = Fem2dMaterialPatch::default();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.material-value")) };
    match payload.field.as_str() {
        "name" => patch.name = Some(payload.value.clone()),
        "e" => patch.e = Some(number()?),
        "nu" => patch.nu = Some(number()?),
        "rho" => patch.rho = Some(number()?),
        _ => return Err(Fault::from("fem2d.patch.material-field")),
    }
    if patch.candidate(&material) == material {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem2dMutation::ChangeMaterial(ChangeMaterial { id: payload.id.clone(), patch })]))
}
//#endregion 🔖️PatchMaterial

//#region 🧪️Tests
//#endregion 🧪️Tests
