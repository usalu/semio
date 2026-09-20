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
pub async fn new_vcs_store(envelope: VcsEnvelope) -> Result<OwnedVcsStore, store::VcsError> {
    let mut store = VcsStore::new(envelope).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<VcsSnapshot, VcsDemoMutation>());
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
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("VCS document store closes through its exact bounded owners");
        }
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

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the artifact's single apply entry
/// point (mirrors dag's `apply_dag_mutation`/puzzle5d's `apply_puzzle5d_mutation`). A rejecting
/// diff carries an empty `VcsDiff`, so the snapshot is left untouched and `Ok(())` is still
/// returned; read [`protocol::MutationOutcome::messages`] to distinguish the two.
pub fn apply_vcs_mutation(snapshot: &mut VcsSnapshot, mutation: &VcsDemoMutation) -> protocol::MutationApplyResult<()> {
    use store::MutationDiff;
    let next = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(mutation, snapshot).diff().apply(snapshot)?;
    *snapshot = next;
    Ok(())
}

/// ↩️ The typed mutation steps that undo `mutation` against `snapshot`.
pub fn inverse_vcs_mutation(snapshot: &VcsSnapshot, mutation: &VcsDemoMutation) -> Vec<VcsDemoMutation> {
    <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::inverse(mutation, snapshot)
}
//#endregion 🔖️Apply

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's internally-tagged (`{"mutation": "addTag", …}`, camelCase payload
/// fields) JSON projection — exactly the shape the committed
/// `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` specification vectors carry — into a real
/// [`VcsDemoMutation`]. The `🌿️mutate-vcs-1` adapter cannot reach `serde_json` (the generated test
/// host links only `semio-repo-test-host` and this crate) and cannot name this crate's private
/// `protocol`/`store` extern-crate aliases either, so the bridge belongs here rather than there.
pub fn decode_vcs_mutation_json(text: &str) -> Result<VcsDemoMutation, String> {
    dsl::json::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ [`apply_vcs_mutation`]'s reporting, non-async twin: applies `mutation` in place and returns
/// the diagnostic CODES it raised, in order. [`apply_vcs_mutation`] discards them and is `async`,
/// so neither the outcome-policy claim a committed `🎯️outcome/🔣️.json` makes nor a
/// synchronous test adapter can be served by it.
pub fn apply_vcs_mutation_reporting(snapshot: &mut VcsSnapshot, mutation: &VcsDemoMutation) -> Vec<String> {
    let outcome = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(mutation, snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| message.code.0.clone()).collect()
}

/// ↩️ [`inverse_vcs_mutation`]'s non-async twin — the mutation's OWN computed undo steps, which is
/// what an `inverse-<kind>` scenario has to apply for the metamorphic law to mean anything.
pub fn inverse_vcs_mutation_steps(mutation: &VcsDemoMutation, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
