//! 🎚️ Persisted local configuration for one exact Sequence graph window.

use crate::SequenceCamera;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.sequence.sequence.mainwindowconfig", extension = "sequencemainwindowcfg")]
pub struct SequenceMainWindowConfig {
    pub orientation: String,
    #[dsl(block)]
    pub camera: SequenceCamera,
}

impl Default for SequenceMainWindowConfig {
    fn default() -> Self { Self { orientation: "leftRight".into(), camera: SequenceCamera::default() } }
}





/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for SequenceMainWindowConfig {
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

semio_framework_os_kernel::config_diff! {
    record: SequenceMainWindowConfig,
    diff: SequenceMainWindowConfigDiff,
    fields: {
        orientation: String,
        camera: SequenceCamera,
    },
}

/// 🎚️ The field-set mutations of [`SequenceMainWindowConfig`]: exactly one field each.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum SequenceMainWindowConfigMutation {
    SetOrientation(String),
    SetCamera(SequenceCamera),
}

impl SequenceMainWindowConfigMutation {
    /// 🎚️ The set mutations that turn `base` into `next`: one per field where they differ, in field order.
    pub fn setting(base: &SequenceMainWindowConfig, next: &SequenceMainWindowConfig) -> Vec<Self> {
        let mut mutations = Vec::new();
        if base.orientation != next.orientation {
            mutations.push(Self::SetOrientation(next.orientation.clone()));
        }
        if base.camera != next.camera {
            mutations.push(Self::SetCamera(next.camera.clone()));
        }
        mutations
    }
}

impl protocol::Mutation<SequenceMainWindowConfig> for SequenceMainWindowConfigMutation {
    type Diff = SequenceMainWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/set-orientation",
            semantic_kind: "set-orientation",
            display_name: "set-orientation",
            emoji: "🎚️",
            aggregate_variant: "SetOrientation",
            payload_schema: "sequence.mainwindowconfig",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/set-camera",
            semantic_kind: "set-camera",
            display_name: "set-camera",
            emoji: "🎚️",
            aggregate_variant: "SetCamera",
            payload_schema: "sequence.mainwindowconfig",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
    ];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::SetOrientation(_) => &Self::DESCRIPTORS[0],
            Self::SetCamera(_) => &Self::DESCRIPTORS[1],
        }
    }
    fn diff(&self, base: &SequenceMainWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::SetOrientation(orientation) if base.orientation == *orientation => protocol::MutationOutcome::empty(),
            Self::SetOrientation(orientation) => protocol::MutationOutcome::new(SequenceMainWindowConfigDiff { orientation: Some(orientation.clone()), ..Default::default() }),
            Self::SetCamera(camera) if base.camera == *camera => protocol::MutationOutcome::empty(),
            Self::SetCamera(camera) => protocol::MutationOutcome::new(SequenceMainWindowConfigDiff { camera: Some(camera.clone()), ..Default::default() }),
        }
    }
    fn inverse(&self, base: &SequenceMainWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetOrientation(_) => Self::SetOrientation(base.orientation.clone()),
            Self::SetCamera(_) => Self::SetCamera(base.camera.clone()),
        }])
    }
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<SequenceMainWindowConfig> for SequenceMainWindowConfigMutation {
    fn exchange(self, post: &mut SequenceMainWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SetOrientation(orientation) => Self::SetOrientation(std::mem::replace(&mut post.orientation, orientation)),
            Self::SetCamera(camera) => Self::SetCamera(std::mem::replace(&mut post.camera, camera)),
        })
    }
    fn payload_bytes(&self) -> usize {
        match self {
            Self::SetOrientation(orientation) => orientation.len(),
            Self::SetCamera(_) => 0,
        }
    }
}

impl protocol::OpText for SequenceMainWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
}

impl protocol::OpBinary for SequenceMainWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

pub struct SequenceMainWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for SequenceMainWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::SEQUENCE_PLAY_WINDOW_MAIN;
    const SCHEMA: &'static str = "sequence.mainwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = SequenceMainWindowConfig;
    type Mutation = SequenceMainWindowConfigMutation;
    type Edit = store::snapshot_clone_preparation::ConfigApplyEdit<SequenceMainWindowConfig, SequenceMainWindowConfigMutation>;
    const MAXIMUM_PREPARATION_DEPTH: usize = 64;
    fn build_retained_edit() -> std::sync::Arc<Self::Edit> {
        std::sync::Arc::new(store::snapshot_clone_preparation::ConfigApplyEdit::new())
    }
    fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> SequenceMainWindowConfig {
    view.window::<SequenceMainWindowConfigOwner>().cloned().unwrap_or_default()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> SequenceMainWindowConfig {
    snapshot.and_then(|snapshot| snapshot.get::<SequenceMainWindowConfigOwner>()).cloned().unwrap_or_default()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, base: &SequenceMainWindowConfig, config: SequenceMainWindowConfig) -> Result<Vec<semio_framework_plugin::WindowConfigMutation>, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| crate::editor::sequence::sequence_fault("sequence.window.unavailable", "sequence-main-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| crate::editor::sequence::sequence_fault("sequence.window.unavailable", "sequence-window-stale"))?;
    if kind != super::SEQUENCE_PLAY_WINDOW_MAIN {
        return Err(crate::editor::sequence::sequence_fault("sequence.window.unavailable", "sequence-main-window-kind-required"));
    }
    Ok(SequenceMainWindowConfigMutation::setting(base, &config).into_iter().map(|mutation| semio_framework_plugin::WindowConfigMutation::of::<SequenceMainWindowConfigOwner>(id, mutation)).collect())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window/🦀️.rs"]
mod tests;
