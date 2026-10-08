//! 🔺️ `delete-nodes` — sparse diff construction: the removed node ids and every incident edge id (the cascade).
use crate::diff::{EquationEdgesDelta, EquationNodesDelta};
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteNodes, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let existing: Vec<String> = base.graph.nodes.iter().filter(|node| payload.ids.contains(&node.id)).map(|node| node.id.clone()).collect();
    if existing.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} requested node(s) exist.", payload.ids.len()), payload.ids.clone());
    }
    let missing: Vec<String> = payload.ids.iter().filter(|id| !existing.contains(id)).cloned().collect();
    let cascaded_edge_ids: Vec<String> = base.graph.edges.iter().filter(|edge| existing.contains(&edge.source) || existing.contains(&edge.target)).map(|edge| edge.id.clone()).collect();
    let diff = EquationDiff {
        nodes: Some(EquationNodesDelta { removed: existing.clone(), ..Default::default() }),
        edges: (!cascaded_edge_ids.is_empty()).then(|| EquationEdgesDelta { removed: cascaded_edge_ids.clone(), ..Default::default() }),
        ..Default::default()
    };
    let mut outcome = protocol::MutationOutcome::new(crate::equation_state_diff(diff, base));
    if !missing.is_empty() {
        outcome = outcome.absorb_messages([protocol::MutationMessage::warning("mutation.partial", format!("{} of {} requested node(s) did not exist and were skipped.", missing.len(), payload.ids.len())).at(missing)]);
    }
    if !cascaded_edge_ids.is_empty() {
        outcome = outcome.info("mutation.cascade", format!("Deleting {} node(s) also removed {} connected edge(s): {}.", existing.len(), cascaded_edge_ids.len(), cascaded_edge_ids.join(", ")));
    }
    outcome
}
//#endregion 🔖️Diff
