//! 🎚️ Persisted local navigation for one exact Generation2d generate preview window.

#[path = "🧬️schema/🦀️.rs"] mod schema;
pub use schema::*;

impl Default for Generation2dGeneratePreviewWindowConfig {
    fn default() -> Self { Self { viewport: semio_framework_os_kernel::Viewport2d::default() } }
}




impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Generation2dGeneratePreviewWindowConfig {

}

semio_framework_os_kernel::config_record! {
    record: Generation2dGeneratePreviewWindowConfig,
    diff: Generation2dGeneratePreviewWindowConfigDiff,
    set: Generation2dGeneratePreviewWindowConfigMutation,
    owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview",
    payload_schema: "procedural.generation2d.generatepreviewwindowconfig",
    emoji: "🎚️",
    fields: {
        viewport: semio_framework_os_kernel::Viewport2d => SetViewport "set-viewport",
    },
}









pub struct Generation2dGeneratePreviewWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for Generation2dGeneratePreviewWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW;
    const SCHEMA: &'static str = "procedural.generation2d.generatepreviewwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
    type State = Generation2dGeneratePreviewWindowConfig;
    type Mutation = Generation2dGeneratePreviewWindowConfigMutation;
    type Edit = semio_framework_plugin::app::WindowConfigApplyEdit<Generation2dGeneratePreviewWindowConfig, Generation2dGeneratePreviewWindowConfigMutation>;
    const MAXIMUM_PREPARATION_DEPTH: usize = 64;
    fn build_retained_edit() -> std::sync::Arc<Self::Edit> { std::sync::Arc::new(semio_framework_plugin::app::WindowConfigApplyEdit::new()) }
    fn preparation_source_birth() -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_framework_plugin::bounded_window_config_preparation_source_birth::<Self>() }
    fn build_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_store_owners::<Self>(grant) }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> { registry.register::<Generation2dGeneratePreviewWindowConfigOwner>() }
pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> Generation2dGeneratePreviewWindowConfig { view.window::<Generation2dGeneratePreviewWindowConfigOwner>().cloned().unwrap_or_default() }
pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Generation2dGeneratePreviewWindowConfig { snapshot.filter(|snapshot| snapshot.window_kind_id() == super::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW).and_then(|snapshot| snapshot.get::<Generation2dGeneratePreviewWindowConfigOwner>()).cloned().unwrap_or_default() }
pub fn addressed(view: &semio_framework_plugin::ViewModel, base: &Generation2dGeneratePreviewWindowConfig, config: Generation2dGeneratePreviewWindowConfig) -> Result<Vec<semio_framework_plugin::WindowConfigMutation>, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("generation2d-generate-preview-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("generation2d-generate-preview-window-stale"))?;
    if kind != super::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW { return Err(semio_framework_plugin::Fault::from("generation2d-generate-preview-window-kind-required")); }
    Ok(Generation2dGeneratePreviewWindowConfigMutation::setting(base, &config).into_iter().map(|mutation| semio_framework_plugin::WindowConfigMutation::of::<Generation2dGeneratePreviewWindowConfigOwner>(id, mutation)).collect())
}

#[path="🚪️io/📝️text/🧬️mutations/🦀️.rs"]
mod field_set_text_io;

#[path="🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
mod field_set_binary_io;

#[path="🚪️io/📝️text/📸️snapshot/🦀️.rs"]
mod text_snapshot_io;

#[path="🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
mod binary_snapshot_io;
