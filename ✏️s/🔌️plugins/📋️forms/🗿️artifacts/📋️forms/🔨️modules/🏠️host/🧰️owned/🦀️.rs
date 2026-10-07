//! 🏠️ Artifact document-store and publication authorities.

use crate::FormsSnapshot;
use store::PackError;
use crate::standards::v1::subsets::any::io::binary::snapshot::{FormsSnapshot};
pub type FormsStore = store::ArtifactStore<FormsSnapshot, crate::op::FormMutation>;

/// 🔐️ Opens a Forms store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `FormsStore::new` can be
/// read but never mutated, undone or closed. The editor app installs the same catalog through
/// `FormsPlayApp::build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_forms_store(envelope: FormsEnvelope, actor: protocol::ActorId) -> Result<OwnedFormsStore, store::VcsError> {
    let mut store = FormsStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<FormsSnapshot, crate::op::FormMutation>());
    Ok(OwnedFormsStore(store))
}

/// 🔚 A standalone Forms store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedFormsStore(FormsStore);

impl OwnedFormsStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Forms document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedFormsStore {
    type Target = FormsStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedFormsStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedFormsStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}

pub type FormsEnvelope = store::ArtifactEnvelope<FormsSnapshot, crate::op::FormMutation>;
