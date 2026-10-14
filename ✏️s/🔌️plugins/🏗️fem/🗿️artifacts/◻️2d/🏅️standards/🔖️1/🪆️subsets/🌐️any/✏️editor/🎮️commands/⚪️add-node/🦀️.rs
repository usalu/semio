//! 🧱️ 🧱️ Fem2d play app commands command — `add-node`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemNode;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-node")]
pub struct AddNode {
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &AddNode, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = semio_s_fem_app_surface::next_id(snapshot.nodes.iter().map(|n| n.id.clone()), "n");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateNode(semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node::CreateNode { node: FemNode { id, x: payload.x, y: payload.y }, index: None })]))
}

//#region 🧪️Tests
//#endregion 🧪️Tests
