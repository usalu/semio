//! 🎚️ Persisted local configuration for one exact Forms Try window.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FormsTryWindowConfig {
    pub current_step_index: u32,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum FormsTryWindowConfigMutation {
    Snapshot { config: FormsTryWindowConfig },
}

impl protocol::Mutation<FormsTryWindowConfig> for FormsTryWindowConfigMutation {
    type Diff = FormsTryWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set Forms Try Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Snapshot",
        payload_schema: "forms.try-window-config",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &FormsTryWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self { Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()) }
    }
    fn inverse(&self, base: &FormsTryWindowConfig) -> Vec<Self> { vec![Self::Snapshot { config: base.clone() }] }
}

impl store::ArtifactDsl for FormsTryWindowConfig {
    const EXTENSION: &'static str = "formstrywindowcfg";
    fn envelope_id() -> &'static str { "s.forms.forms.try-window-config" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        dsl::json::from_json_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = dsl::json::to_json_string(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Forms Try window config envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for FormsTryWindowConfig {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = dsl::json::to_json_string(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) { return Err(store::PackError::Schema("Forms Try window config pack envelope mismatch".into())); }
        let text = std::str::from_utf8(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::json::from_json_str(text).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> { None }
}

store::impl_whole_record_config!(FormsTryWindowConfig);

impl protocol::OpText for FormsTryWindowConfigMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for FormsTryWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

pub struct FormsTryWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for FormsTryWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::FORMS_PLAY_WINDOW_TRY;
    const SCHEMA: &'static str = "forms.try-window-config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
    type State = FormsTryWindowConfig;
    type Mutation = FormsTryWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<FormsTryWindowConfigOwner>()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> FormsTryWindowConfig {
    snapshot.and_then(|snapshot| snapshot.get::<FormsTryWindowConfigOwner>()).cloned().unwrap_or_default()
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> FormsTryWindowConfig { from_snapshot(view.window) }

pub fn addressed(view: &semio_framework_plugin::ViewModel, config: FormsTryWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("forms-try-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("forms-try-window-stale"))?;
    if kind != super::FORMS_PLAY_WINDOW_TRY { return Err(semio_framework_plugin::Fault::from("forms-try-window-kind-required")); }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<FormsTryWindowConfigOwner>(id, FormsTryWindowConfigMutation::Snapshot { config }))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window-ownership/🦀️.rs"]
mod tests;
