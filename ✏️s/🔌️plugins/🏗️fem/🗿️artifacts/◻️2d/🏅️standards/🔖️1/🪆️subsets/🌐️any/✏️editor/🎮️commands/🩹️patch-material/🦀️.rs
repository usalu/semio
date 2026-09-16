//! 🩹️ Fem2d play app command — `patch-material`: one-field edit of a material (`name`, `e`, `nu`, `rho`) → `ReplaceMaterial`.

use crate::standards::v1::subsets::any::schema::mutations::replace_material;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️PatchMaterial
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-material")]
pub struct PatchMaterial {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Edits one isotropic-elasticity property (or the display name) of a material. The
/// plausibility guard on `replace-material` owns the admissible ranges — this handler only parses
/// and re-emits, so an inadmissible Poisson ratio is refused once, by the mutation, not twice.
pub fn handle(payload: &PatchMaterial, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let material = doc.snapshot.materials.iter().find(|material| material.id == payload.id).ok_or_else(|| Fault::from("fem2d.patch.material-missing"))?;
    let mut new_material = material.clone();
    match payload.field.as_str() {
        "name" => new_material.name = payload.value.clone(),
        "e" => new_material.e = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.material-value"))?,
        "nu" => new_material.nu = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.material-value"))?,
        "rho" => new_material.rho = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.material-value"))?,
        _ => return Err(Fault::from("fem2d.patch.material-field")),
    }
    if &new_material == material {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: payload.id.clone(), new_material })]))
}
//#endregion 🔖️PatchMaterial

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
