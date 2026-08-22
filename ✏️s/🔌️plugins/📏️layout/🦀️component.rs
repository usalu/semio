//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the layout editor and viewer surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum LayoutApps: PluginApp {
        Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::layout::LayoutPlayApp>>),
        Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::layout::LayoutViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `LayoutPlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by `.editor(…)` below. `.editor(…)` +
/// `.viewer(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the old single
/// `.document_app(…)` registration — the subset's mutation-capable and read-only surfaces are now two
/// independently addressable apps sharing one `LAYOUT_DIALECT` coordinate.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M1, `📓️design-abi.md` §3/§6): the host
/// activates one instance whenever a `"2d.layout"` artifact
/// (`crate::artifacts::layout::artifact_kind().id`) is opened, this plugin's actor runs `Isolated`
/// (no cross-plugin extension attachment, no self-tick/`pending_effects` loop found by grep — the
/// SDK default holds), and it asks the broker for document write access because `LayoutPlayApp`
/// persists edits back to the open document. No quota declared: layout's ~20 `Effect` call sites
/// (`DispatchAction`/`DownloadMediaExport`) are per-turn UI/export effects with no evidence of
/// long-running computation, large held buffers, or high-frequency timers.
pub fn plugin() -> Result<Plugin<LayoutApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<LayoutApps>::builder("layout")
        .label("Layout")
        .version("0.1.0")
        .artifact(crate::artifacts::layout::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::layout::LayoutPlayApp>(crate::editor::layout::create_layout_app())
        .editor_mutation_roster::<crate::editor::layout::LayoutPlayApp>()
        .viewer::<crate::viewer::layout::LayoutViewer>(crate::viewer::layout::create_layout_viewer())
        .viewer_mutation_roster::<crate::viewer::layout::LayoutViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::layout::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist layout edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 canonical helpers
    //! (`semio_framework_plugin::testkit::{assert_viewer_never_mutates,
    //! assert_editor_and_viewer_share_dialect, new_viewer}`) — closed by lane 0-F (`📓️w0-f-report.md`
    //! Gap 2), used directly here rather than local stand-ins.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn layout_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::layout::LayoutViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn layout_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::layout::LayoutPlayApp, crate::viewer::layout::LayoutViewer>();
    }
}
//#endregion 🧪️SurfaceTests
