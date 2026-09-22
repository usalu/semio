//! ⚖️ Wires artifact — binary command protocol surface + laws (constitutional: spr, renamed from
//! protocol). The app-level `WiresCommand` binary command envelope (the old hand-derived enum this
//! module used to also host) is now REBUILT by `app_commands!` in `crate::editor::wires::component` — see
//! `crate::editor::wires::WiresCommand`'s doc there.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::WiresMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `WiresMutation` to its binary command form.
pub fn encode_op(operation: &WiresMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `WiresMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<WiresMutation, protocol::ProtocolError> {
    WiresMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type WiresEnvelope = store::ArtifactEnvelope<crate::WiresSnapshot, WiresMutation>;
pub type WiresStore = store::ArtifactStore<crate::WiresSnapshot, WiresMutation>;

/// 🔐️ Opens a Wires store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `WiresStore::new` can be
/// read but never mutated, undone or closed. The editor app installs the same catalog through
/// `WiresPlayApp::build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_wires_store(envelope: WiresEnvelope) -> Result<OwnedWiresStore, store::VcsError> {
    let mut store = WiresStore::new(envelope).await?;
    store.install_document_store_owners_exact(crate::schema::retirement::document_store_owners());
    Ok(OwnedWiresStore(store))
}

/// 🔚 A standalone Wires store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedWiresStore(WiresStore);

impl OwnedWiresStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Wires document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedWiresStore {
    type Target = WiresStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedWiresStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedWiresStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
