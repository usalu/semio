//! 🔺️ Sparse diff builder for `MoveSelection`.
//!
//! Guards, in the order they run: `mutation.invariant` (Fatal) for a non-finite number, a factor that is not
//! positive, or a target named twice — breaches the payload schema states as hard bounds; `mutation.target-missing`
//! (Error) when none of the named nodes and regions exist; `mutation.no-op` (Warning) when the transform moves none
//! of them; and `mutation.partial` (Warning), addressed at the absent ones, when some are missing.
use super::MoveSelection;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dNodesDelta, Fem2dNodesPatchEntry, Fem2dRegionsDelta, Fem2dRegionsPatchEntry};
use crate::{Fem2dSnapshot, FemNode, FemRegion};

//#region 🔖️Diff
pub fn diff(payload: &MoveSelection, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let targets: Vec<String> = payload.node_ids.iter().chain(&payload.region_ids).cloned().collect();
    if [payload.pivot_x, payload.pivot_y, payload.dx, payload.dy, payload.angle, payload.sx, payload.sy].iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A move-selection carries a non-finite number.".to_string(), targets);
    }
    if payload.sx <= 0.0 || payload.sy <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A move-selection needs positive scale factors, got ({}, {}).", payload.sx, payload.sy), targets);
    }
    if let Some(twice) = repeated(&payload.node_ids).or_else(|| repeated(&payload.region_ids)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A move-selection names \"{twice}\" twice."), [twice.to_string()]);
    }
    let nodes: Vec<&FemNode> = base.nodes.iter().filter(|node| payload.node_ids.contains(&node.id)).collect();
    let regions: Vec<&FemRegion> = base.regions.iter().filter(|region| payload.region_ids.contains(&region.id)).collect();
    if nodes.is_empty() && regions.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} named node(s) and region(s) exist.", targets.len()), targets);
    }
    let point = |point: &[f64; 2]| {
        let (x, y) = payload.map(point[0], point[1]);
        [x, y]
    };
    let patched_nodes: Vec<Fem2dNodesPatchEntry> = nodes
        .iter()
        .map(|node| {
            let (x, y) = payload.map(node.x, node.y);
            FemNode { x, y, ..(*node).clone() }
        })
        .filter(|moved| base.nodes.iter().any(|node| node.id == moved.id && node != moved))
        .map(|item| Fem2dNodesPatchEntry { id: item.id.clone(), item })
        .collect();
    let patched_regions: Vec<Fem2dRegionsPatchEntry> = regions
        .iter()
        .map(|region| FemRegion { outline: region.outline.iter().map(point).collect(), holes: region.holes.iter().map(|hole| hole.iter().map(point).collect()).collect(), ..(*region).clone() })
        .filter(|moved| base.regions.iter().any(|region| region.id == moved.id && region != moved))
        .map(|item| Fem2dRegionsPatchEntry { id: item.id.clone(), item })
        .collect();
    if patched_nodes.is_empty() && patched_regions.is_empty() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The transform moves none of the {} named node(s) and region(s).", targets.len()));
    }
    let outcome = protocol::MutationOutcome::new(Fem2dDiff {
        nodes: (!patched_nodes.is_empty()).then(|| Fem2dNodesDelta { patched: patched_nodes, ..Default::default() }),
        regions: (!patched_regions.is_empty()).then(|| Fem2dRegionsDelta { patched: patched_regions, ..Default::default() }),
        ..Default::default()
    });
    let missing: Vec<String> = payload.node_ids.iter().filter(|id| !nodes.iter().any(|node| &node.id == *id)).chain(payload.region_ids.iter().filter(|id| !regions.iter().any(|region| &region.id == *id))).cloned().collect();
    if missing.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warn("mutation.partial", format!("{} of {} named node(s) and region(s) do not exist and were skipped.", missing.len(), targets.len())).at(missing)])
    }
}

/// 🔁️ The first id `ids` names more than once.
fn repeated(ids: &[String]) -> Option<&str> {
    ids.iter().enumerate().find(|(at, id)| ids[..*at].contains(id)).map(|(_, id)| id.as_str())
}
//#endregion 🔖️Diff
