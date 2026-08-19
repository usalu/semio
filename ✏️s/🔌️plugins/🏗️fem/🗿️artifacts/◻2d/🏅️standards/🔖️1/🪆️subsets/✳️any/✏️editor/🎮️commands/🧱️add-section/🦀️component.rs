//! 🧱️ 🧱️ Fem2d play app commands command — `add-section`.

use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::FemSection;
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
#[dsl(keyword = "add-section")]
pub struct AddSection {
    pub name: String,
    pub area: f64,
    pub iy: f64,
}

pub async fn handle(payload: &AddSection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.sections.iter().map(|s| s.id.clone()), "s");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateSection(crate::artifacts::fem2d::mutations::create_section::mutation::CreateSection { section: FemSection { id, name: payload.name.clone(), area: payload.area, iy: payload.iy } })]))
}
