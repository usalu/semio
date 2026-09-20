//! 📡️ CAD artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `CadMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! cad host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

use crate::op::CadMutation;
use crate::CadSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `CadMutation` to its binary command form.
pub fn encode_op(operation: &CadMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `CadMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<CadMutation, protocol::ProtocolError> {
    CadMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type CadEnvelope = ArtifactEnvelope<CadSnapshot, CadMutation>;
pub type CadStore = ArtifactStore<CadSnapshot, CadMutation>;

/// 🔐️ Opens a CAD store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `CadStore::new` can be
/// read but never mutated, undone or closed. The editor app installs the same catalog through
/// `CadPlayApp::build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_cad_store(envelope: CadEnvelope) -> Result<OwnedCadStore, store::VcsError> {
    let mut store = CadStore::new(envelope).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<CadSnapshot, CadMutation>());
    Ok(OwnedCadStore(store))
}

/// 🔚 A standalone CAD store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedCadStore(CadStore);

impl OwnedCadStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("CAD document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedCadStore {
    type Target = CadStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedCadStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedCadStore {
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
