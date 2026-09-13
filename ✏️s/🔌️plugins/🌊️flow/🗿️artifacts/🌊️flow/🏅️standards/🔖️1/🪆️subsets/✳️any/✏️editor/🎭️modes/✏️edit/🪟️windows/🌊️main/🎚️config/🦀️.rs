//! 🎚️ Persisted local configuration for one exact Flow main window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

use std::collections::HashMap;

impl FlowMainWindowConfig {
    pub fn automation_enabled(&self) -> HashMap<String, bool> {
        serde_json::from_str(&self.automation_enabled_json).unwrap_or_default()
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum FlowMainWindowConfigMutation {
    Snapshot { config: FlowMainWindowConfig },
}

impl protocol::Mutation<FlowMainWindowConfig> for FlowMainWindowConfigMutation {
    type Diff = FlowMainWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set Flow Main Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Snapshot",
        payload_schema: "flow.mainwindowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }

    fn diff(&self, _base: &FlowMainWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()),
        }
    }

    fn inverse(&self, base: &FlowMainWindowConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: base.clone() }]
    }
}

impl store::ArtifactDsl for FlowMainWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Flow window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for FlowMainWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}

store::impl_whole_record_config!(FlowMainWindowConfig);

impl protocol::OpText for FlowMainWindowConfigMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for FlowMainWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

pub struct FlowMainWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for FlowMainWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::FLOW_PLAY_WINDOW_MAIN;
    const SCHEMA: &'static str = "flow.mainwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = FlowMainWindowConfig;
    type Mutation = FlowMainWindowConfigMutation;

    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
        semio_framework_plugin::bounded_window_config_store_owners::<Self>()
    }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
    }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
    }
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> FlowMainWindowConfig {
    view.window::<FlowMainWindowConfigOwner>().cloned().unwrap_or_default()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> FlowMainWindowConfig {
    snapshot.and_then(|snapshot| snapshot.get::<FlowMainWindowConfigOwner>()).cloned().unwrap_or_default()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, config: FlowMainWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("flow-main-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str());
    if kind != Some(super::FLOW_PLAY_WINDOW_MAIN) {
        return Err(semio_framework_plugin::Fault::from("flow-main-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<FlowMainWindowConfigOwner>(id, FlowMainWindowConfigMutation::Snapshot { config }))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window-ownership/🦀️.rs"]
mod tests;
