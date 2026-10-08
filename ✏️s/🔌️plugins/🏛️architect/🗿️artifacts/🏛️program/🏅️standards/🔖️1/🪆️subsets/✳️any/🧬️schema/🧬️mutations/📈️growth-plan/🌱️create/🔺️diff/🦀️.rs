//! 🔺️ Sparse diff construction for the `create-growth-plan` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📈growth` per Wave C.

use super::CreateGrowthPlan;
use crate::diff::ProgramGrowthDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateGrowthPlan, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.growth_plan.header.id;
    if base.growth.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A growth plan already exists with this id.", [id.0.clone()]);
    }
    let length = base.growth.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the growth plan list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.growth.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { growth: Some(ProgramGrowthDelta { added: vec![payload.growth_plan.clone()], reordered, ..Default::default() }), ..Default::default() })
}
