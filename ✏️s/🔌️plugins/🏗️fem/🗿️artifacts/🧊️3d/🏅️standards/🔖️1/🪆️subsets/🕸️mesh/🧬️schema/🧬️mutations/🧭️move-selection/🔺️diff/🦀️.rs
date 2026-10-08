//! 🔺️ Sparse diff builder for `MoveSelection`.
//!
//! Guards, in the order they run: `mutation.invariant` (Fatal) for a non-finite number, a factor that is not
//! positive, a rotation about the zero axis, or a target named twice — breaches the payload schema states as hard
//! bounds; `mutation.target-missing` (Error) when none of the named nodes and solids exist; `mutation.no-op`
//! (Warning) when the transform moves none of them; and `mutation.partial` (Warning), addressed at the ones that do
//! not exist or cannot follow the transform, when some are skipped.
use super::MoveSelection;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dNodesDelta, Fem3dNodesModification, Fem3dSolidsDelta, Fem3dSolidsModification};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveSelection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let targets: Vec<String> = payload.node_ids.iter().chain(&payload.solid_ids).cloned().collect();
    if let Some((message, address)) = payload.breach() {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, address);
    }
    let nodes: Vec<_> = base.nodes.iter().filter(|node| payload.node_ids.contains(&node.id)).collect();
    let solids: Vec<_> = base.solids.iter().filter(|solid| payload.solid_ids.contains(&solid.id)).collect();
    if nodes.is_empty() && solids.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} named node(s) and solid(s) exist.", targets.len()), targets);
    }
    let patched_nodes: Vec<Fem3dNodesModification> = nodes.iter().filter_map(|node| payload.moved_node(node)).map(|item| Fem3dNodesModification { id: item.id.clone(), patch: item }).collect();
    let followed: Vec<(&str, Option<crate::FemSolid>)> = solids.iter().map(|solid| (solid.id.as_str(), payload.map_solid(solid))).collect();
    let patched_solids: Vec<Fem3dSolidsModification> = solids.iter().filter_map(|solid| payload.moved_solid(solid)).map(|item| Fem3dSolidsModification { id: item.id.clone(), patch: item }).collect();
    if patched_nodes.is_empty() && patched_solids.is_empty() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The transform moves none of the {} named node(s) and solid(s).", targets.len()));
    }
    let outcome = protocol::MutationOutcome::new(Fem3dDiff {
        nodes: (!patched_nodes.is_empty()).then(|| Fem3dNodesDelta { modified: patched_nodes, ..Default::default() }),
        solids: (!patched_solids.is_empty()).then(|| Fem3dSolidsDelta { modified: patched_solids, ..Default::default() }),
        ..Default::default()
    });
    let skipped: Vec<String> = payload
        .node_ids
        .iter()
        .filter(|id| !nodes.iter().any(|node| &node.id == *id))
        .chain(payload.solid_ids.iter().filter(|id| followed.iter().all(|(solid, mapped)| solid != &id.as_str() || mapped.is_none())))
        .cloned()
        .collect();
    if skipped.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warning("mutation.partial", format!("{} of {} named node(s) and solid(s) do not exist or cannot follow the transform and were skipped.", skipped.len(), targets.len())).at(skipped)])
    }
}
//#endregion 🔖️Diff
