//! 🧱️ 🧱️ Fem2d play app commands command — `add-bar`.

use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::{element_id, FemElement};
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
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
#[dsl(keyword = "add-bar")]
pub struct AddBar {
    pub start: String,
    pub end: String,
    pub material_id: String,
    pub section_id: String,
}

pub fn handle(payload: &AddBar, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.elements.iter().map(|e| element_id(e).to_string()), "e");
    let element = FemElement::Bar { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone() };
    Ok(Emit::mutations(vec![Fem2dMutation::CreateElement(crate::artifacts::fem2d::mutations::create_element::mutation::CreateElement { element: Box::new(element) })]))
}
