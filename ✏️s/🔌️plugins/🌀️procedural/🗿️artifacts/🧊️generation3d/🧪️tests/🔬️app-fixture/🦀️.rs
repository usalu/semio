//! 🧪️ Neutral registered artifact fixtures own their document and publication lifetimes.
use crate::editor::generation3d::{create_generation3d_app, Generation3dCommand, Generation3dPlayApp};
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry, settle_registered_typed_operation, TypedOperationFixtureReceipt};
use semio_framework_plugin::{EditorApp, PluginApp, VcsArtifactApp, ViewModel};

pub(crate) type Generation3dApp = VcsArtifactApp<EditorApp<Generation3dPlayApp>>;
pub(crate) struct Generation3dAppFixture(Generation3dApp);
impl std::ops::Deref for Generation3dAppFixture {
    type Target = Generation3dApp;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::ops::DerefMut for Generation3dAppFixture {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
impl Drop for Generation3dAppFixture {
    fn drop(&mut self) { semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0); }
}

/// 🏗️ Supplies the artifact's registered tool catalog without installing geometry implementations.
pub(crate) async fn app_with_registry() -> Generation3dAppFixture {
    let mut app = new_app_with_registry::<EditorApp<Generation3dPlayApp>>(|| semio_framework_plugin::App { definition: create_generation3d_app(), examples: Vec::new() }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    app.bind_instance_id(1).await;
    Generation3dAppFixture(app)
}
pub(crate) async fn app() -> Generation3dAppFixture { app_with_registry().await }
pub(crate) fn snapshot(app: &Generation3dApp) -> Generation3dSnapshotRead { Generation3dSnapshotRead::new(app.snapshot().expect("snapshot")) }
/// 🎯️ Completes the registered typed publication before returning a live document read.
pub(crate) async fn dispatch(app: &mut Generation3dApp, command: Generation3dCommand) -> TypedOperationFixtureReceipt {
    let action_meta = meta("local");
    app.dispatch_typed(command, &action_meta).await.expect("dispatch");
    settle_registered_typed_operation(app, action_meta.instance_id).await.expect("retained publication")
}
pub(crate) async fn render_body(app: &mut Generation3dApp, body: &str) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render")).expect("render json")
}
