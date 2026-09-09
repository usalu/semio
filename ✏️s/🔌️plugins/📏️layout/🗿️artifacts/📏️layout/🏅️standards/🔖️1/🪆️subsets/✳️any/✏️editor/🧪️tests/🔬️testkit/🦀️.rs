use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type LayoutApp = VcsArtifactApp<EditorApp<LayoutPlayApp>>;

/// ✏️ `LayoutPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<LayoutPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<LayoutPlayApp>` builds it.

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub async fn layout_app() -> LayoutApp {
    new_app::<EditorApp<LayoutPlayApp>>().await
}

/// 🧪️ Adapts `create_layout_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `new_app_with_registry` still expects — framework testkit gap, not
/// modifiable here (`🧰️framework/**` is outside this packet's lease).
fn layout_app_manifest_for_testkit() -> App {
    App { definition: create_layout_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn layout_app_with_registry() -> LayoutApp {
    new_app_with_registry::<EditorApp<LayoutPlayApp>>(layout_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut LayoutApp, command: LayoutCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut LayoutApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("fixture projection")
}

pub fn test_screen_point(camera_x: f64, camera_y: f64, zoom: f64, width: f64, height: f64, world_x: f64, world_y: f64) -> (f64, f64) {
    let camera = infinite_canvas::camera::Camera { x: camera_x, y: camera_y, zoom };
    let viewport = infinite_canvas::camera::Viewport { width: width as u32, height: height as u32, dpr: 1.0 };
    let screen = infinite_canvas::camera::world_to_screen(&camera, &viewport, infinite_canvas::Point::new(world_x, world_y));
    (screen.x, screen.y)
}
