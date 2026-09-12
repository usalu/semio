//! 👁️ Viewer test harness — the read-only twin of the sibling surface's own testkit.
//!
//! `ViewerApp<Generation3dViewer>` is the real `ArtifactApp` implementor `VcsArtifactApp` wraps,
//! exactly the way `PluginBuilder::viewer::<Generation3dViewer>` builds it, so every assertion below
//! runs against the production adapter rather than a hand-rolled stand-in.

use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use std::sync::{Mutex, MutexGuard};

/// 🧵️ `tessellate_geometry` and the flow-eval neuron kernel cache behind it are process-wide, so
/// every viewer test that evaluates a flow fixture or tessellates BRep geometry — directly, or
/// indirectly through the Preview window's `render()` — acquires this lock first.
static TEST_SERIAL: Mutex<()> = Mutex::new(());

pub fn lock() -> MutexGuard<'static, ()> {
    crate::flow_operators::installed();
    TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Generation3dViewerHarness = VcsArtifactApp<ViewerApp<Generation3dViewer>>;

pub fn generation3d_viewer_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_generation3d_viewer(), examples: Vec::new() }
}

/// 🧹️ A live viewer fixture that CLOSES itself — the read-only twin of the editor testkit's
/// `Generation3dAppFixture`, and for the same reason: `VcsArtifactApp`'s `ArtifactStore` owns a
/// disposer whose `Drop` asserts terminal-empty ownership, so a plainly-dropped harness aborts the
/// whole test binary (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub struct Generation3dViewerFixture(Generation3dViewerHarness);

impl std::ops::Deref for Generation3dViewerFixture {
    type Target = Generation3dViewerHarness;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Generation3dViewerFixture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Generation3dViewerFixture {
    fn drop(&mut self) {
        for _ in 0..1_000_000 {
            if self.0.close_terminal_is_empty() {
                return;
            }
            if self.0.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).is_err() {
                break;
            }
        }
        assert!(std::thread::panicking() || self.0.close_terminal_is_empty(), "Generation3d viewer fixture did not reach its terminal-empty close witness");
    }
}

pub async fn app() -> Generation3dViewerFixture {
    let mut app = new_app_with_registry::<ViewerApp<Generation3dViewer>>(generation3d_viewer_manifest_for_testkit).await;
    app.bind_instance_id(1).await;
    Generation3dViewerFixture(app)
}

/// 📸️ Reads the live projection into a self-retiring [`Generation3dSnapshotRead`] — the read-only
/// twin of the editor testkit's own, and for the same reason (an owned `Generation3dSnapshot`
/// aborts the binary on a bare drop).
pub fn snapshot(app: &Generation3dViewerHarness) -> crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead {
    crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(app.snapshot().expect("snapshot"))
}

pub async fn dispatch(app: &mut Generation3dViewerHarness, command: Generation3dViewCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut Generation3dViewerHarness, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}

//#region ⏱️FlowEvalChain
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::testkit::{settle_registered_typed_operation, TypedOperationFixtureReceipt};
use semio_framework_plugin::{ActionMeta, Effect, ViewWindowInstance};

/// 🏛️ The SHELL's own roster for the read-only surface. This viewer mounts exactly ONE window, so
/// the preview window is also the current one — which is precisely why a window-scoped route here
/// must still find its own window on the roster rather than assume it: the addressing law is the
/// same law the sibling surface runs with a flow window in front of it.
pub fn view_shell_view(preview: &str) -> ViewModel {
    let roster = vec![ViewWindowInstance { id: preview.into(), window_kind_id: preview::WINDOW_KIND_ID.into() }];
    ViewModel { window_instances: roster, ..Default::default() }.for_window_instance(preview).expect("viewer preview window instance")
}

