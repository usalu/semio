//! 🩹️ Fem2d play app command — `patch-region`: one-field edit of a meshed region (`name`, `thickness`, `meshSize`, `materialId`) → `ReplaceRegion`.

use crate::standards::v1::subsets::any::schema::mutations::replace_region;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️PatchRegion
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-region")]
pub struct PatchRegion {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Edits a region's scalar properties. The `outline`/`holes` polygons are deliberately not
/// reachable from a single-field patch — a polygon is edited in the viewport, not in a text box —
/// so the inspector shows their vertex counts read-only and every editable field lands here.
pub fn handle(payload: &PatchRegion, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let region = doc.snapshot.regions.iter().find(|region| region.id == payload.id).ok_or_else(|| Fault::from("fem2d.patch.region-missing"))?;
    let mut new_region = region.clone();
    match payload.field.as_str() {
        "name" => new_region.name = payload.value.clone(),
        "materialId" => new_region.material_id = payload.value.clone(),
        "thickness" => new_region.thickness = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.region-value"))?,
        "meshSize" => new_region.mesh_size = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.region-value"))?,
        _ => return Err(Fault::from("fem2d.patch.region-field")),
    }
    if &new_region == region {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem2dMutation::ReplaceRegion(replace_region::ReplaceRegion { id: payload.id.clone(), new_region })]))
}
//#endregion 🔖️PatchRegion

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
