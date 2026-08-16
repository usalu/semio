//! 🔺️ `create-node` — sparse diff construction.

use super::mutation::CreateNode;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalNode, MathematicalSnapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is Fatal `duplicate-id` — an id-keyed entity that already exists cannot be
/// "created" again.
pub fn diff(payload: &CreateNode, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    let mut graph = mathematical_graph(base);
    if graph.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    graph.nodes.push(MathematicalNode { id: payload.id.clone(), label: payload.label.clone(), x: payload.x, y: payload.y });
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    protocol::MutationOutcome::new(MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
