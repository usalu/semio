//! 🔺️ Sparse diff builder for `CreateRule` — a real id-keyed upsert into `rules`.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::mutation::CreateRule, _base: &AssemblySnapshot) -> AssemblyDiff {
    AssemblyDiff { rules_upserted: vec![(payload.index, payload.rule.clone())], ..Default::default() }
}
