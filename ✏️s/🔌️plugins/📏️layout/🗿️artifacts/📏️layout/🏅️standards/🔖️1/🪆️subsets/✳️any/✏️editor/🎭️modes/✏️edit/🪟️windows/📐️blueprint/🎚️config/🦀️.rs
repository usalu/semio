//! 🎚️ Persisted local configuration for one exact Layout Blueprint or Preview window.

use crate::LayoutCamera;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct LayoutWindowConfig {
    pub active_page_id: String,
    pub camera: LayoutCamera,
}

impl Default for LayoutWindowConfig {
    fn default() -> Self {
        Self { active_page_id: "page-1".into(), camera: LayoutCamera::default() }
    }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum LayoutWindowConfigMutation {
    Snapshot { config: LayoutWindowConfig },
}

impl protocol::Mutation<LayoutWindowConfig> for LayoutWindowConfigMutation {
    type Diff = LayoutWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set Layout Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Snapshot",
        payload_schema: "layout.windowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &LayoutWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self { Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()) }
    }
    fn inverse(&self, base: &LayoutWindowConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: base.clone() }]
    }
}

impl store::ArtifactDsl for LayoutWindowConfig {
    const EXTENSION: &'static str = "layoutwindowcfg";
    fn envelope_id() -> &'static str { "s.layout.layout.windowconfig" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let json: serde_json::Value = serde_json::from_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_string_pretty(&value).expect("Layout window config JSON");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Layout window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for LayoutWindowConfig {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) { return Err(store::PackError::Schema("Layout window pack envelope mismatch".into())); }
        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> { None }
}

store::impl_whole_record_config!(LayoutWindowConfig);

impl protocol::OpText for LayoutWindowConfigMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for LayoutWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

macro_rules! config_owner {
    ($owner:ident, $kind:path) => {
        pub struct $owner;
        impl semio_framework_plugin::WindowConfigOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = "layout.windowconfig";
            const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
            type State = LayoutWindowConfig;
            type Mutation = LayoutWindowConfigMutation;
            fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
            fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
        }
    };
}

config_owner!(LayoutBlueprintWindowConfigOwner, super::LAYOUT_PLAY_WINDOW_BLUEPRINT);
config_owner!(LayoutPreviewWindowConfigOwner, crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW);

pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<LayoutBlueprintWindowConfigOwner>()?;
    registry.register::<LayoutPreviewWindowConfigOwner>()
}

fn from_owner<O: semio_framework_plugin::WindowConfigOwner<State = LayoutWindowConfig, Mutation = LayoutWindowConfigMutation>>(snapshot: &semio_framework_plugin::WindowConfigSnapshot) -> Option<LayoutWindowConfig> {
    snapshot.get::<O>().cloned()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> LayoutWindowConfig {
    let Some(snapshot) = snapshot else { return LayoutWindowConfig::default() };
    match snapshot.window_kind_id() {
        super::LAYOUT_PLAY_WINDOW_BLUEPRINT => from_owner::<LayoutBlueprintWindowConfigOwner>(snapshot),
        crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW => from_owner::<LayoutPreviewWindowConfigOwner>(snapshot),
        _ => None,
    }.unwrap_or_default()
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> LayoutWindowConfig {
    from_snapshot(view.window)
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, config: LayoutWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("layout-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("layout-window-stale"))?;
    let mutation = LayoutWindowConfigMutation::Snapshot { config };
    match kind {
        super::LAYOUT_PLAY_WINDOW_BLUEPRINT => Ok(semio_framework_plugin::WindowConfigMutation::of::<LayoutBlueprintWindowConfigOwner>(id, mutation)),
        crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW => Ok(semio_framework_plugin::WindowConfigMutation::of::<LayoutPreviewWindowConfigOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("layout-window-kind-required")),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window-ownership/🦀️.rs"]
mod tests;