/// 🏛️ Redispatches one armed `Effect::DispatchAction` exactly the way `makeEffectDispatchOne`
/// (`🛠️ShellHelpers/🟦️.tsx`) does: an action id the app declares as a COMMAND re-enters the typed
/// command channel with the shell's own live view attached, never the scoped action channel.
pub async fn dispatch_effect_command(app: &mut Generation3dViewerHarness, command_id: &str, args: Option<&dsl::DslValue>, action_meta: &ActionMeta) -> Result<(), semio_framework_plugin::Fault> {
    use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
    let arguments = match args {
        Some(dsl::DslValue::Object(entries)) => entries.iter().cloned().collect(),
        _ => std::collections::BTreeMap::new(),
    };
    let app_id = app.app_id().await.to_string();
    let invocation = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: String::new(), app_id }, command_id: command_id.to_string() }, arguments };
    app.handle_command(&invocation, None, action_meta).await.map(|_| ())
}

pub async fn dispatch_with_view(app: &mut Generation3dViewerHarness, command: Generation3dViewCommand, view_state: ViewModel) -> Result<TypedOperationFixtureReceipt, semio_framework_plugin::Fault> {
    let action_meta = ActionMeta { view_state: Some(view_state), ..meta("local") };
    app.dispatch_typed(command, &action_meta).await?;
    settle_registered_typed_operation(app, action_meta.instance_id).await
}

/// 🎯️ Every `flowEvalTick` an effect list armed, as the `args` object the shell would redispatch.
pub fn armed_ticks(effects: &[Effect]) -> Vec<Option<dsl::DslValue>> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == "flowEvalTick" => Some(args.clone()),
            _ => None,
        })
        .collect()
}

pub fn armed_window_ids(effects: &[Effect]) -> Vec<String> {
    armed_ticks(effects).into_iter().map(|args| args.as_ref().and_then(|args| args.get("windowId")).and_then(dsl::DslValue::as_str).unwrap_or_default().to_string()).collect()
}

fn arm(armed: &mut Vec<Option<dsl::DslValue>>, more: Vec<Option<dsl::DslValue>>) {
    for entry in more {
        if !armed.contains(&entry) {
            armed.push(entry);
        }
    }
}

/// 🔁️ The REAL served chain, end to end: `pending_effects` arms the first tick off the host's
/// attached-window roster, and every following tick is the `action`+`args` of an
/// `Effect::DispatchAction` the app itself emitted, replayed through `PluginApp::handle_command`
/// the way `ShellHost` feeds `requestedEffects` back — with the in-process `brep` extension
/// answering every `Effect::InvokeExtension` the way the shell does in production.
pub async fn drain_armed_flow_eval_ticks(app: &mut Generation3dViewerHarness, shell_view: &ViewModel) -> usize {
    let armed = app.pending_effects(Some(shell_view)).await;
    drain_armed_flow_eval_ticks_from(app, shell_view, &armed).await
}

pub async fn drain_armed_flow_eval_ticks_from(app: &mut Generation3dViewerHarness, shell_view: &ViewModel, initial: &[Effect]) -> usize {
    let action_meta = ActionMeta { view_state: Some(shell_view.clone()), ..meta("local") };
    let mut armed = armed_ticks(initial);
    let mut ticks = 0;
    for _ in 0..1000 {
        let Some(args) = armed.pop() else { return ticks };
        dispatch_effect_command(app, "flowEvalTick", args.as_ref(), &action_meta).await.expect("the shell redispatches an armed flowEvalTick effect");
        let receipt = settle_registered_typed_operation(app, action_meta.instance_id).await.expect("retained publication");
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "an armed flowEvalTick faulted in the retained job ladder: args={args:?}");
        ticks += 1;
        arm(&mut armed, armed_ticks(&receipt.effects));
        arm(&mut armed, armed_ticks(&crate::brep_extension::settle_with_meta(app, action_meta.instance_id, &action_meta).await.effects));
    }
    panic!("the armed flowEvalTick chain did not converge within 1000 dispatches");
}

pub async fn render_with_view(app: &mut Generation3dViewerHarness, body_key: &str, view_state: &ViewModel) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")
}

/// 🧊️ The mesh count a rendered viewer preview body actually carries.
pub fn preview_mesh_count(projection: &str) -> usize {
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::testkit::decode_fixture_scene_with_lanes(projection).expect("the viewer preview body must decode as an assembled world-3d scene");
    let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("preview meshes json");
    meshes.as_array().map_or(0, Vec::len)
}
//#endregion ⏱️FlowEvalChain
