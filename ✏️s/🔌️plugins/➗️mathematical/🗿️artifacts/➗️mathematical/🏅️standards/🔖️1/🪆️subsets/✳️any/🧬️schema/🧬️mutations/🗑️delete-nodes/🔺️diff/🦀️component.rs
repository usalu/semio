//! 🔺️ `delete-nodes` — sparse diff construction, cascading to every incident edge.

use super::mutation::DeleteNodes;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &DeleteNodes, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    let mut graph = mathematical_graph(base);
    let existing: Vec<String> = payload.ids.iter().filter(|id| graph.nodes.iter().any(|node| &node.id == *id)).cloned().collect();
    if existing.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} requested node(s) exist.", payload.ids.len()), payload.ids.clone());
    }
    let missing: Vec<String> = payload.ids.iter().filter(|id| !existing.contains(id)).cloned().collect();
    let cascaded_edge_ids: Vec<String> = graph.edges.iter().filter(|edge| existing.contains(&edge.source) || existing.contains(&edge.target)).map(|edge| edge.id.clone()).collect();
    graph.nodes.retain(|node| !existing.contains(&node.id));
    graph.edges.retain(|edge| !existing.contains(&edge.source) && !existing.contains(&edge.target));
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    let mut outcome = protocol::MutationOutcome::new(MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() });
    if !missing.is_empty() {
        outcome = outcome.absorb_messages([protocol::MutationMessage::warn("mutation.partial", format!("{} of {} requested node(s) did not exist and were skipped.", missing.len(), payload.ids.len())).at(missing.clone())]);
    }
    if !cascaded_edge_ids.is_empty() {
        outcome = outcome.info("mutation.cascade", format!("Deleting {} node(s) also removed {} connected edge(s): {}.", existing.len(), cascaded_edge_ids.len(), cascaded_edge_ids.join(", ")));
    }
    outcome
}
//#endregion 🔖️Diff
