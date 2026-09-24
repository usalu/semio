//! 📦️ Forms artifact — binary document surface + laws (constitutional: pack). `store::ArtifactPack for
//! FormsSnapshot` encodes the derived `dsl::DslRecord` spec (composed `structure`/`results` child handles
//! included) and validates the document marker and child identities both ways. This component also owns
//! the thin `encode`/`decode` wrappers, the pack↔dsl equivalence law and the command-envelope law.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::FormsSnapshot;
use store::PackError;

//#region 🔖️ArtifactPack
impl store::ArtifactPack for FormsSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.validate().map_err(store::PackError::Schema)?;
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        let snapshot = Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?;
        snapshot.validate().map_err(store::PackError::Schema)?;
        Ok(snapshot)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactPack

/// 📦️ Encodes a `FormsSnapshot` to its binary pack form.
pub fn encode(document: &FormsSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `FormsSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<FormsSnapshot, PackError> {
    <FormsSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🔖️Store
pub type FormsEnvelope = store::ArtifactEnvelope<FormsSnapshot, crate::op::FormMutation>;
pub type FormsStore = store::ArtifactStore<FormsSnapshot, crate::op::FormMutation>;

/// 🔐️ Opens a Forms store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `FormsStore::new` can be
/// read but never mutated, undone or closed. The editor app installs the same catalog through
/// `FormsPlayApp::build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_forms_store(envelope: FormsEnvelope) -> Result<OwnedFormsStore, store::VcsError> {
    let mut store = FormsStore::new(envelope).await?;
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
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
