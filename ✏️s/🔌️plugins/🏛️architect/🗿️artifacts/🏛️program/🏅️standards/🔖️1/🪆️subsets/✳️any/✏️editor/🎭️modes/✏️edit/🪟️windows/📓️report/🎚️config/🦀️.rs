//! 📓️ Persisted local authored-record selection for one exact Architect Report window.

use crate::EntityId;
use semio_framework_value_derive::{FromValue, ToValue};

/// 📑️ Names the authored ReportRecord rendered by one concrete Report window.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "s.architect.program.report-window.config", extension = "architectreportwindowcfg")]
pub struct ArchitectReportWindowConfig {
    pub selected_report_id: Option<EntityId>,
}

/// 🎯️ Sets the selected report to `value`, `None` clearing the selection — a present edit is how a diff writes an optional field, so a cleared
/// selection stays distinguishable from an untouched one.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ReportSelectionSet {
    pub value: Option<EntityId>,
}

/// 🔺️ Sparse field diff of one report selection window config: a present field is written, the rest of the config is untouched.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ArchitectReportWindowConfigDiff {
    pub selected_report_id: Option<ReportSelectionSet>,
}

impl protocol::MutationDiff<ArchitectReportWindowConfig> for ArchitectReportWindowConfigDiff {
    fn apply(&self, base: &ArchitectReportWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<ArchitectReportWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.selected_report_id {
            next.selected_report_id = value.value.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.selected_report_id.is_some() {
            self.selected_report_id = other.selected_report_id;
        }
    }
}

impl protocol::DiffAlgebra<ArchitectReportWindowConfig> for ArchitectReportWindowConfigDiff {
    fn inverse(&self, base: &ArchitectReportWindowConfig) -> Self {
        Self { selected_report_id: self.selected_report_id.as_ref().map(|_| ReportSelectionSet { value: base.selected_report_id.clone() }) }
    }

    fn is_empty(&self) -> bool {
        self.selected_report_id.is_none()
    }
}

/// 📦️ Payload of `SelectReport`; its field names are the wire names of the former named variant.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelectReport {
    pub selected_report_id: Option<EntityId>,
}

/// 🔁️ Changes the authored ReportRecord selected by one addressed Report window.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum ArchitectReportWindowConfigMutation {
    SelectReport(SelectReport),
}

impl protocol::Mutation<ArchitectReportWindowConfig> for ArchitectReportWindowConfigMutation {
    type Diff = ArchitectReportWindowConfigDiff;
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
        outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp],
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
        let Self::SelectReport(SelectReport { selected_report_id }) = self;
        if base.selected_report_id == *selected_report_id {
            return protocol::MutationOutcome::new(ArchitectReportWindowConfigDiff::default()).warning("mutation.no-op", "Report selection is unchanged.");
        }
        protocol::MutationOutcome::new(ArchitectReportWindowConfigDiff { selected_report_id: Some(ReportSelectionSet { value: selected_report_id.clone() }) })
    }

    fn inverse(&self, base: &ArchitectReportWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        let Self::SelectReport(SelectReport { selected_report_id }) = self;
        (base.selected_report_id != *selected_report_id).then(|| Self::SelectReport(SelectReport { selected_report_id: base.selected_report_id.clone() })).into_iter().collect()
    
    })())
}
}

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for ArchitectReportWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Architect report window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for ArchitectReportWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ConfigRecord for ArchitectReportWindowConfig {}

impl protocol::OpText for ArchitectReportWindowConfigMutation {
    fn print_op(&self) -> String { semio_framework_pack_json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> { semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1))) }
}

impl protocol::OpBinary for ArchitectReportWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<ArchitectReportWindowConfig> for ArchitectReportWindowConfigMutation {
    fn exchange(self, post: &mut ArchitectReportWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SelectReport(SelectReport { selected_report_id }) => Self::SelectReport(SelectReport { selected_report_id: std::mem::replace(&mut post.selected_report_id, selected_report_id) }),
        })
    }
}

pub struct ArchitectReportWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for ArchitectReportWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::ARCHITECT_WINDOW_REPORT;
    const SCHEMA: &'static str = "architect.report-window.config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 1_024;
    type State = ArchitectReportWindowConfig;
    type Mutation = ArchitectReportWindowConfigMutation;
    type Edit = semio_framework_plugin::app::WindowConfigApplyEdit<Self::State, Self::Mutation>;
    const MAXIMUM_PREPARATION_DEPTH: usize = 64;
    fn build_retained_edit() -> std::sync::Arc<Self::Edit> { std::sync::Arc::new(semio_framework_plugin::app::WindowConfigApplyEdit::new()) }
    fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
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
        ArchitectReportWindowConfigMutation::SelectReport(SelectReport { selected_report_id: Some(selected_report_id) }),
    )))
}
