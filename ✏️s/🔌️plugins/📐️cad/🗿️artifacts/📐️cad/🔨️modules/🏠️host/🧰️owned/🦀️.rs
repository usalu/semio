//! 🏠️ Artifact document-store and publication authorities.

use crate::op::CadMutation;
use crate::CadSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};
pub type CadStore = ArtifactStore<CadSnapshot, CadMutation>;

/// 🔐️ Opens a CAD store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `CadStore::new` can be
/// read but never mutated, undone or closed. The editor app installs the same catalog through
/// `CadPlayApp::build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_cad_store(envelope: CadEnvelope, actor: protocol::ActorId) -> Result<OwnedCadStore, store::VcsError> {
    let mut store = CadStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(crate::editor::cad::cad_document_store_owners());
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

pub type CadEnvelope = ArtifactEnvelope<CadSnapshot, CadMutation>;
