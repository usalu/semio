//! 🧪️ Neutral registered artifact fixtures own their document and publication lifetimes.
use semio_s_artifact_procedural_generation3d::manifest::create_generation3d_app;
use semio_s_artifact_procedural_3d_editor::Generation3dCommand;
use semio_s_artifact_procedural_3d_editor::Generation3dPlayApp;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshotRead;
use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry, settle_registered_typed_operation, TypedOperationFixtureReceipt};
use semio_framework_plugin::{EditorApp, PluginApp, VcsArtifactApp, ViewModel};

pub(crate) type Generation3dApp = VcsArtifactApp<EditorApp<Generation3dPlayApp>>;
/// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
    pub(crate) struct Generation3dAppFixture(pub Generation3dApp, pub semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);
impl std::ops::Deref for Generation3dAppFixture {
    type Target = Generation3dApp;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::ops::DerefMut for Generation3dAppFixture {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
impl Drop for Generation3dAppFixture {
    fn drop(&mut self) { semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close"); }
}

/// 🏗️ Supplies the artifact's registered tool catalog without installing geometry implementations.
pub(crate) async fn app_with_registry() -> Generation3dAppFixture {
    app_with_actor(semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await
}
/// 👤️ Binds the genuine opened fixture actor before any original store or window owner is constructed.
pub(crate) async fn app_with_actor(actor: semio_framework_os_kernel::ActorId) -> Generation3dAppFixture {
    let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = new_app_with_registry::<EditorApp<Generation3dPlayApp>>(|| semio_framework_plugin::App { definition: create_generation3d_app(), examples: Vec::new() }, actor, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(1).await;
    Generation3dAppFixture(app, scope)
}
pub(crate) async fn app() -> Generation3dAppFixture { app_with_registry().await }
pub(crate) fn snapshot(app: &Generation3dApp) -> Generation3dSnapshotRead { Generation3dSnapshotRead::new(app.snapshot().expect("snapshot")) }
/// 🎯️ Completes the registered typed publication before returning a live document read.
pub(crate) async fn dispatch(app: &mut Generation3dAppFixture, command: Generation3dCommand) -> TypedOperationFixtureReceipt {
    let action_meta = meta("local");
    app.0.dispatch_typed(command, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
    settle_registered_typed_operation(&mut app.0, action_meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("retained publication")
}
pub(crate) async fn render_body(app: &mut Generation3dAppFixture, body: &str) -> String {
    semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("render json")
}
