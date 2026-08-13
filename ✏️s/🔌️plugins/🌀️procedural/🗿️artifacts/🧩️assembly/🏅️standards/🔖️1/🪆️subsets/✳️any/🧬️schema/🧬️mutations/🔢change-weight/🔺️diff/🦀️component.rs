//! 🔺️ Sparse diff builder for `ChangeWeight` — upserts the id-keyed `weights` entry.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::{AssemblyModuleWeight, AssemblySnapshot};

pub fn diff(payload: &super::mutation::ChangeWeight, _base: &AssemblySnapshot) -> AssemblyDiff {
    AssemblyDiff { weights_upserted: vec![AssemblyModuleWeight { module_id: payload.module_id.clone(), weight: payload.weight }], ..Default::default() }
}
