//! 🔺️ Sparse diff builder for `MoveSelection`.
//!
//! Guards, in the order they run: `mutation.invariant` (Fatal) for a non-finite number, a factor that is not
//! positive, or a target named twice — breaches the payload schema states as hard bounds; `mutation.target-missing`
//! (Error) when none of the named nodes and regions exist; `mutation.no-op` (Warning) when the transform moves none
//! of them; and `mutation.partial` (Warning), addressed at the absent ones, when some are missing.
use super::MoveSelection;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dNodesDelta, Fem2dNodesModification, Fem2dRegionsDelta, Fem2dRegionsModification};
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveSelection, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let targets: Vec<String> = payload.node_ids.iter().chain(&payload.region_ids).cloned().collect();
    if let Some((message, address)) = payload.breach() {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, address);
    }
    let nodes: Vec<_> = base.nodes.iter().filter(|node| payload.node_ids.contains(&node.id)).collect();
    let regions: Vec<_> = base.regions.iter().filter(|region| payload.region_ids.contains(&region.id)).collect();
    if nodes.is_empty() && regions.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} named node(s) and region(s) exist.", targets.len()), targets);
    }
    let patched_nodes: Vec<Fem2dNodesModification> = nodes.iter().filter_map(|node| payload.moved_node(node)).map(|item| Fem2dNodesModification { id: item.id.clone(), patch: item }).collect();
    let patched_regions: Vec<Fem2dRegionsModification> = regions.iter().filter_map(|region| payload.moved_region(region)).map(|item| Fem2dRegionsModification { id: item.id.clone(), patch: item }).collect();
    if patched_nodes.is_empty() && patched_regions.is_empty() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The transform moves none of the {} named node(s) and region(s).", targets.len()));
    }
    let outcome = protocol::MutationOutcome::new(Fem2dDiff {
        nodes: (!patched_nodes.is_empty()).then(|| Fem2dNodesDelta { modified: patched_nodes, ..Default::default() }),
        regions: (!patched_regions.is_empty()).then(|| Fem2dRegionsDelta { modified: patched_regions, ..Default::default() }),
        ..Default::default()
    });
    let missing: Vec<String> = payload.node_ids.iter().filter(|id| !nodes.iter().any(|node| &node.id == *id)).chain(payload.region_ids.iter().filter(|id| !regions.iter().any(|region| &region.id == *id))).cloned().collect();
    if missing.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warning("mutation.partial", format!("{} of {} named node(s) and region(s) do not exist and were skipped.", missing.len(), targets.len())).at(missing)])
    }
}
//#endregion 🔖️Diff
