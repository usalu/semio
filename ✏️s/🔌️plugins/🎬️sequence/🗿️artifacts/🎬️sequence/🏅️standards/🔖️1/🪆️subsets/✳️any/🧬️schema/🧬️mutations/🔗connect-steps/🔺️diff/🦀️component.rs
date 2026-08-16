//! 🔺️ Sparse diff builder for `ConnectSteps`.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceEdge, SequenceSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectSteps, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
    let scene = sequence_working_scene(base);
    if !scene.steps.iter().any(|step| step.id == payload.from) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.from), [payload.from.clone()]);
    }
    if !scene.steps.iter().any(|step| step.id == payload.to) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.to), [payload.to.clone()]);
    }
    if scene.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if payload.from == payload.to {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Step \"{}\" cannot connect to itself.", payload.from), [payload.id.clone()]);
    }
    if scene.edges.iter().any(|edge| edge.from == payload.from && edge.to == payload.to) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" is already connected to \"{}\".", payload.from, payload.to));
    }
    let mut edges = scene.edges;
    edges.push(SequenceEdge { id: payload.id.clone(), from: payload.from.clone(), to: payload.to.clone() });
    protocol::MutationOutcome::new(diff_replace_content(scene.steps, edges))
}
//#endregion 🔖️Diff
