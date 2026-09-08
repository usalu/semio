
use super::*;
use semio_framework_plugin::app::EditorApp;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel, WindowMeasure};

pub type ShootingApp = VcsArtifactApp<EditorApp<ShootingPlayApp>>;

/// ✏️ `ShootingPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<ShootingPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<ShootingPlayApp>` builds it.
///
/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub async fn shooting_app() -> ShootingApp {
    new_app::<EditorApp<ShootingPlayApp>>().await
}

/// ✏️ Adapts `create_shooting_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `new_app_with_registry`/`assert_declared_actions_bridge_to_commands` still
/// expect — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this packet's
/// lease).
pub fn shooting_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_shooting_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn shooting_app_with_registry() -> ShootingApp {
    new_app_with_registry::<EditorApp<ShootingPlayApp>>(shooting_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut ShootingApp, command: ShootingCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut ShootingApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("project and retire semantic tree")
}

pub async fn world_scene(app: &mut ShootingApp) -> semio_framework_plugin::World3dScene {
    let tree = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewModel::default()).await.expect("render scene");
    let decoded = match &tree.root.component {
        semio_framework_plugin::Component::Surface(props) => semio_framework_ui_scene::decode(props),
        _ => panic!("3D surface"),
    };
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree).expect("retire scene tree");
    decoded.expect("packed 3D scene")
}

pub async fn icon_scene(app: &mut ShootingApp) -> semio_framework_plugin::IconRenderScene {
    let tree = app.render(SHOOTING_PLAY_BODY_ICON, None, &ViewModel::default()).await.expect("render icon");
    let decoded = match &tree.root.component {
        semio_framework_plugin::Component::Surface(props) => semio_framework_ui_scene::decode(props),
        _ => panic!("icon surface"),
    };
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree).expect("retire icon tree");
    decoded.expect("packed icon scene")
}

pub async fn scene_window_measures(app: &mut ShootingApp) -> Vec<WindowMeasure> {
    app.window_measures().await.get(SHOOTING_PLAY_WINDOW_SCENE).cloned().expect("scene window measures")
}

pub async fn icon_window_measures(app: &mut ShootingApp) -> Vec<WindowMeasure> {
    app.window_measures().await.get(SHOOTING_PLAY_WINDOW_ICON).cloned().expect("icon window measures")
}
