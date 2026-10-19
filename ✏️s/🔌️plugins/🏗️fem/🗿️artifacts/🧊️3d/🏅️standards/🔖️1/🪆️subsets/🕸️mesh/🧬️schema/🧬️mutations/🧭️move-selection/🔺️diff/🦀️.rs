//! 🔺️ Sparse diff builder for `MoveSelection`.
//!
//! Guards, in the order they run: `mutation.invariant` (Fatal) for a non-finite number, a factor that is not
//! positive, a rotation about the zero axis, or a target named twice — breaches the payload schema states as hard
//! bounds; `mutation.target-missing` (Error) when none of the named nodes and solids exist; `mutation.no-op`
//! (Warning) when the transform moves none of them; and `mutation.partial` (Warning), addressed at the ones that do
//! not exist or cannot follow the transform, when some are skipped.
use super::MoveSelection;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSolidPatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dNodePatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dNodesDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dNodesModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSolidsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSolidsModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::node_breach;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::solid_breach;
use std::collections::HashSet;

//#region 🔖️Diff
pub fn diff(payload: &MoveSelection, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let targets: Vec<String> = payload.node_ids.iter().chain(&payload.solid_ids).cloned().collect();
    if let Some((message, address)) = payload.breach() {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, address);
    }
    let node_ids: HashSet<_> = payload.node_ids.iter().map(String::as_str).collect();
    let solid_ids: HashSet<_> = payload.solid_ids.iter().map(String::as_str).collect();
    let nodes: Vec<_> = base.nodes.iter().filter(|node| node_ids.contains(node.id.as_str())).collect();
    let solids: Vec<_> = base.solids.iter().filter(|solid| solid_ids.contains(solid.id.as_str())).collect();
    if nodes.is_empty() && solids.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} named node(s) and solid(s) exist.", targets.len()), targets);
    }
    let moved_nodes: Vec<_> = nodes.iter().filter_map(|node| payload.moved_node(node).map(|moved| (*node, moved))).collect();
    let followed: Vec<_> = solids.iter().map(|solid| (*solid, payload.map_solid(solid))).collect();
    for (node, moved) in &moved_nodes {
        if let Some(message) = node_breach(moved) { return protocol::MutationOutcome::fatal("mutation.invariant", message, [node.id.clone()]); }
    }
    for (solid, moved) in &followed {
        if let Some(message) = moved.as_ref().and_then(solid_breach) { return protocol::MutationOutcome::fatal("mutation.invariant", message, [solid.id.clone()]); }
    }
    let patched_nodes: Vec<Fem3dNodesModification> = moved_nodes.into_iter().map(|(base, moved)| Fem3dNodesModification {
        id: base.id.clone(),
        patch: Fem3dNodePatch { x: (moved.x != base.x).then_some(moved.x), y: (moved.y != base.y).then_some(moved.y), z: (moved.z != base.z).then_some(moved.z) },
    }).collect();
    let patched_solids: Vec<Fem3dSolidsModification> = followed.iter().filter_map(|(base, moved)| moved.as_ref().filter(|moved| *moved != *base).map(|moved| Fem3dSolidsModification {
        id: base.id.clone(),
        patch: Fem3dSolidPatch {
            outline: (moved.outline != base.outline).then(|| moved.outline.clone()),
            holes: (moved.holes != base.holes).then(|| moved.holes.clone()),
            base_z: (moved.base_z != base.base_z).then_some(moved.base_z),
            height: (moved.height != base.height).then_some(moved.height),
            ..Default::default()
        },
    })).collect();
    if patched_nodes.is_empty() && patched_solids.is_empty() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The transform moves none of the {} named node(s) and solid(s).", targets.len()));
    }
    let outcome = protocol::MutationOutcome::new(Fem3dDiff {
        nodes: (!patched_nodes.is_empty()).then(|| Fem3dNodesDelta { modified: patched_nodes, ..Default::default() }),
        solids: (!patched_solids.is_empty()).then(|| Fem3dSolidsDelta { modified: patched_solids, ..Default::default() }),
        ..Default::default()
    });
    let found_nodes: HashSet<_> = nodes.iter().map(|node| node.id.as_str()).collect();
    let followed_solids: HashSet<_> = followed.iter().filter(|(_, mapped)| mapped.is_some()).map(|(solid, _)| solid.id.as_str()).collect();
    let skipped: Vec<String> = payload
        .node_ids
        .iter()
        .filter(|id| !found_nodes.contains(id.as_str()))
        .chain(payload.solid_ids.iter().filter(|id| !followed_solids.contains(id.as_str())))
        .cloned()
        .collect();
    if skipped.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warning("mutation.partial", format!("{} of {} named node(s) and solid(s) do not exist or cannot follow the transform and were skipped.", skipped.len(), targets.len())).at(skipped)])
    }
}
//#endregion 🔖️Diff
