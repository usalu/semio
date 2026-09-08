
use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// ✏️ `EquationPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<EquationPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<EquationPlayApp>` builds it.
pub type MathApp = VcsArtifactApp<EditorApp<EquationPlayApp>>;

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub async fn math_app() -> MathApp {
    new_app::<EditorApp<EquationPlayApp>>().await
}

/// ✏️ Adapts `create_equation_app`'s `AppDefinition` (contract §2.4) into the `App {
/// definition, examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still
/// expects — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
/// packet's lease).
pub fn equation_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_equation_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn math_app_with_registry() -> MathApp {
    new_app_with_registry::<EditorApp<EquationPlayApp>>(equation_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut MathApp, command: EquationCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut MathApp, body_key: &str) -> String {
    // 🌱️ `UiNode` (`semio-framework-plugin`, framework-owned) has not itself gained `ToValue` —
    // `Debug` gives every test caller here the same "does the render mention X" substring check.
    format!("{:?}", app.render(body_key, None, &ViewModel::default()).await.expect("render"))
}
