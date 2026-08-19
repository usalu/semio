//! 🧱️ 🧱️ Fem2d play app commands command — `add-material`.

use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::FemMaterial;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-material")]
pub struct AddMaterial {
    pub name: String,
    pub e: f64,
}

pub async fn handle(payload: &AddMaterial, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.materials.iter().map(|m| m.id.clone()), "m");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateMaterial(crate::artifacts::fem2d::mutations::create_material::mutation::CreateMaterial { material: FemMaterial { id, name: payload.name.clone(), e: payload.e, nu: 0.3, rho: 7850.0 } })]))
}
