//! 🔺️ Sparse diff builder for `DeleteRule` — removes the id from `rules`.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub async fn diff(payload: &super::mutation::DeleteRule, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    if !base.rules.iter().any(|rule| rule.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rule \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(AssemblyDiff { rules_removed: vec![payload.id.clone()], ..Default::default() })
}
