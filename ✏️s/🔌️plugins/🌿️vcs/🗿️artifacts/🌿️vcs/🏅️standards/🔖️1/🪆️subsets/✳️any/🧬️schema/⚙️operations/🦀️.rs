//! ⚙️ VCS mutation protocol dispatch, codec bridges, and cross-mutation store laws.

use crate::mutations::VcsDemoMutation;
use crate::VcsSnapshot;

//#region 🏷️Roster
/// 🏷️ Language-neutral catalog roster in aggregate declaration order.
pub const KINDS: &[&str] = &["rename-vcs", "change-counter", "change-notes", "change-status", "add-tag", "remove-tag"];
//#endregion 🏷️Roster

//#region 🔖️Store
pub type VcsEnvelope = store::ArtifactEnvelope<VcsSnapshot, VcsDemoMutation>;
pub type VcsStore = store::ArtifactStore<VcsSnapshot, VcsDemoMutation>;

/// 🔐️ Opens a standalone VCS store WITH its exact owner catalog installed. `ArtifactStore::new`
/// installs none, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`), so a bare `VcsStore::new` can be read
/// but never mutated, undone or closed. `VcsPlayApp::build_document_store_owners` installs the SAME
/// catalog for the app-hosted store; every standalone store goes through here instead.
pub async fn new_vcs_store(envelope: VcsEnvelope, actor: protocol::ActorId) -> Result<OwnedVcsStore, store::VcsError> {
    let mut store = VcsStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(store::funded_bounded_artifact_store_owners::<VcsSnapshot, VcsDemoMutation>().expect("funded bounded document owners")).map_err(|(error, _)| error).expect("document owners install");
    Ok(OwnedVcsStore(store))
}

/// 🔚 A standalone VCS store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so this guard runs that loop on drop (skipped while unwinding, where
/// the original panic is the report worth keeping). Derefs to the bare store for read and dispatch.
pub struct OwnedVcsStore(VcsStore);

impl OwnedVcsStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        self.0.close_owned_unscheduled().expect("VCS document store closes through its exact bounded owners");
    }
}

impl std::ops::Deref for OwnedVcsStore {
    type Target = VcsStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedVcsStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedVcsStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}
//#endregion 🔖️Store


/// ↩️ The typed mutation steps that undo `mutation` against `snapshot`.
pub fn inverse_vcs_mutation(snapshot: &VcsSnapshot, mutation: &VcsDemoMutation) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok({
    <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::inverse(mutation, snapshot)?

    })
}
//#endregion 🔖️Apply

//#region 🌉️ExternalCodecBridge



/// ↩️ [`inverse_vcs_mutation`]'s non-async twin — the mutation's OWN computed undo steps, which is
/// what an `inverse-<kind>` scenario has to apply for the metamorphic law to mean anything.
pub fn inverse_vcs_mutation_steps(mutation: &VcsDemoMutation, base: &VcsSnapshot) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok({
    <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::inverse(mutation, base)?

    })
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
