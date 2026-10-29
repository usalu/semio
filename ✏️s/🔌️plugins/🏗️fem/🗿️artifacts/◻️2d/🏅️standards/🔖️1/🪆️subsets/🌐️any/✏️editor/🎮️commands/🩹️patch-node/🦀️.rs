//! 🩹️ Fem2d play app command — `patch-node`: one-field edit of a node (`x`, `y`) → `ChangeNode`.

use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::change_node::ChangeNode;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodePatch;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️PatchNode
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-node")]
pub struct PatchNode {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Moves one node ordinate. The inspector's number inputs carry their `Trigger::Change` value as
/// text, so the parsed value authors only that ordinate through `ChangeNode`. An unchanged value
/// emits no history revision.
pub fn handle(payload: &PatchNode, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let row = doc.snapshot.nodes.iter().find(|row| row.id.as_str() == payload.id.as_str()).ok_or_else(|| Fault::from("fem2d.patch.node-missing"))?;
    let row = semio_s_fem_node::planar::FemNodeValue { id: row.id.to_string(), x: row.x, y: row.y };
    let mut patch = Fem2dNodePatch::default();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.node-value")) };
    match payload.field.as_str() {
        "x" => patch.x = Some(number()?),
        "y" => patch.y = Some(number()?),
        _ => return Err(Fault::from("fem2d.patch.node-field")),
    }
    if patch.candidate(&row) == row { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![Fem2dMutation::ChangeNode(ChangeNode { id: payload.id.clone(), patch })]))
}

//#endregion 🔖️PatchNode

//#region 🧪️Tests
//#endregion 🧪️Tests
