//! ↔️ Persisted local filtering for one exact Architect Adjacency window.

use crate::registers::AdjacencyKind;
use semio_framework_value_derive::{FromValue, ToValue};

/// 🔍️ Selects the optional adjacency kind rendered by one concrete Adjacency window.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArchitectAdjacencyWindowConfig {
    pub adjacency_kind_filter: Option<AdjacencyKind>,
}

/// 🔁️ Changes the kind filter of one addressed Adjacency window.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum ArchitectAdjacencyWindowConfigMutation {
    SetAdjacencyKindFilter { adjacency_kind_filter: Option<AdjacencyKind> },
}

impl protocol::Mutation<ArchitectAdjacencyWindowConfig> for ArchitectAdjacencyWindowConfigMutation {
    type Diff = ArchitectAdjacencyWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🎚️config",
        semantic_kind: "set-adjacency-kind-filter",
        display_name: "Set Architect Adjacency Window Filter",
        emoji: "↔️",
        aggregate_variant: "SetAdjacencyKindFilter",
        payload_schema: "architect.adjacency-window.config",
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

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }

    fn diff(&self, base: &ArchitectAdjacencyWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        let Self::SetAdjacencyKindFilter { adjacency_kind_filter } = self;
        if base.adjacency_kind_filter == *adjacency_kind_filter {
            return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Adjacency filter is unchanged.");
        }
        protocol::MutationOutcome::new(ArchitectAdjacencyWindowConfig { adjacency_kind_filter: adjacency_kind_filter.clone() })
    }

    fn inverse(&self, base: &ArchitectAdjacencyWindowConfig) -> Vec<Self> {
        let Self::SetAdjacencyKindFilter { adjacency_kind_filter } = self;
        (base.adjacency_kind_filter != *adjacency_kind_filter)
            .then(|| Self::SetAdjacencyKindFilter { adjacency_kind_filter: base.adjacency_kind_filter.clone() })
            .into_iter()
            .collect()
    }
}

impl store::ArtifactDsl for ArchitectAdjacencyWindowConfig {
    const EXTENSION: &'static str = "architectadjacencywindowcfg";
    fn envelope_id() -> &'static str { "s.architect.program.adjacency-window.config" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let json: serde_json::Value = serde_json::from_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_string_pretty(&value).expect("Architect Adjacency window config JSON");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Architect Adjacency window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ArchitectAdjacencyWindowConfig {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("Architect Adjacency window config pack envelope mismatch".into()));
        }
        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> { None }
}

store::impl_whole_record_config!(ArchitectAdjacencyWindowConfig);

impl protocol::OpText for ArchitectAdjacencyWindowConfigMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> { dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1))) }
}

impl protocol::OpBinary for ArchitectAdjacencyWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

pub struct ArchitectAdjacencyWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for ArchitectAdjacencyWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::ARCHITECT_WINDOW_ADJACENCY;
    const SCHEMA: &'static str = "architect.adjacency-window.config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 1_024;
    type State = ArchitectAdjacencyWindowConfig;
    type Mutation = ArchitectAdjacencyWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

/// 🔍️ Reads the caller's exact Adjacency-window configuration or its initial value.
pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> ArchitectAdjacencyWindowConfig {
    view.window::<ArchitectAdjacencyWindowConfigOwner>().cloned().unwrap_or_default()
}

/// 📬️ Addresses a filter change to the caller's concrete Adjacency window.
pub fn addressed(view: &semio_framework_plugin::ViewModel, adjacency_kind_filter: Option<AdjacencyKind>) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("architect-adjacency-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("architect-adjacency-window-stale"))?;
    if kind != super::ARCHITECT_WINDOW_ADJACENCY {
        return Err(semio_framework_plugin::Fault::from("architect-adjacency-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<ArchitectAdjacencyWindowConfigOwner>(
        id,
        ArchitectAdjacencyWindowConfigMutation::SetAdjacencyKindFilter { adjacency_kind_filter },
    ))
}
