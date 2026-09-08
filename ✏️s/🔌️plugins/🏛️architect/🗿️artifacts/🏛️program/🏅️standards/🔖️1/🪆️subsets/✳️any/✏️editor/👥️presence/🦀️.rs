//! 👥️ Architect presence — shareable live ephemeral state + mutations.
//!
//! 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: peer selection no longer lives
//! here — it broadcasts automatically via the framework's typed `PresenceInteraction` (assembled
//! from the "program" domain's `InteractionState`, zero app code).

use crate::registers::AdjacencyKind;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of architect view state (active register, adjacency filter, graph camera).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(extension = "architect.presence")]
#[dsl(layout = "lines")]
pub struct ArchitectPresence {
    pub active_register: String,
    pub adjacency_kind_filter: Option<AdjacencyKind>,
    pub graph_camera_x: f64,
    pub graph_camera_y: f64,
    pub graph_camera_zoom: f64,
}

impl Default for ArchitectPresence {
    fn default() -> Self {
        Self { active_register: "elements".into(), adjacency_kind_filter: None, graph_camera_x: 0.0, graph_camera_y: 0.0, graph_camera_zoom: 1.0 }
    }
}

impl protocol::MutationDiff<ArchitectPresence> for ArchitectPresence {
    fn apply(&self, _base: &ArchitectPresence) -> protocol::MutationApplyResult<ArchitectPresence> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for ArchitectPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for ArchitectPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    use dsl::os_pack as pack;

    #[test]
    fn architect_presence_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
        let base: ArchitectPresence = pack::from_json_str(&vectors["base"].to_string()).expect("owned base decoder");
        assert_eq!(<ArchitectPresenceMutation as Mutation<ArchitectPresence>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
        for vector in vectors["cases"].as_array().expect("cases") {
            let mutation: ArchitectPresenceMutation = pack::from_json_str(&vector["mutation"].to_string()).expect("owned operation decoder");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
            assert_eq!(ArchitectPresenceMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
            assert_eq!(ArchitectPresenceMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
            let outcome = mutation.diff(&base);
            assert!(outcome.messages().is_empty());
            let next = outcome.diff().apply(&base).expect("apply diff");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
            let next_for_noop = next.clone();
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("apply inverse"));
            assert_eq!(restored, base);
            let noop = mutation.diff(&next_for_noop);
            assert!(!noop.messages().is_empty());
        }
    }
}
