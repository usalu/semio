//! 🔺️ Sparse diff construction for the `delete-wayfinding-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧭wayfinding` per Wave C.

use super::DeleteWayfindingRequirement;
use crate::standards::v1::subsets::any::schema::diff::ProgramWayfindingDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteWayfindingRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.wayfinding.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No wayfinding requirement exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { wayfinding: Some(ProgramWayfindingDelta::removal(&base.wayfinding, position)), ..Default::default() })
}
