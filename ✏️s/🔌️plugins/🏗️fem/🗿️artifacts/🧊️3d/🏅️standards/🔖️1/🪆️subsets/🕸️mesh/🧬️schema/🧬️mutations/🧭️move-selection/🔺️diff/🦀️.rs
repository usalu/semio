//! 🔺️ Sparse diff builder for `MoveSelection`.
//!
//! Guards, in the order they run: `mutation.invariant` (Fatal) for a non-finite number, a factor that is not positive,
//! a rotation about the zero axis, or a target named twice — breaches the payload schema states as hard bounds or a
//! declared `x-semio-invariant`; `mutation.target-missing` (Error) when none of the named nodes and solids exist;
//! `mutation.no-op` (Warning) when the transform moves none of them; and `mutation.partial` (Warning), addressed at
//! the skipped ones, when some are missing or a solid cannot follow (a rotation that would tip its footprint plane).
use super::MoveSelection;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dNodesDelta, Fem3dNodesPatchEntry, Fem3dSolidsDelta, Fem3dSolidsPatchEntry};
use crate::{Fem3dSnapshot, FemNode};

//#region 🔖️Diff
pub fn diff(payload: &MoveSelection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let targets: Vec<String> = payload.node_ids.iter().chain(&payload.solid_ids).cloned().collect();
    let numbers = [payload.pivot_x, payload.pivot_y, payload.pivot_z, payload.dx, payload.dy, payload.dz, payload.axis_x, payload.axis_y, payload.axis_z, payload.angle, payload.sx, payload.sy, payload.sz];
    if numbers.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A move-selection carries a non-finite number.".to_string(), targets);
    }
    if payload.sx <= 0.0 || payload.sy <= 0.0 || payload.sz <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A move-selection needs positive scale factors, got ({}, {}, {}).", payload.sx, payload.sy, payload.sz), targets);
    }
    if payload.angle != 0.0 && payload.unit_axis().is_none() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A move-selection cannot rotate about the zero axis.".to_string(), targets);
    }
    if let Some(twice) = repeated(&payload.node_ids).or_else(|| repeated(&payload.solid_ids)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A move-selection names \"{twice}\" twice."), [twice.to_string()]);
    }
    let nodes: Vec<&FemNode> = base.nodes.iter().filter(|node| payload.node_ids.contains(&node.id)).collect();
    let solids: Vec<&crate::FemSolid> = base.solids.iter().filter(|solid| payload.solid_ids.contains(&solid.id)).collect();
    if nodes.is_empty() && solids.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} named node(s) and solid(s) exist.", targets.len()), targets);
    }
    let patched_nodes: Vec<Fem3dNodesPatchEntry> = nodes
        .iter()
        .map(|node| {
            let [x, y, z] = payload.map([node.x, node.y, node.z]);
            FemNode { x, y, z, ..(*node).clone() }
        })
        .filter(|moved| base.nodes.iter().any(|node| node.id == moved.id && node != moved))
        .map(|item| Fem3dNodesPatchEntry { id: item.id.clone(), item })
        .collect();
    let followed: Vec<(String, Option<crate::FemSolid>)> = solids.iter().map(|solid| (solid.id.clone(), payload.map_solid(solid))).collect();
    let patched_solids: Vec<Fem3dSolidsPatchEntry> = followed.iter().filter_map(|(_, mapped)| mapped.clone()).filter(|moved| base.solids.iter().any(|solid| solid.id == moved.id && solid != moved)).map(|item| Fem3dSolidsPatchEntry { id: item.id.clone(), item }).collect();
    if patched_nodes.is_empty() && patched_solids.is_empty() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The transform moves none of the {} named node(s) and solid(s).", targets.len()));
    }
    let outcome = protocol::MutationOutcome::new(Fem3dDiff {
        nodes: (!patched_nodes.is_empty()).then(|| Fem3dNodesDelta { patched: patched_nodes, ..Default::default() }),
        solids: (!patched_solids.is_empty()).then(|| Fem3dSolidsDelta { patched: patched_solids, ..Default::default() }),
        ..Default::default()
    });
    let skipped: Vec<String> = payload
        .node_ids
        .iter()
        .filter(|id| !nodes.iter().any(|node| &node.id == *id))
        .chain(payload.solid_ids.iter().filter(|id| followed.iter().all(|(solid, mapped)| solid != *id || mapped.is_none())))
        .cloned()
        .collect();
    if skipped.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warn("mutation.partial", format!("{} of {} named node(s) and solid(s) do not exist or cannot follow the transform and were skipped.", skipped.len(), targets.len())).at(skipped)])
    }
}

/// 🔁️ The first id `ids` names more than once.
fn repeated(ids: &[String]) -> Option<&str> {
    ids.iter().enumerate().find(|(at, id)| ids[..*at].contains(id)).map(|(_, id)| id.as_str())
}
//#endregion 🔖️Diff
