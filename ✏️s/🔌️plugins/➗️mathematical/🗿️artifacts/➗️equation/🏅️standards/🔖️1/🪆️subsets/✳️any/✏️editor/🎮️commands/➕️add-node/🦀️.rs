//! ➕️ Equation play app commands command — `add-node`: the guest's own add-node verb (design §13.3: adding a node is never
//! a `nodeGraphEdit` row), the ONE intent leaf `create-node` with the first free `n<k>` id at the requested position.

use crate::op::EquationMutation;
use crate::standards::v1::subsets::graph::schema::mutations::create_node::CreateNode;
use crate::{EquationGraph, EquationSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "add-node")]
pub struct AddNode {
    pub x: f64,
    pub y: f64,
}

/// ➕️ The `create-node` leaf `payload` means on `graph`: the first `n<k>` id (counting from the node count) the graph does
/// not hold, labelled by its upper-cased id; a non-finite position is refused by name.
pub(crate) fn add_node_leaves(payload: &AddNode, graph: &EquationGraph) -> Result<Vec<EquationMutation>, Fault> {
    if !payload.x.is_finite() || !payload.y.is_finite() {
        return Err(Fault::from("equation addNode needs a finite position"));
    }
    let id = (graph.nodes.len()..).map(|k| format!("n{k}")).find(|id| !graph.nodes.iter().any(|node| &node.id == id)).unwrap_or_default();
    Ok(vec![EquationMutation::CreateNode(CreateNode { label: id.to_uppercase(), id, x: payload.x, y: payload.y, index: None })])
}

pub fn handle(payload: &AddNode, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(add_node_leaves(payload, &doc.snapshot.graph.clone())?))
}
