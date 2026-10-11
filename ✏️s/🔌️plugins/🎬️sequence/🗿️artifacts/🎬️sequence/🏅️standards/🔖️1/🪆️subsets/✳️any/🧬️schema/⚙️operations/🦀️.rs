//! ⚙️ Sequence store bridges and the parent projection (design §20.15: the parent owns no content leaf).

use crate::schema::mutations::SequenceMutation;
use crate::SequenceSnapshot;

//#region 🔖️Store
pub type SequenceEnvelope = store::ArtifactEnvelope<SequenceSnapshot, SequenceMutation>;
pub type SequenceStore = store::ArtifactStore<SequenceSnapshot, SequenceMutation>;

/// 🔐️ Opens a standalone sequence store WITH its exact owner catalog installed. `ArtifactStore::new`
/// installs none, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`), so a bare `SequenceStore::new` can be
/// read but never mutated, undone or closed. the framework's bounded default owners install
/// the SAME catalog for the app-hosted store; every standalone store goes through here instead.
pub async fn new_sequence_store(envelope: SequenceEnvelope, actor: protocol::ActorId) -> Result<OwnedSequenceStore, store::VcsError> {
    let mut store = SequenceStore::new(envelope, actor).await?;
    let owners = store::funded_bounded_artifact_store_owners::<SequenceSnapshot, SequenceMutation>().expect("sequence document store owner catalog is fully funded");
    if let Err((error, _owners)) = store.install_document_store_owners_exact(owners) {
        panic!("sequence document store refused its exact owner catalog: {error}");
    }
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
            let demand = self.0.close_owned_demands(0).expect("sequence document store quotes its next close turn");
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            self.0.close_owned_step(grant).expect("sequence document store closes through its exact bounded owners");
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

//#endregion 🔖️CaseBridges
