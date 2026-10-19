//! 🎮️ `layers`, `meshSize`, `materialId`, `axis`) → `ChangeSolid`.
//! 🩹️ Fem3d play app command — `patch-solid`: one-field edit of a solid (`name`, `baseZ`, `height`,

use semio_s_artifact_fem_3d_mesh_solids::standards::v1::subsets::any::schema::mutations::change_solid::ChangeSolid;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSolidPatch;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemAxis;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem3dSnapshot = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️PatchSolid
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-solid")]
pub struct PatchSolid {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchSolid, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let solid = doc.snapshot.solids.iter().find(|solid| solid.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.solid-missing"))?;
    let mut patch = Fem3dSolidPatch::default();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.solid-value")) };
    match payload.field.as_str() {
        "name" => patch.name = Some(payload.value.clone()),
        "baseZ" => patch.base_z = Some(number()?),
        "height" => patch.height = Some(number()?),
        "layers" => patch.layers = Some(payload.value.trim().parse::<usize>().map_err(|_| Fault::from("fem3d.patch.solid-value"))?),
        "meshSize" => patch.mesh_size = Some(number()?),
        "materialId" => patch.material_id = Some(payload.value.trim().to_string()),
        "axis" => patch.axis = Some(FemAxis::from_key(&payload.value).ok_or_else(|| Fault::from("fem3d.patch.solid-value"))?),
        _ => return Err(Fault::from("fem3d.patch.solid-field")),
    }
    if patch.name.as_ref().is_none_or(|value| value == &solid.name)
        && patch.base_z.is_none_or(|value| value == solid.base_z)
        && patch.height.is_none_or(|value| value == solid.height)
        && patch.layers.is_none_or(|value| value == solid.layers)
        && patch.mesh_size.is_none_or(|value| value == solid.mesh_size)
        && patch.material_id.as_ref().is_none_or(|value| value == &solid.material_id)
        && patch.axis.as_ref().is_none_or(|value| value == &solid.axis)
    {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem3dMutation::ChangeSolid(ChangeSolid { id: payload.id.clone(), patch })]))
}
//#endregion 🔖️PatchSolid

//#region 🧪️Tests
//#endregion 🧪️Tests
