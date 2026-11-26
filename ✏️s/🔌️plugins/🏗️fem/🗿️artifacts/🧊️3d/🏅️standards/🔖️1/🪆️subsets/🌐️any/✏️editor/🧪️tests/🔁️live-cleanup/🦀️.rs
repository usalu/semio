//! 🔁️ Live cleanup target: the mounted fem3d editor inside the production runtime, so the zero-progress credit of its live maintenance pump is exercised on its own.
#![recursion_limit = "256"]

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as vcs;

use semio_plugin::artifact_app_laws::{fixture_identity, fixture_mounted_policy, meta, new_app_with_registry, settle_registered_typed_operation, FixtureUiArena, LiveCleanupFixture, OriginalFixtureScope};
use semio_plugin::{EditorApp, PluginApp as _, ViewModel, ViewWindowInstance};
use semio_s_artifact_fem_3d_editor::editor::fem3d::{Fem3dCommand, Fem3dPlayApp};
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_node::AddNode;
use semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::modes::edit::windows::model::FEM3D_WINDOW_MODEL;

const INSTANCE: u32 = 1;
const TURNS: usize = 400;

#[path = "../🚀️initialized-manifest/🦀️.rs"]
mod initialized_manifest;
use initialized_manifest::manifest;

fn view() -> ViewModel {
    ViewModel { window_id: Some("model-left".into()), window_instances: vec![ViewWindowInstance { id: "model-left".into(), window_kind_id: FEM3D_WINDOW_MODEL.into() }], ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) }
}

/// ⚖️ LAW: the live maintenance pump of a freshly mounted fem3d editor, and of the same editor after one committed edit, never exhausts its zero-progress credit over hundreds of host turns.
#[semio_framework_async_macros::async_test]
async fn fem3d_live_cleanup_never_reaches_the_zero_progress_verdict() {
    let _arena = FixtureUiArena::admit();
    let mut scope = OriginalFixtureScope::admit();
    let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry::<EditorApp<Fem3dPlayApp>>(manifest, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), fixture_mounted_policy(), &mut fixture_identity()));
    ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(INSTANCE));
    let fixture = LiveCleanupFixture::admit(app, INSTANCE);
    let booted = fixture.pump(TURNS, scope.cx()).unwrap_or_else(|refusal| panic!("live cleanup faulted after boot: {refusal}"));
    assert!(fixture.app().app.maintenance_terminal_is_empty(), "the booted editor must settle every live maintenance frontier: {booted:?}");
    {
        let mut guard = fixture.app();
        let mut action = meta(semio_os_kernel::LOCAL_ACTOR_ID);
        action.view_state = Some(std::sync::Arc::new(view()));
        semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut guard.app, &Fem3dCommand::AddNode(AddNode { x: 1.0, y: 2.0, z: 3.0 }), &action, fixture_mounted_policy(), &mut fixture_identity(), scope.cx()).await.expect("dispatch the edit");
        settle_registered_typed_operation(&mut guard.app, INSTANCE, fixture_mounted_policy(), &mut fixture_identity(), scope.cx()).await.expect("settle the edit");
    }
    let edited = fixture.pump(TURNS, scope.cx()).unwrap_or_else(|refusal| panic!("live cleanup faulted after the edit: {refusal}"));
    assert_eq!((booted.final_stall, edited.final_stall), (0, 0), "an idle editor must leave the zero-progress counter at rest: {booted:?} {edited:?}");
    assert!(fixture.app().app.maintenance_terminal_is_empty(), "the idle editor must settle every live maintenance frontier");
    fixture.retire(scope.cx()).await;
}
