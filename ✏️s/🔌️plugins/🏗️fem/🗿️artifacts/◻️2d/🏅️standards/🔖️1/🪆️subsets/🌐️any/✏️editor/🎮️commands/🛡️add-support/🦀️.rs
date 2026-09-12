//! 🧱️ 🧱️ Fem2d play app commands command — `add-support`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::{FemDof, FemSupport};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-support")]
pub struct AddSupport {
    pub node_id: String,
    pub fixed: Vec<FemDof>,
}

pub fn handle(payload: &AddSupport, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.supports.iter().map(|s| s.id.clone()), "sup");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateSupport(crate::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport { support: FemSupport { id, node_id: payload.node_id.clone(), fixed: payload.fixed.clone() } })]))
}
