//! 👥️ Architect presence — shareable live ephemeral state + mutations.
//!
//! 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: peer selection no longer lives
//! here — it broadcasts automatically via the framework's typed `PresenceInteraction` (assembled
//! from the "program" domain's `InteractionState`, zero app code).

use crate::artifacts::program::registers::AdjacencyKind;
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of architect view state (active register, adjacency filter, graph camera).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
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
    async fn apply(&self, _base: &ArchitectPresence) -> protocol::MutationApplyResult<ArchitectPresence> {
        Ok({
            self.clone()
        })
    }
    async fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for ArchitectPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
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
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for ArchitectPresence {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
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
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(rename_all = "camelCase")]
pub enum ArchitectPresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: ArchitectPresence,
    },
}

impl Mutation<ArchitectPresence> for ArchitectPresenceMutation {
    type Diff = ArchitectPresence;

    /// ✏️ Warning `mutation.no-op` if `presence` already equals `base` (empty diff), else the
    /// whole-snapshot replacement.
    async fn diff(&self, base: &ArchitectPresence) -> protocol::MutationOutcome<ArchitectPresence> {
        match self {
            Self::Snapshot { presence } => {
                if presence == base {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", "Presence already matches the requested value.");
                }
                protocol::MutationOutcome::new(presence.clone())
            }
        }
    }

    async fn inverse(&self, base: &ArchitectPresence) -> Vec<Self> {
        vec![Self::Snapshot { presence: base.clone() }]
    }
}

impl protocol::OpText for ArchitectPresenceMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for ArchitectPresenceMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
