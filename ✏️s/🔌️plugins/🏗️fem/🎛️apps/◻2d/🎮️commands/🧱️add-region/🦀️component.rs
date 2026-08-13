//! 🧱️ 🧱️ Fem2d play app commands command — `add-region`.

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::{element_id, FemDof, FemElement, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
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
#[dsl(keyword = "add-region")]
pub struct AddRegion {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub material_id: String,
    pub thickness: Option<f64>,
    pub mesh_size: Option<f64>,
}

pub fn handle(payload: &AddRegion, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.regions.iter().map(|r| r.id.clone()), "r");
    let outline = vec![[payload.x, payload.y], [payload.x + payload.width, payload.y], [payload.x + payload.width, payload.y + payload.height], [payload.x, payload.y + payload.height]];
    let region = FemRegion { id, name: "Region".into(), outline, holes: Vec::new(), thickness: payload.thickness.unwrap_or(0.02), material_id: payload.material_id.clone(), mesh_size: payload.mesh_size.unwrap_or(0.25) };
    Ok(Emit::mutations(vec![Fem2dMutation::CreateRegion(crate::artifacts::fem2d::mutations::create_region::mutation::CreateRegion { region })]))
}
