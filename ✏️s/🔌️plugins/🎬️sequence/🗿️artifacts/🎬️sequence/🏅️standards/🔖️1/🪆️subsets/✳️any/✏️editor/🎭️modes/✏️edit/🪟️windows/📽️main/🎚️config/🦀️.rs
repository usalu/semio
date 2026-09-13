//! 🎚️ Persisted local configuration for one exact Sequence graph window.

use crate::SequenceCamera;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.sequence.sequence.mainwindowconfig", extension = "sequencemainwindowcfg", layout = "lines")]
pub struct SequenceMainWindowConfig {
    pub orientation: String,
    #[dsl(block)]
    pub camera: SequenceCamera,
}

impl Default for SequenceMainWindowConfig {
    fn default() -> Self { Self { orientation: "leftRight".into(), camera: SequenceCamera::default() } }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum SequenceMainWindowConfigMutation {
    Snapshot { config: SequenceMainWindowConfig },
}

impl protocol::Mutation<SequenceMainWindowConfig> for SequenceMainWindowConfigMutation {
    type Diff = SequenceMainWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set Sequence Main Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Snapshot",
        payload_schema: "sequence.mainwindowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &SequenceMainWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self { Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()) }
    }
    fn inverse(&self, base: &SequenceMainWindowConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: base.clone() }]
    }
}

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for SequenceMainWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Sequence window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for SequenceMainWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

store::impl_whole_record_config!(SequenceMainWindowConfig);
impl protocol::OpText for SequenceMainWindowConfigMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
impl protocol::OpBinary for SequenceMainWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

pub struct SequenceMainWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for SequenceMainWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::SEQUENCE_PLAY_WINDOW_MAIN;
    const SCHEMA: &'static str = "sequence.mainwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = SequenceMainWindowConfig;
    type Mutation = SequenceMainWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> SequenceMainWindowConfig {
    view.window::<SequenceMainWindowConfigOwner>().cloned().unwrap_or_default()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> SequenceMainWindowConfig {
    snapshot.and_then(|snapshot| snapshot.get::<SequenceMainWindowConfigOwner>()).cloned().unwrap_or_default()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, config: SequenceMainWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("sequence-main-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("sequence-window-stale"))?;
    if kind != super::SEQUENCE_PLAY_WINDOW_MAIN {
        return Err(semio_framework_plugin::Fault::from("sequence-main-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<SequenceMainWindowConfigOwner>(id, SequenceMainWindowConfigMutation::Snapshot { config }))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window-ownership/🦀️.rs"]
mod tests;
