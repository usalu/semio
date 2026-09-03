//! 🔺️ `change-node-label` — sparse diff construction.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::ChangeNodeLabel, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = equation_graph(base);
    let Some(existing) = graph.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" already has label \"{}\".", payload.id, payload.new_label));
    }
    if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.label = payload.new_label.clone();
    }
    let (notation, results, computed) = equation_children_from_state(&graph, &equation_geometry(base));
    protocol::MutationOutcome::new(EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
