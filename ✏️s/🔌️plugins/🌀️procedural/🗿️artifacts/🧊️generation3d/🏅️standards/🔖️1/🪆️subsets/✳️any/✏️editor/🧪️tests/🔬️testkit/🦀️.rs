
use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// ✏️ `Generation3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<Generation3dPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<Generation3dPlayApp>` builds it.
pub type Generation3dApp = VcsArtifactApp<EditorApp<Generation3dPlayApp>>;

/// ✏️ Adapts `create_generation3d_app`'s `AppDefinition` (contract §2.4) into the
/// `App { definition, examples }` shape `testkit::assert_declared_actions_bridge_to_commands` /
/// `testkit::new_app_with_registry` still expect — framework testkit gap, not modifiable here
/// (`🧰️framework/**` is outside this packet's lease).
pub fn generation3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_generation3d_app(), examples: Vec::new() }
}

pub async fn app() -> Generation3dApp {
    new_app::<EditorApp<Generation3dPlayApp>>().await
}

pub async fn app_with_registry() -> Generation3dApp {
    new_app_with_registry::<EditorApp<Generation3dPlayApp>>(generation3d_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut Generation3dApp, command: Generation3dCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut Generation3dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}

/// 🧵️ A `flowEvalTick` chain self-dispatches via `requestedEffects`, which only the JS renderer
/// drains in production — a test has to do that draining itself.
pub async fn drain_flow_eval_ticks(app: &mut Generation3dApp) {
    app.pending_effects().await;
    for _ in 0..1000 {
        let result = app.dispatch_typed(Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}), &meta("local")).await.expect("flowEvalTick");
        if !result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")) {
            return;
        }
    }
    panic!("flowEvalTick chain did not converge within 1000 ticks");
}
