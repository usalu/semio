//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{EditorApp, ExecutionMode, Plugin, PluginApp, PluginAssemblyError, VcsArtifactApp, ViewerApp};

//#region 🗃️Apps
// 🗃️ Closed runtime app fleet for the declaration-owned animate surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum AnimateApps: PluginApp {
        PresentEditor(VcsArtifactApp<EditorApp<crate::editor::animate::AnimatePresentPlayApp>>),
        PresentViewer(VcsArtifactApp<ViewerApp<crate::viewer::animate::AnimatePresentViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining,
/// `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s shape.
pub fn plugin() -> Result<Plugin<AnimateApps>, PluginAssemblyError> {
    Plugin::<AnimateApps>::builder("animate")
        .label("Animate")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::present::artifact::<AnimateApps>())
        .editor_mutation_roster::<crate::editor::animate::AnimatePresentPlayApp>()
        .viewer_mutation_roster::<crate::viewer::animate::AnimatePresentViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::present::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist animate present edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — SDK gap now closed
    //! (`📓️w0-f-report.md`): `semio_framework_plugin::testkit::{assert_viewer_never_mutates,
    //! assert_editor_and_viewer_share_dialect}` are real, exercised directly here.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn animate_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::animate::AnimatePresentViewer>().await;
    }

    #[semio_framework_async_macros::async_test]
    async fn animate_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::animate::AnimatePresentPlayApp, crate::viewer::animate::AnimatePresentViewer>().await;
    }
}
//#endregion 🧪️SurfaceTests
