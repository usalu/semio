//! 🧱️ 🧱️ FEM 3D app commands command — `add-node`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-node")]
pub struct AddNode {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn handle(payload: &AddNode, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = semio_s_fem_app_surface::next_id(snapshot.nodes.iter().map(|n| n.id.clone()), "n");
    Ok(Emit::mutations(vec![Fem3dMutation::CreateNode(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node::CreateNode { node: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemNode { id, x: payload.x, y: payload.y, z: payload.z }, index: None })]))
}

