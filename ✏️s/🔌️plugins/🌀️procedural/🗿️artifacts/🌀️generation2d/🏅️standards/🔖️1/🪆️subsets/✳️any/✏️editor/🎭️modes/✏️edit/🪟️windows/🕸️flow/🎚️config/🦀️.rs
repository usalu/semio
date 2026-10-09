//! 🎚️ Persisted local navigation for one exact Generation2d main window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

impl Default for Generation2dMainWindowConfig {
    fn default() -> Self { Self { viewport: semio_framework_os_kernel::Viewport2d::default() } }
}

impl store::ArtifactDsl for Generation2dMainWindowConfig {
    const EXTENSION: &'static str = "generation2dmainwindowcfg";
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Generation2d main-window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Generation2dMainWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Generation2d main-window pack envelope mismatch")));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(Self::__dsl_spec()) }
}

semio_framework_os_kernel::config_record! {
    record: Generation2dMainWindowConfig,
    diff: Generation2dMainWindowConfigDiff,
    set: Generation2dMainWindowConfigMutation,
    owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow",
    payload_schema: "procedural.generation2d.mainwindowconfig",
    emoji: "🎚️",
    fields: {
        viewport: semio_framework_os_kernel::Viewport2d => SetViewport "set-viewport",
    },
}









pub struct Generation2dMainWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for Generation2dMainWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::GENERATION2D_PLAY_WINDOW_MAIN;
    const SCHEMA: &'static str = "procedural.generation2d.mainwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
    type State = Generation2dMainWindowConfig;
    type Mutation = Generation2dMainWindowConfigMutation;
    fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
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

pub fn addressed(view: &semio_framework_plugin::ViewModel, base: &Generation2dMainWindowConfig, config: Generation2dMainWindowConfig) -> Result<Vec<semio_framework_plugin::WindowConfigMutation>, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("generation2d-main-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("generation2d-main-window-stale"))?;
    if kind != super::GENERATION2D_PLAY_WINDOW_MAIN { return Err(semio_framework_plugin::Fault::from("generation2d-main-window-kind-required")); }
    Ok(Generation2dMainWindowConfigMutation::setting(base, &config).into_iter().map(|mutation| semio_framework_plugin::WindowConfigMutation::of::<Generation2dMainWindowConfigOwner>(id, mutation)).collect())
}
