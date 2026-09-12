//! 🎚️ Persisted local navigation for one exact Generation2d main window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

impl Default for Generation2dMainWindowConfig {
    fn default() -> Self { Self { viewport: semio_framework::Viewport2d::default() } }
}

impl store::ArtifactDsl for Generation2dMainWindowConfig {
    const EXTENSION: &'static str = "generation2dmainwindowcfg";
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Generation2d main-window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Generation2dMainWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("Generation2d main-window pack envelope mismatch".into()));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}

store::impl_whole_record_config!(Generation2dMainWindowConfig);

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslOps)]
pub enum Generation2dMainWindowConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot { #[dsl(block)] config: Box<Generation2dMainWindowConfig> },
}

impl protocol::OpText for Generation2dMainWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        for (keyword, spec_fn) in &<Self as dsl::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown Generation2d main-window mutation '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let spec = <Self as dsl::DslVariants>::variants().iter().find(|(key, _)| key == &keyword).map(|(_, spec)| spec()).expect("Generation2d main-window mutation variant");
        dsl::print(&record, &spec, dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Generation2dMainWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}

impl protocol::Mutation<Generation2dMainWindowConfig> for Generation2dMainWindowConfigMutation {
    type Diff = Generation2dMainWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set Generation2d Main Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Snapshot",
        payload_schema: "procedural.generation2d.mainwindowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::Typescript, protocol::MutationLanguageSurface::JsonSchema, protocol::MutationLanguageSurface::Graphql, protocol::MutationLanguageSurface::Protobuf],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &Generation2dMainWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self { Self::Snapshot { config } => protocol::MutationOutcome::new(config.as_ref().clone()) }
    }
    fn inverse(&self, base: &Generation2dMainWindowConfig) -> Vec<Self> { vec![Self::Snapshot { config: Box::new(base.clone()) }] }
}

pub struct Generation2dMainWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for Generation2dMainWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::GENERATION2D_PLAY_WINDOW_MAIN;
    const SCHEMA: &'static str = "procedural.generation2d.mainwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
    type State = Generation2dMainWindowConfig;
    type Mutation = Generation2dMainWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> { registry.register::<Generation2dMainWindowConfigOwner>() }

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> Generation2dMainWindowConfig {
    view.window::<Generation2dMainWindowConfigOwner>().cloned().unwrap_or_default()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Generation2dMainWindowConfig {
    snapshot.filter(|snapshot| snapshot.window_kind_id() == super::GENERATION2D_PLAY_WINDOW_MAIN).and_then(|snapshot| snapshot.get::<Generation2dMainWindowConfigOwner>()).cloned().unwrap_or_default()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, config: Generation2dMainWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("generation2d-main-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("generation2d-main-window-stale"))?;
    if kind != super::GENERATION2D_PLAY_WINDOW_MAIN { return Err(semio_framework_plugin::Fault::from("generation2d-main-window-kind-required")); }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<Generation2dMainWindowConfigOwner>(id, Generation2dMainWindowConfigMutation::Snapshot { config: Box::new(config) }))
}
