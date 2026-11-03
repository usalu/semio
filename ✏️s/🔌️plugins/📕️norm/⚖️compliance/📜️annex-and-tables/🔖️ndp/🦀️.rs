//! 🔖️ Registry of nationally determined parameters (NDP): provenance and verification status of every code value a check may use. Lookup facade over the per-Eurocode table crates, a leaf crate of the norm compliance core, re-exported by `semio-s-plugin-norm-compliance`; data in `🔣️.json`, schema in `../../🧬️schema/🔣️.json`.

#[path = "🗃️table.rs"]
mod table;
pub use semio_s_plugin_norm_ndp_model::{NdpEntry, NdpSource, NdpStatus, NdpValue};
pub use table::NDP_TABLE;

/// 🔍️ Binary-search lookup in the static, id-sorted table; no allocation.
pub fn ndp(id: &str) -> Option<&'static NdpEntry> {
    NDP_TABLE.binary_search_by(|entry| entry.id.cmp(id)).ok().map(|index| &NDP_TABLE[index])
}

/// ✅️ Unknown ids count as unverified so a typo can never yield a compliant verdict.
pub fn ndp_is_verified(id: &str) -> bool {
    ndp(id).is_some_and(|entry| entry.status == NdpStatus::Verified)
}

/// 📋️ Every entry still awaiting verification.
pub fn unverified_ndp_entries() -> impl Iterator<Item = &'static NdpEntry> {
    NDP_TABLE.iter().filter(|entry| entry.status == NdpStatus::Unverified)
}
