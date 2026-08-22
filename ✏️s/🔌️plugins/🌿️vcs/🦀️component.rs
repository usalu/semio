//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the VCS editor and viewer surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum VcsApps: PluginApp {
        Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::vcs::VcsPlayApp>>),
        Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::vcs::VcsViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. Atomic cutover (ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): `.declare_artifact(...)` (new declaration
/// tree) replaces `.artifact(...)`/`.editor::<>()`/`.viewer::<>()` outright — the old channel is NOT
/// kept alongside it (a second parallel registration channel is the compatibility layer this ticket
/// forbids). `.editor_mutation_roster()`/`.viewer_mutation_roster()` stay: they are an orthogonal,
/// still-supported opt-in (`contributor.list-artifact-mutations`) the new declaration tree's
/// `SurfaceDeclaration.mutation_roster` does not yet wire live (`📓️w1-c-report.md` openQuestion 3)
/// — not a second registration of the artifact/schema/io itself. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining,
/// `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s shape.
pub fn plugin() -> Result<Plugin<VcsApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<VcsApps>::builder("vcs")
        .label("VCS")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::vcs::artifact())
        .editor_mutation_roster::<crate::editor::vcs::VcsPlayApp>()
        .viewer_mutation_roster::<crate::viewer::vcs::VcsViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::vcs::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist vcs edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 —
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
    //! now exist for real (landed by lane 0-F, see `📓️w0-f-report.md`), so this uses them directly
    //! rather than writing local stand-ins.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn vcs_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::vcs::VcsViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn vcs_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::vcs::VcsPlayApp, crate::viewer::vcs::VcsViewer>();
    }
}
//#endregion 🧪️SurfaceTests
