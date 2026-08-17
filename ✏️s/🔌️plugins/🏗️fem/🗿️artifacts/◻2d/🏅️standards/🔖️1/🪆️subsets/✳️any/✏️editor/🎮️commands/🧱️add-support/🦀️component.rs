//! 🧱️ 🧱️ Fem2d play app commands command — `add-support`.

use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
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
#[dsl(keyword = "add-support")]
pub struct AddSupport {
    pub node_id: String,
    pub fixed: Vec<FemDof>,
}

pub fn handle(payload: &AddSupport, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.supports.iter().map(|s| s.id.clone()), "sup");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateSupport(crate::artifacts::fem2d::mutations::create_support::mutation::CreateSupport { support: FemSupport { id, node_id: payload.node_id.clone(), fixed: payload.fixed.clone() } })]))
}
