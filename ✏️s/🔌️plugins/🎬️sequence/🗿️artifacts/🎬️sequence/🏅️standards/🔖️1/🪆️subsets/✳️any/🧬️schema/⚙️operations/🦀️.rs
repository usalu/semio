//! ⚙️ Sequence store bridges and the parent projection (design §20.15: the parent owns no content leaf).

use crate::schema::mutations::SequenceMutation;
use crate::SequenceSnapshot;

//#region 🔖️Store
pub type SequenceEnvelope = store::ArtifactEnvelope<SequenceSnapshot, SequenceMutation>;
pub type SequenceStore = store::ArtifactStore<SequenceSnapshot, SequenceMutation>;

/// 🔐️ Opens a standalone sequence store WITH its exact owner catalog installed. `ArtifactStore::new`
/// installs none, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`), so a bare `SequenceStore::new` can be
/// read but never mutated, undone or closed. `SequencePlayApp::build_document_store_owners` installs
/// the SAME catalog for the app-hosted store; every standalone store goes through here instead.
pub async fn new_sequence_store(envelope: SequenceEnvelope) -> Result<OwnedSequenceStore, store::VcsError> {
    let mut store = SequenceStore::new(envelope).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<SequenceSnapshot, SequenceMutation>());
    Ok(OwnedSequenceStore(store))
}

/// 🔚 A standalone sequence store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so this guard runs that loop on drop (skipped while unwinding, where
/// the original panic is the report worth keeping). Derefs to the bare store for read and dispatch.
pub struct OwnedSequenceStore(SequenceStore);

impl OwnedSequenceStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("sequence document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedSequenceStore {
    type Target = SequenceStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedSequenceStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedSequenceStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}
//#endregion 🔖️Store

//#region 🔖️CaseBridges
/// ⚖️ The SEMANTIC PROJECTION a parent document is compared through: its own fields (`schema` and the composed `content`
/// handle). The content lives in the child's store and is compared there, never read off the parent.
pub fn encode_sequence_projection_json(snapshot: &SequenceSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}
//#endregion 🔖️CaseBridges
