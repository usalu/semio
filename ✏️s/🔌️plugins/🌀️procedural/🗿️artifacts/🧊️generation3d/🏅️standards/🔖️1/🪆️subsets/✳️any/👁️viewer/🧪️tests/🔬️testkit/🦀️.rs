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
