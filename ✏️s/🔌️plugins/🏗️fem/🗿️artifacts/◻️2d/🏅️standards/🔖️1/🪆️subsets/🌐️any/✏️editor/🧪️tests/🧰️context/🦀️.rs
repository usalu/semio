use semio_s_artifact_fem_2d_editor::editor::fem2d::create_fem2d_app;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model as model_window;
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results as results_window;
use semio_framework_value::DslValue;
use semio_plugin::artifact_app_laws::{
    OriginalFixtureScope, close_registered_fixture_app, dispatch_fixture_action,
    dispatch_original_fixture_command, fixture_identity, fixture_mounted_policy, load_document, meta,
    new_app_with_registry_boxed, render_registered_fixture_app, settle_history_verb,
    settle_registered_typed_operation,
};
use semio_plugin::App;
use semio_plugin::EditorApp;
use semio_plugin::Effect;
use semio_plugin::InvocationResult;
use semio_plugin::PluginApp;
use semio_plugin::VcsArtifactApp;
use semio_plugin::ViewModel;
use semio_plugin::ViewWindowInstance;
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};

/// 🧪️ The registered, MOUNTED fixture app every editor test drives — bound to
/// [`FEM2D_TEST_INSTANCE`] so typed commands reach their retained routes, and retired through the
/// framework's exact close loop when it goes out of scope (the store's `Drop` demands the
/// terminal-empty witness; a test that already called [`close`] drops a closed shell, a no-op).
/// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
pub struct Fem2dApp(pub VcsArtifactApp<EditorApp<Fem2dPlayApp>>, pub OriginalFixtureScope);

impl std::ops::Deref for Fem2dApp {
    type Target = VcsArtifactApp<EditorApp<Fem2dPlayApp>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Fem2dApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Fem2dApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            close(self);
        }
    }
}

/// 📱️ The addressed fixture window for a registered action or render.
fn view(kind: &str) -> ViewModel {
    let id = if kind == model_window::WINDOW_KIND_ID { "model-left" } else { "results-left" };
    ViewModel { window_id: Some(id.into()), window_instances: vec![ViewWindowInstance { id: id.into(), window_kind_id: kind.into() }], ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) }
}

pub(super) fn manifest() -> App { App { definition: create_fem2d_app(), examples: Vec::new() } }

semio_plugin::history_edit_acceptance_law!("fem", Fem2dPlayApp, manifest, "../../../../../../..");

pub fn fem2d_app() -> Fem2dApp {
    let scope = OriginalFixtureScope::admit();
    let mut identity = fixture_identity();
    let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry_boxed::<EditorApp<Fem2dPlayApp>>(
        manifest, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), fixture_mounted_policy(), &mut identity,
    ));
    ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(FEM2D_TEST_INSTANCE));
    Fem2dApp(*app, scope)
}

/// 🪪️ The instance id every `meta("local")` dispatch is stamped with.
pub const FEM2D_TEST_INSTANCE: u32 = 1;

/// 🧮️ A MOUNTED app: bound to [`FEM2D_TEST_INSTANCE`], so a typed command reaches its retained
/// route instead of being refused with `interactive-job.live-instance`, and its publication
/// actually settles into the stores.
pub fn fem2d_mounted_app() -> Fem2dApp {
    fem2d_app()
}

/// 🫙️ A mounted app on the EMPTY document — the app boots on the bundled demo, so a test that
/// counts what it adds itself starts from nothing instead (any example id but the demo's loads
/// the empty snapshot).
pub fn fem2d_empty_app() -> Fem2dApp {
    let mut app = fem2d_app();
    let mut action = meta("local");
    action.view_state = Some(view(model_window::WINDOW_KIND_ID).into());
    let args = DslValue::object([("exampleId".to_owned(), DslValue::String("empty".into()))]);
    let invoked = ::semio_framework_async::poll::resolve_ready(dispatch_fixture_action(
        &mut app.0, "setActiveExample", Some(args), &action, &mut fixture_identity(), app.1.cx(), fixture_mounted_policy().maintenance,
    )).expect("dispatch the empty example");
    ::semio_framework_async::poll::resolve_ready(finish_dispatch(&mut app, invoked));
    assert!(app.snapshot_ref().nodes.is_empty(), "the empty example must load an empty document");
    app
}

