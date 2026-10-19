//! 🩹️ Fem3d play app command — `patch-node`: one-field edit of a node (`x`, `y`, `z`) → `ChangeNode`.

use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::change_node::ChangeNode;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dNodePatch;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem3dSnapshot = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️PatchNode
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-node")]
pub struct PatchNode {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Moves one node ordinate. The inspector's number inputs carry their `Trigger::Change` value as
/// text, so the coordinate becomes one authored field in `ChangeNode`. Equal values emit no revision.
pub fn handle(payload: &PatchNode, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let row = doc.snapshot.nodes.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.node-missing"))?;
    let mut patch = Fem3dNodePatch::default();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.node-value")) };
    match payload.field.as_str() {
        "x" => patch.x = Some(number()?),
        "y" => patch.y = Some(number()?),
        "z" => patch.z = Some(number()?),
        _ => return Err(Fault::from("fem3d.patch.node-field")),
    }
    if patch.candidate(row) == *row { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![Fem3dMutation::ChangeNode(ChangeNode { id: payload.id.clone(), patch })]))
}

//#endregion 🔖️PatchNode

//#region 🧪️Tests
//#endregion 🧪️Tests
