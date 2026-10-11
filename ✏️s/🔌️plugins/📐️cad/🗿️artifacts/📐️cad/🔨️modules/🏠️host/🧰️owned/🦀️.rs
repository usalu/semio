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
/// the framework's bounded default owners; every standalone store goes through here instead.
pub async fn new_cad_store(envelope: CadEnvelope, actor: protocol::ActorId) -> Result<OwnedCadStore, store::VcsError> {
    let mut store = CadStore::new(envelope, actor).await?;
    let owners = store::funded_bounded_artifact_store_owners::<CadSnapshot, CadMutation>().expect("CAD document store owner catalog is fully funded");
    if let Err((error, _owners)) = store.install_document_store_owners_exact(owners) {
        panic!("CAD document store refused its exact owner catalog: {error}");
    }
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
            let demand = self.0.close_owned_demands(0).expect("CAD document store quotes its next close turn");
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            self.0.close_owned_step(grant).expect("CAD document store closes through its exact bounded owners");
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
