//! 🩹️ Fem3d play app command — `patch-solid`: one-field edit of a solid (`name`, `baseZ`, `height`,
//! `layers`, `meshSize`, `materialId`, `axis`) → `ReplaceSolid`.

use crate::standards::v1::subsets::any::schema::mutations::replace_solid;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::FemAxis;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

//#region 🔖️PatchSolid
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-solid")]
pub struct PatchSolid {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchSolid, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let solid = doc.snapshot.solids.iter().find(|solid| solid.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.solid-missing"))?;
    let mut new_solid = solid.clone();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.solid-value")) };
    match payload.field.as_str() {
        "name" => new_solid.name = payload.value.clone(),
        "baseZ" => new_solid.base_z = number()?,
        "height" => new_solid.height = number()?,
        "layers" => new_solid.layers = number()?.max(0.0) as usize,
        "meshSize" => new_solid.mesh_size = number()?,
        "materialId" => new_solid.material_id = payload.value.trim().to_string(),
        "axis" => new_solid.axis = FemAxis::from_key(&payload.value).ok_or_else(|| Fault::from("fem3d.patch.solid-value"))?,
        _ => return Err(Fault::from("fem3d.patch.solid-field")),
    }
    if &new_solid == solid {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem3dMutation::ReplaceSolid(replace_solid::ReplaceSolid { id: payload.id.clone(), new_solid })]))
}
//#endregion 🔖️PatchSolid

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
