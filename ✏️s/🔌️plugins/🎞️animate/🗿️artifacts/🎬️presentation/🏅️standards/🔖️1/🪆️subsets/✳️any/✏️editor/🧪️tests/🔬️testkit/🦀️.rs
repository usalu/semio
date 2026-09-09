use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type PresentationApp = VcsArtifactApp<EditorApp<AnimatePresentationPlayApp>>;

/// ✏️ `AnimatePresentationPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<AnimatePresentationPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<E>`
/// builds it.
/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub async fn presentation_app() -> PresentationApp {
    new_app::<EditorApp<AnimatePresentationPlayApp>>().await
}

/// 🧪️ Adapts `create_animate_presentation_app`'s `AppDefinition` (contract §2.4) into the
/// `App { definition, examples }` shape `new_app_with_registry`/
/// `testkit::assert_declared_actions_bridge_to_commands` still expect — framework testkit gap, not
/// modifiable here (`🧰️framework/**` is outside this packet's lease).
fn animate_presentation_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_animate_presentation_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn presentation_app_with_registry() -> PresentationApp {
    new_app_with_registry::<EditorApp<AnimatePresentationPlayApp>>(animate_presentation_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut PresentationApp, command: PresentationCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut PresentationApp, body_key: &str) -> String {
    // 🌱️ `BuiltNode` deliberately has no `ToValue`/`FromValue` (framework `🦀️builder.rs`'s own
    // "DslValue-free exception" for `UiValue`-embedding types), so every caller here reads
    // rendered content back off the `Debug` rendering instead of round-tripping through JSON —
    // every call site below only substring-searches the result, never parses it as JSON.
    format!("{:?}", app.render(body_key, None, &ViewModel::default()).await.expect("render"))
}
