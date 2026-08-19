//! 🔺️ Sparse diff construction for the `connect-adjacency` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗺️set-adjacency` per Wave C.

use super::mutation::ConnectAdjacency;
use crate::artifacts::program::diff::{ProgramAdjacenciesDelta, ProgramAdjacenciesPatchEntry};
use crate::artifacts::program::standards::v1::subsets::any::schema::normalize_pair;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔌️ Error `mutation.target-missing` if either endpoint element is absent (empty diff); Warning
/// `mutation.no-op` if the edge already carries this exact value (empty diff); else
/// `added = [normalized edge]` if the pair is new, else `patched = [{existing id, full patch}]`
/// — the existing edge's own id is preserved even if `payload.adjacency` carries a different one.
pub async fn diff(payload: &ConnectAdjacency, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let (a, b) = normalize_pair(&payload.adjacency.element_a_id, &payload.adjacency.element_b_id);
    if !base.elements.iter().any(|row| row.header.id == a) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No program element exists with this id.", [a.0.clone()]);
    }
    if !base.elements.iter().any(|row| row.header.id == b) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No program element exists with this id.", [b.0.clone()]);
    }
    let mut value = payload.adjacency.clone();
    value.element_a_id = a.clone();
    value.element_b_id = b.clone();
    value.normalized = true;
    match base.adjacencies.iter().find(|row| row.element_a_id == a && row.element_b_id == b) {
        Some(existing) => {
            value.header.id = existing.header.id.clone();
            if existing == &value {
                return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This adjacency already matches the requested value.").at([existing.header.id.0.clone()])]);
            }
            let patch = existing.diff_patch(&value).expect("diff_patch always produces a full patch");
            protocol::MutationOutcome::new(ProgramDiff { adjacencies: Some(ProgramAdjacenciesDelta { patched: vec![ProgramAdjacenciesPatchEntry { id: existing.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
        }
        None => protocol::MutationOutcome::new(ProgramDiff { adjacencies: Some(ProgramAdjacenciesDelta { added: vec![value], ..Default::default() }), ..Default::default() }),
    }
}
