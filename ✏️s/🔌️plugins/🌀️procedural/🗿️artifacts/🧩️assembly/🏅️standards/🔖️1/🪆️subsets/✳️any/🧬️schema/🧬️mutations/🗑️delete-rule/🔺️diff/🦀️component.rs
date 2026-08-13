//! 🔺️ Sparse diff builder for `DeleteRule` — removes the id from `rules`.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::mutation::DeleteRule, _base: &AssemblySnapshot) -> AssemblyDiff {
    AssemblyDiff { rules_removed: vec![payload.id.clone()], ..Default::default() }
}
