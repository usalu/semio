//! 🔺️ Sparse diff builder for `CreateSlot` — a real id-keyed upsert into `slots` (never a
//! whole-snapshot capture).

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::mutation::CreateSlot, _base: &AssemblySnapshot) -> AssemblyDiff {
    AssemblyDiff { slots_upserted: vec![(payload.index, payload.slot.clone())], ..Default::default() }
}
