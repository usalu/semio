//! 🩹️ Fem2d play app command — `patch-node`: one-field edit of a node (`x`, `y`) → `ReplaceNode`.

use crate::standards::v1::subsets::any::schema::mutations::replace_node;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️PatchNode
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-node")]
pub struct PatchNode {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Moves one node ordinate. The inspector's number inputs carry their `Trigger::Change` value as
/// text, so the coordinate is parsed here and the whole record is re-emitted through the only
/// in-history spelling of a node move, `ReplaceNode`. A value equal to the current one emits nothing
/// rather than a no-op revision.
pub fn handle(payload: &PatchNode, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let node = doc.snapshot.nodes.iter().find(|node| node.id == payload.id).ok_or_else(|| Fault::from("fem2d.patch.node-missing"))?;
    let mut new_node = node.clone();
    let ordinate = match payload.field.as_str() {
        "x" => &mut new_node.x,
        "y" => &mut new_node.y,
        _ => return Err(Fault::from("fem2d.patch.node-field")),
    };
    *ordinate = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.node-value"))?;
    if &new_node == node {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem2dMutation::ReplaceNode(replace_node::ReplaceNode { id: payload.id.clone(), new_node })]))
}
//#endregion 🔖️PatchNode

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
