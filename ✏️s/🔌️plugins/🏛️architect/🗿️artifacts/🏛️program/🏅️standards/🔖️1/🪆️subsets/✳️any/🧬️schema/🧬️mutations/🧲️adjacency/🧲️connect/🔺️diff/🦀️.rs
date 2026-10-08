//! 🔺️ Sparse diff construction for the `connect-adjacency` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗺️set-adjacency` per Wave C.

use super::ConnectAdjacency;
use crate::diff::ProgramAdjacenciesDelta;
use crate::standards::v1::subsets::any::schema::normalize_pair;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔌️ Error `mutation.target-missing` if either endpoint element is absent (empty diff); Warning
/// `mutation.no-op` if the edge already carries this exact value (empty diff); else `added = [normalized edge]` if the pair is new,
/// else the edge is replaced under its own id: `removed = [id]`, `added = [value]`, and `reordered` (the base order) unless the edge was last.
pub fn diff(payload: &ConnectAdjacency, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let (a, b) = normalize_pair(&payload.adjacency.element_a_id, &payload.adjacency.element_b_id);
    if !base.elements.iter().any(|row| row.header.id == a) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No program element exists with this id.", [a.0]);
    }
    if !base.elements.iter().any(|row| row.header.id == b) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No program element exists with this id.", [b.0]);
    }
    let mut value = payload.adjacency.clone();
    value.element_a_id = a.clone();
    value.element_b_id = b.clone();
    value.normalized = true;
    let Some(position) = base.adjacencies.iter().position(|row| row.element_a_id == a && row.element_b_id == b) else {
        return protocol::MutationOutcome::new(ProgramDiff { adjacencies: Some(ProgramAdjacenciesDelta { added: vec![value], ..Default::default() }), ..Default::default() });
    };
    let id = base.adjacencies[position].header.id.clone();
    value.header.id = id.clone();
    if base.adjacencies[position] == value {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This adjacency already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.adjacencies.len()).then(|| base.adjacencies.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { adjacencies: Some(ProgramAdjacenciesDelta { removed: vec![id.0.clone()], added: vec![value], reordered, ..Default::default() }), ..Default::default() })
}
