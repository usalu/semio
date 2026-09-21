//! ⚖️ Layout artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpText`/`protocol::OpBinary for LayoutMutation` are implemented directly in
//! `../📝️text/🦀️.rs` (`serde_json`-based, no DSL mirror needed now that every variant wraps
//! a plain local payload struct — see that file's doc comment for why the pre-migration
//! `LayoutMutationDsl`/`FramePatchDsl`/`ColorPatch` mirrors were retired). This component only adds
//! the thin artifact-facing `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law.
//!
//! The app's typed `LayoutCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it lives in `✏️editor/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::text::LayoutMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `LayoutMutation` to its binary state-patch form.
pub fn encode_op(operation: &LayoutMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `LayoutMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<LayoutMutation, protocol::ProtocolError> {
    LayoutMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type LayoutEnvelope = store::ArtifactEnvelope<crate::LayoutSnapshot, LayoutMutation>;
pub type LayoutStore = store::ArtifactStore<crate::LayoutSnapshot, LayoutMutation>;

/// 🔐️ Opens a layout store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `LayoutStore::new` can be
/// read but never mutated, undone or closed. The editor app installs the same catalog through
/// `LayoutPlayApp::build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_layout_store(envelope: LayoutEnvelope) -> Result<OwnedLayoutStore, store::VcsError> {
    let mut store = LayoutStore::new(envelope).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<crate::LayoutSnapshot, LayoutMutation>());
    Ok(OwnedLayoutStore(store))
}

/// 🔚 A standalone layout store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedLayoutStore(LayoutStore);

impl OwnedLayoutStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("layout document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedLayoutStore {
    type Target = LayoutStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedLayoutStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedLayoutStore {
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
