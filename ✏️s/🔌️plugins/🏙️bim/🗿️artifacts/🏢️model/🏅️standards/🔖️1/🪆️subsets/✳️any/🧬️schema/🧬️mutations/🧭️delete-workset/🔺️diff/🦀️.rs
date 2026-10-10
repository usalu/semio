//! 🔺️ Sparse declarative delete-workset diff.
use super::DeleteWorkset;
use crate::*;
use super::super::{elements, option_rules};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &DeleteWorkset, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.worksets.contains_key(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Record is missing.", [payload.id.clone()]); }
    let memberships = base.element_worksets.iter().filter(|(_, membership)| membership.target == payload.id).map(|(id, _)| (id.clone(), Entry::Deleted)).collect::<std::collections::BTreeMap<_, _>>();
    let mut diff = ModelDiff::worksets(payload.id.clone(), Entry::Deleted);
    if !memberships.is_empty() { diff.element_worksets = Some(KeyedDelta(memberships)); }
    return MutationOutcome::new(diff);
}
