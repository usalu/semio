//! 🔺️ Sparse diff builder for `RemoveWeight` — removes the id from `weights`.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::mutation::RemoveWeight, _base: &AssemblySnapshot) -> AssemblyDiff {
    AssemblyDiff { weights_removed: vec![payload.module_id.clone()], ..Default::default() }
}
