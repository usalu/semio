//! 📓️ Persisted local authored-record selection for one exact Architect Report window.

use crate::EntityId;
use semio_framework_value_derive::{FromValue, ToValue};

/// 📑️ Names the authored ReportRecord rendered by one concrete Report window.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArchitectReportWindowConfig {
    pub selected_report_id: Option<EntityId>,
}

/// 🔁️ Changes the authored ReportRecord selected by one addressed Report window.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum ArchitectReportWindowConfigMutation {
    SelectReport { selected_report_id: Option<EntityId> },
}

impl protocol::Mutation<ArchitectReportWindowConfig> for ArchitectReportWindowConfigMutation {
    type Diff = ArchitectReportWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📓️report/🎚️config",
        semantic_kind: "select-report",
        display_name: "Select Architect Report Window Record",
        emoji: "📓️",
        aggregate_variant: "SelectReport",
        payload_schema: "architect.report-window.config",
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

    fn diff(&self, base: &ArchitectReportWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        let Self::SelectReport { selected_report_id } = self;
        if base.selected_report_id == *selected_report_id {
            return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Report selection is unchanged.");
        }
        protocol::MutationOutcome::new(ArchitectReportWindowConfig { selected_report_id: selected_report_id.clone() })
    }

    fn inverse(&self, base: &ArchitectReportWindowConfig) -> Vec<Self> {
        let Self::SelectReport { selected_report_id } = self;
        (base.selected_report_id != *selected_report_id).then(|| Self::SelectReport { selected_report_id: base.selected_report_id.clone() }).into_iter().collect()
    }
}

impl store::ArtifactDsl for ArchitectReportWindowConfig {
    const EXTENSION: &'static str = "architectreportwindowcfg";
    fn envelope_id() -> &'static str { "s.architect.program.report-window.config" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let json: serde_json::Value = serde_json::from_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_string_pretty(&value).expect("Architect Report window config JSON");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Architect Report window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ArchitectReportWindowConfig {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("Architect Report window config pack envelope mismatch".into()));
        }
        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> { None }
}

store::impl_whole_record_config!(ArchitectReportWindowConfig);

impl protocol::OpText for ArchitectReportWindowConfigMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> { dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1))) }
}

impl protocol::OpBinary for ArchitectReportWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

pub struct ArchitectReportWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for ArchitectReportWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::ARCHITECT_WINDOW_REPORT;
    const SCHEMA: &'static str = "architect.report-window.config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 1_024;
    type State = ArchitectReportWindowConfig;
    type Mutation = ArchitectReportWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

/// 📖️ Reads the caller's exact Report-window configuration or its empty selection.
pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> ArchitectReportWindowConfig {
    view.window::<ArchitectReportWindowConfigOwner>().cloned().unwrap_or_default()
}

/// 📬️ Selects a record only when the invocation itself comes from an exact Report window.
pub fn addressed_if_report(
    view: Option<&semio_framework_plugin::ViewModel>,
    selected_report_id: EntityId,
) -> Result<Option<semio_framework_plugin::WindowConfigMutation>, semio_framework_plugin::Fault> {
    let Some(view) = view else { return Ok(None) };
    let Some(id) = view.window_id.as_deref() else { return Ok(None) };
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("architect-report-window-stale"))?;
    if kind != super::ARCHITECT_WINDOW_REPORT {
        return Ok(None);
    }
    Ok(Some(semio_framework_plugin::WindowConfigMutation::of::<ArchitectReportWindowConfigOwner>(
        id,
        ArchitectReportWindowConfigMutation::SelectReport { selected_report_id: Some(selected_report_id) },
    )))
}
