//! 📦️ Equation artifact — binary document surface + laws (constitutional: pack). The
//! `store::ArtifactPack` impl for `EquationSnapshot` encodes the derived `EquationPackRecord` spec.

use crate::EquationSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

//#region 🔖️PackRecord
/// 🕸️ Derived pack record of an `EquationSnapshot`: the three composed-child handles, the authored
/// equation, and the live `(graph, geometry)` scene those handles own. The scene lives only in the
/// handles' `EquationWorkingScene` local owner, so it is persisted beside them — a bare-handle pack
/// could not be reopened by a fresh process (ticket 26/09/19, `📓️knowledge.md` §4).
#[derive(dsl::DslRecord)]
#[dsl(extension = "equation")]
struct EquationPackRecord {
    notation: crate::EquationNotationChild,
    results: crate::EquationResultsChild,
    computed: crate::EquationComputedChild,
    equation: dsl::DslValue,
    graph: dsl::DslValue,
    geometry: dsl::DslValue,
}

impl EquationPackRecord {
    fn from_snapshot(snapshot: &EquationSnapshot) -> Self {
        let scene = crate::equation_scene(snapshot);
        Self {
            notation: snapshot.notation.clone(),
            results: snapshot.results.clone(),
            computed: snapshot.computed.clone(),
            equation: dsl::ToValue::to_value(&snapshot.equation),
            graph: dsl::ToValue::to_value(&scene.graph),
            geometry: dsl::ToValue::to_value(&scene.geometry),
        }
    }

    fn into_snapshot(self) -> Result<EquationSnapshot, String> {
        let owner = std::sync::Arc::new(crate::EquationWorkingScene { graph: dsl::FromValue::from_value(self.graph).map_err(|e| e.to_string())?, geometry: dsl::FromValue::from_value(self.geometry).map_err(|e| e.to_string())? });
        Ok(EquationSnapshot {
            notation: self.notation.with_local_owner(owner.clone()),
            results: self.results.with_local_owner(owner.clone()),
            computed: self.computed.with_local_owner(owner),
            equation: dsl::FromValue::from_value(self.equation).map_err(|e| e.to_string())?,
        })
    }
}

/// 🖨️ The derived text body: the same `EquationPackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &EquationSnapshot) -> String {
    dsl::print(&EquationPackRecord::from_snapshot(snapshot).__dsl_to_record(), &EquationPackRecord::__dsl_spec(), dsl::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `EquationPackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<EquationSnapshot, store::TextError> {
    let record = dsl::parse(body, &EquationPackRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
    EquationPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
}
//#endregion 🔖️PackRecord

//#region 🔖️ArtifactPack
impl store::ArtifactPack for EquationSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = store::pack_rt::encode_document(&EquationPackRecord::__dsl_spec(), &EquationPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &EquationPackRecord::__dsl_spec(), options)?;
        EquationPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(PackError::Schema)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(EquationPackRecord::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactPack

/// 📦️ Encodes a `EquationSnapshot` to its binary pack form.
pub fn encode(snapshot: &EquationSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

/// 📖️ Decodes a `EquationSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<EquationSnapshot, PackError> {
    <EquationSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🔖️Store
pub type EquationEnvelope = store::ArtifactEnvelope<crate::EquationSnapshot, crate::schema::mutations::EquationMutation>;
pub type EquationStore = store::ArtifactStore<crate::EquationSnapshot, crate::schema::mutations::EquationMutation>;

/// 🔐️ Opens a Equation store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `EquationStore::new` can be
/// read but never mutated, undone or closed. The app installs the same catalog through
/// `build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_equation_store(envelope: EquationEnvelope) -> Result<OwnedEquationStore, store::VcsError> {
    let mut store = EquationStore::new(envelope).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<crate::EquationSnapshot, crate::schema::mutations::EquationMutation>());
    Ok(OwnedEquationStore(store))
}

/// 🔚 A standalone Equation store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedEquationStore(EquationStore);

impl OwnedEquationStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Equation document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedEquationStore {
    type Target = EquationStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedEquationStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedEquationStore {
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

