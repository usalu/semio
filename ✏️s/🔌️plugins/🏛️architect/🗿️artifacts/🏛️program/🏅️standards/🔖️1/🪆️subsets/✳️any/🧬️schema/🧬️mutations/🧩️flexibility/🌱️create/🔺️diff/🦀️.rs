//! 🔺️ Sparse diff construction for the `create-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::CreateFlexibilityRequirement;
use crate::diff::ProgramFlexibilityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateFlexibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.flexibility_requirement.header.id;
    if base.flexibility.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A flexibility requirement already exists with this id.", [id.0.clone()]);
    }
    let length = base.flexibility.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the flexibility requirement list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.flexibility.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { added: vec![payload.flexibility_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