/// 🧹️ Retires a [`fem2d_mounted_app`] the way the plugin host retires a closing instance.
pub fn close(app: &mut Fem2dApp) {
    if !app.0.close_terminal_is_empty() {
        close_registered_fixture_app(&mut app.0, fixture_mounted_policy(), app.1.cx()).expect("original caller paid app close");
    }
}

pub async fn dispatch(app: &mut Fem2dApp, command: Fem2dCommand) -> InvocationResult {
    let kind = match &command {
        Fem2dCommand::SetResultDisplay(_) | Fem2dCommand::SetResultAnimation(_) | Fem2dCommand::ResultAnimationTick(_) => results_window::WINDOW_KIND_ID,
        _ => model_window::WINDOW_KIND_ID,
    };
    let mut action = meta("local");
    action.view_state = Some(view(kind).into());
    let invoked = dispatch_original_fixture_command(&mut app.0, &command, &action, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), app.1.cx()).await.expect("dispatch");
    finish_dispatch(app, invoked).await
}

async fn finish_dispatch(app: &mut Fem2dApp, mut invoked: InvocationResult) -> InvocationResult {
    let mut identity = fixture_identity();
    let settled = settle_registered_typed_operation(&mut app.0, FEM2D_TEST_INSTANCE, fixture_mounted_policy(), &mut identity, app.1.cx()).await.expect("settle the typed operation");
    invoked.requested_effects.extend(settled.effects);
    for effect in &invoked.requested_effects {
        if let Effect::LoadDocument { pack, spr } = effect {
            let files = store::os_store::serialization::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
            load_document(&mut app.0, &files, &mut identity, app.1.cx()).await.expect("test host applies load-document effect");
        }
    }
    invoked
}

/// 📨️ Publishes a staged shell action from the model window using its declared argument names.
pub async fn dispatch_editor_action(app: &mut Fem2dApp, action_id: &str, args: DslValue) -> InvocationResult {
    let mut action = meta("local");
    action.view_state = Some(view(model_window::WINDOW_KIND_ID).into());
    let invoked = dispatch_fixture_action(
        &mut app.0, action_id, Some(args), &action, &mut fixture_identity(), app.1.cx(), fixture_mounted_policy().maintenance,
    ).await.expect("dispatch the staged editor action");
    finish_dispatch(app, invoked).await
}

/// 🧾️ Dispatches `command` from the model window like [`dispatch`] and drives its retained publication home,
/// answering the applied history rows with document ops its publications upserted — how a law reads the one row a
/// gesture leaves.
pub async fn dispatch_rows(app: &mut Fem2dApp, command: Fem2dCommand) -> Vec<semio_framework::kernel::HistoryEntry> {
    let mut action = meta("local");
    action.view_state = Some(view(model_window::WINDOW_KIND_ID).into());
    let mut identity = fixture_identity();
    dispatch_original_fixture_command(&mut app.0, &command, &action, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut identity, app.1.cx()).await.expect("dispatch");
    let started = Instant::now();
    let settled = settle_registered_typed_operation(&mut app.0, FEM2D_TEST_INSTANCE, fixture_mounted_policy(), &mut identity, app.1.cx()).await.expect("settle the typed operation");
    assert!(started.elapsed() < Duration::from_secs(30), "the typed operation settles within 30 seconds");
    settled.history_patches.into_iter().flat_map(|patch| patch.upserts).filter(|row| row.applied && !row.op_lines.is_empty()).collect()
}

/// ↩️ Runs a framework-reserved history verb (`undo`/`redo`) through a boxed settlement future.
pub fn history_verb<'a>(app: &'a mut Fem2dApp, verb: &'a str) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    Box::pin(async move {
        let mut identity = fixture_identity();
        settle_history_verb(&mut app.0, verb, FEM2D_TEST_INSTANCE, fixture_mounted_policy(), &mut identity, app.1.cx()).await;
    })
}

pub fn render(app: &mut Fem2dApp, body_key: &str) -> String {
    let kind = if body_key == model_window::BODY_KEY { model_window::WINDOW_KIND_ID } else { results_window::WINDOW_KIND_ID };
    ::semio_framework_async::poll::resolve_ready(render_registered_fixture_app(&mut app.0, body_key, &view(kind), app.1.cx())).expect("fixture projection")
}
