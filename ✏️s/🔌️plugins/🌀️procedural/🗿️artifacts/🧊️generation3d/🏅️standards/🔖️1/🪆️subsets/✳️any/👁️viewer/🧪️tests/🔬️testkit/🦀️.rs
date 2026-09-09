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
    TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Generation3dViewerHarness = VcsArtifactApp<ViewerApp<Generation3dViewer>>;

pub fn generation3d_viewer_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_generation3d_viewer(), examples: Vec::new() }
}

pub async fn app() -> Generation3dViewerHarness {
    let mut app = new_app_with_registry::<ViewerApp<Generation3dViewer>>(generation3d_viewer_manifest_for_testkit).await;
    app.bind_instance_id(1).await;
    app
}

pub async fn dispatch(app: &mut Generation3dViewerHarness, command: Generation3dViewCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut Generation3dViewerHarness, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}
