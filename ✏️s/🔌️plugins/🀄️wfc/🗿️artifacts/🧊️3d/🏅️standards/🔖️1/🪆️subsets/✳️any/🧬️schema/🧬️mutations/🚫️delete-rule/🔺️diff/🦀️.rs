//! 🔺️ Sparse diff builder for `DeleteRule` — removes the id from `rules`.

use crate::diff::{Wfc3dDiff, Wfc3dRows};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::DeleteRule, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if !base.rules.iter().any(|rule| rule.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rule \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc3dDiff { rules: Wfc3dRows { removed: vec![payload.id.clone()], ..Default::default() }, ..Default::default() })
}
