//! 🕸️ Persisted local navigation for one exact Architect Graph window.

use semio_framework_os_kernel::Viewport2d;
use semio_framework_value_derive::{FromValue, ToValue};

/// 🪟️ Owns the shared two-dimensional viewport of one concrete Graph window.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArchitectGraphWindowConfig {
    pub viewport: Viewport2d,
}

/// 🔁️ Changes the shared viewport of one addressed Graph window.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum ArchitectGraphWindowConfigMutation {
    SetViewport { viewport: Viewport2d },
}

impl protocol::Mutation<ArchitectGraphWindowConfig> for ArchitectGraphWindowConfigMutation {
    type Diff = ArchitectGraphWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config",
        semantic_kind: "set-viewport",
        display_name: "Set Architect Graph Window Viewport",
        emoji: "🕸️",
        aggregate_variant: "SetViewport",
        payload_schema: "architect.graph-window.config",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::Warning],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[
            protocol::MutationLanguageSurface::Rust,
            protocol::MutationLanguageSurface::Typescript,
            protocol::MutationLanguageSurface::JsonSchema,
            protocol::MutationLanguageSurface::Graphql,
            protocol::MutationLanguageSurface::Protobuf,
            protocol::MutationLanguageSurface::Text,
            protocol::MutationLanguageSurface::Binary,
        ],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }

    fn diff(&self, base: &ArchitectGraphWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        let Self::SetViewport { viewport } = self;
        if base.viewport == *viewport {
            return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Graph viewport is unchanged.");
        }
        protocol::MutationOutcome::new(ArchitectGraphWindowConfig { viewport: viewport.clone() })
    }

    fn inverse(&self, base: &ArchitectGraphWindowConfig) -> Vec<Self> {
        let Self::SetViewport { viewport } = self;
        (base.viewport != *viewport).then(|| Self::SetViewport { viewport: base.viewport.clone() }).into_iter().collect()
    }
}

impl store::ArtifactDsl for ArchitectGraphWindowConfig {
    const EXTENSION: &'static str = "architectgraphwindowcfg";
    fn envelope_id() -> &'static str { "s.architect.program.graph-window.config" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let json: serde_json::Value = serde_json::from_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_string_pretty(&value).expect("Architect Graph window config JSON");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Architect Graph window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ArchitectGraphWindowConfig {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("Architect Graph window config pack envelope mismatch".into()));
        }
        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> { None }
}

store::impl_whole_record_config!(ArchitectGraphWindowConfig);

impl protocol::OpText for ArchitectGraphWindowConfigMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> { dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1))) }
}

impl protocol::OpBinary for ArchitectGraphWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

pub struct ArchitectGraphWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for ArchitectGraphWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::ARCHITECT_WINDOW_GRAPH;
    const SCHEMA: &'static str = "architect.graph-window.config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 1_024;
    type State = ArchitectGraphWindowConfig;
    type Mutation = ArchitectGraphWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

/// 🧭️ Reads the caller's exact Graph-window configuration or its initial value.
pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> ArchitectGraphWindowConfig {
    view.window::<ArchitectGraphWindowConfigOwner>().cloned().unwrap_or_default()
}

/// 📬️ Addresses a viewport change to the caller's concrete Graph window.
pub fn addressed(view: &semio_framework_plugin::ViewModel, viewport: Viewport2d) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("architect-graph-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("architect-graph-window-stale"))?;
    if kind != super::ARCHITECT_WINDOW_GRAPH {
        return Err(semio_framework_plugin::Fault::from("architect-graph-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<ArchitectGraphWindowConfigOwner>(
        id,
        ArchitectGraphWindowConfigMutation::SetViewport { viewport },
    ))
}
