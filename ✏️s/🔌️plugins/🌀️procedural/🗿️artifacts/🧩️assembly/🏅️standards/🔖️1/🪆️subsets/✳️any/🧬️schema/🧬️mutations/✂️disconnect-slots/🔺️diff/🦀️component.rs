//! 🔺️ Sparse diff builder for `DisconnectSlots` — removes the id from `edges`.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::mutation::DisconnectSlots, _base: &AssemblySnapshot) -> AssemblyDiff {
    AssemblyDiff { edges_removed: vec![payload.id.clone()], ..Default::default() }
}
