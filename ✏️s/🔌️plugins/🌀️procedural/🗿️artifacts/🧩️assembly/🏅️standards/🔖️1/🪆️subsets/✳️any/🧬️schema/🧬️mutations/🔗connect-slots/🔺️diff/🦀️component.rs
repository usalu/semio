//! 🔺️ Sparse diff builder for `ConnectSlots` — a real id-keyed upsert into `edges`.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::mutation::ConnectSlots, _base: &AssemblySnapshot) -> AssemblyDiff {
    AssemblyDiff { edges_upserted: vec![(payload.index, payload.edge.clone())], ..Default::default() }
}
