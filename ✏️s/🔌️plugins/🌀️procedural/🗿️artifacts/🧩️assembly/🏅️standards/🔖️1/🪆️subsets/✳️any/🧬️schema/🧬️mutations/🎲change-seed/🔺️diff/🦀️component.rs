//! 🔺️ Sparse diff builder for `ChangeSeed` — a single-field scalar delta.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::mutation::ChangeSeed, _base: &AssemblySnapshot) -> AssemblyDiff {
    AssemblyDiff { seed: Some(payload.seed), ..Default::default() }
}
