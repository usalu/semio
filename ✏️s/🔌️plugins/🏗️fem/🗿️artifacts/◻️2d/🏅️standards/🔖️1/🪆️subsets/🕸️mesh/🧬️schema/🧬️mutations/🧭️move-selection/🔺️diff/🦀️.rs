//! 🔺️ Sparse diff builder for `MoveSelection`.
//!
//! Guards, in the order they run: `mutation.invariant` (Fatal) for a non-finite number, a factor that is not
//! positive, or a target named twice — breaches the payload schema states as hard bounds; `mutation.target-missing`
//! (Error) when none of the named nodes and regions exist; `mutation.no-op` (Warning) when the transform moves none
//! of them; and `mutation.partial` (Warning), addressed at the absent ones, when some are missing.
use super::MoveSelection;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dRegionPatch;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodePatch;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodesDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodesModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dRegionsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dRegionsModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use std::collections::HashSet;

//#region 🔖️Diff
pub fn diff(payload: &MoveSelection, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let targets: Vec<String> = payload.node_ids.iter().chain(&payload.region_ids).cloned().collect();
    if let Some((message, address)) = payload.breach() {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, address);
    }
    let node_ids: HashSet<_> = payload.node_ids.iter().map(String::as_str).collect();
    let region_ids: HashSet<_> = payload.region_ids.iter().map(String::as_str).collect();
    let nodes: Vec<_> = base.nodes.iter().filter(|node| node_ids.contains(node.id.as_str())).collect();
    let regions: Vec<_> = base.regions.iter().filter(|region| region_ids.contains(region.id.as_str())).collect();
    if nodes.is_empty() && regions.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} named node(s) and region(s) exist.", targets.len()), targets);
    }
    let mut patched_nodes = Vec::new();
    for node in &nodes {
        let Some(item) = payload.moved_node(node) else { continue };
        if let Some(refusal) = guards::node_geometry(&item) { return refusal; }
        patched_nodes.push(Fem2dNodesModification { id: item.id, patch: Fem2dNodePatch { x: (item.x != node.x).then_some(item.x), y: (item.y != node.y).then_some(item.y) } });
    }
    let mut patched_regions = Vec::new();
    for region in &regions {
        let Some(item) = payload.moved_region(region) else { continue };
        if let Some(refusal) = guards::region_geometry(&item) { return refusal; }
        patched_regions.push(Fem2dRegionsModification { id: item.id, patch: Fem2dRegionPatch { outline: (item.outline != region.outline).then_some(item.outline), holes: (item.holes != region.holes).then_some(item.holes), ..Default::default() } });
    }
    if patched_nodes.is_empty() && patched_regions.is_empty() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The transform moves none of the {} named node(s) and region(s).", targets.len()));
    }
    let outcome = protocol::MutationOutcome::new(Fem2dDiff {
        nodes: (!patched_nodes.is_empty()).then(|| Fem2dNodesDelta { modified: patched_nodes, ..Default::default() }),
        regions: (!patched_regions.is_empty()).then(|| Fem2dRegionsDelta { modified: patched_regions, ..Default::default() }),
        ..Default::default()
    });
    let found_nodes: HashSet<_> = nodes.iter().map(|node| node.id.as_str()).collect();
    let found_regions: HashSet<_> = regions.iter().map(|region| region.id.as_str()).collect();
    let missing: Vec<String> = payload.node_ids.iter().filter(|id| !found_nodes.contains(id.as_str())).chain(payload.region_ids.iter().filter(|id| !found_regions.contains(id.as_str()))).cloned().collect();
    if missing.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warning("mutation.partial", format!("{} of {} named node(s) and region(s) do not exist and were skipped.", missing.len(), targets.len())).at(missing)])
    }
}
//#endregion 🔖️Diff
