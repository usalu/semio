//! 🔺️ `create-node` — sparse diff construction.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationNode, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is Fatal `duplicate-id` — an id-keyed entity that already exists cannot be
/// "created" again.
pub async fn diff(payload: &super::CreateNode, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = equation_graph(base);
    if graph.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    graph.nodes.push(EquationNode { id: payload.id.clone(), label: payload.label.clone(), x: payload.x, y: payload.y });
    let (notation, results, computed) = equation_children_from_state(&graph, &equation_geometry(base));
    protocol::MutationOutcome::new(EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
