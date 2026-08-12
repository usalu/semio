//! 🔺️ Sparse diff construction for the `connect-adjacency` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗺️set-adjacency` per Wave C.

use super::mutation::ConnectAdjacency;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAdjacenciesDelta, ProgramAdjacenciesPatchEntry};
use crate::artifacts::program::engine::adjacency::normalize_pair;

/// 🔌️ `added = [normalized edge]` if the pair is new, else `patched = [{existing id, full patch}]`
/// — the existing edge's own id is preserved even if `payload.adjacency` carries a different one.
pub fn diff(payload: &ConnectAdjacency, base: &ProgramSnapshot) -> ProgramDiff {
    let (a, b) = normalize_pair(&payload.adjacency.element_a_id, &payload.adjacency.element_b_id);
    let mut value = payload.adjacency.clone();
    value.element_a_id = a.clone();
    value.element_b_id = b.clone();
    value.normalized = true;
    match base.adjacencies.iter().find(|row| row.element_a_id == a && row.element_b_id == b) {
        Some(existing) => {
            value.header.id = existing.header.id.clone();
            let patch = existing.diff_patch(&value).expect("diff_patch always produces a full patch");
            ProgramDiff { adjacencies: Some(ProgramAdjacenciesDelta { patched: vec![ProgramAdjacenciesPatchEntry { id: existing.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
        }
        None => ProgramDiff { adjacencies: Some(ProgramAdjacenciesDelta { added: vec![value], ..Default::default() }), ..Default::default() },
    }
}
