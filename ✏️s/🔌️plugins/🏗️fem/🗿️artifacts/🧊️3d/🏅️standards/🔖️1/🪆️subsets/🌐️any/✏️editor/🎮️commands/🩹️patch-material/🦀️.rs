//! 🩹️ Fem3d play app command — `patch-material`: one-field edit of a material (`name`, `e`, `g`,
//! `nu`, `rho`) → `ReplaceMaterial`.

use crate::standards::v1::subsets::any::schema::mutations::replace_material;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

//#region 🔖️PatchMaterial
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-material")]
pub struct PatchMaterial {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchMaterial, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let material = doc.snapshot.materials.iter().find(|material| material.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.material-missing"))?;
    let mut new_material = material.clone();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.material-value")) };
    match payload.field.as_str() {
        "name" => new_material.name = payload.value.clone(),
        "e" => new_material.e = number()?,
        "g" => new_material.g = number()?,
        "nu" => new_material.nu = number()?,
        "rho" => new_material.rho = number()?,
        _ => return Err(Fault::from("fem3d.patch.material-field")),
    }
    if &new_material == material {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem3dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: payload.id.clone(), new_material })]))
}
//#endregion 🔖️PatchMaterial

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
