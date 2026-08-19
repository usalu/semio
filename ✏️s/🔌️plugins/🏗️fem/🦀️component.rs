//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(register_all_engines)`
/// escape hatch for both artifacts; `.setup()` itself is gone (W1c). Ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET dissolved `apps::fem2d`/`apps::fem3d` into
/// `editor::fem2d`/`editor::fem3d` (each still `Fem2dPlayApp`/`Fem3dPlayApp: ArtifactEditor`) plus new
/// `viewer::fem2d`/`viewer::fem3d` (`Fem2dViewer`/`Fem3dViewer: ArtifactViewer`) — every subset now
/// registers one editor and one viewer surface instead of one document app. `.activation(…)`/
/// `.execution(…)`/`.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
/// M6-remaining, `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s and
/// `🗄️stdio`'s own shape: one `OnArtifactKind` event per owned kind, read live from each subset's own
/// `computation_artifact_kind().id` (never hardcoded), `Isolated` execution (nothing here justifies a
/// publisher-trusted mode), and one `documents.write` ask covering both editors' persisted mutations.
pub async fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("fem")
        .label("FEM")
        .version("0.1.0")
        .artifact(crate::artifacts::fem2d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::fem3d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::fem2d::Fem2dPlayApp>(crate::editor::fem2d::create_fem2d_app())
        .editor_mutation_roster::<crate::editor::fem2d::Fem2dPlayApp>()
        .viewer::<crate::viewer::fem2d::Fem2dViewer>(crate::viewer::fem2d::create_fem2d_viewer())
        .viewer_mutation_roster::<crate::viewer::fem2d::Fem2dViewer>()
        .editor::<crate::editor::fem3d::Fem3dPlayApp>(crate::editor::fem3d::create_fem3d_app())
        .editor_mutation_roster::<crate::editor::fem3d::Fem3dPlayApp>()
        .viewer::<crate::viewer::fem3d::Fem3dViewer>(crate::viewer::fem3d::create_fem3d_viewer())
        .viewer_mutation_roster::<crate::viewer::fem3d::Fem3dViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::fem2d::computation_artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::fem3d::computation_artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist fem2d/fem3d edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
/// 🧪️ Contract §2.3/§2.5: each artifact's editor and viewer share one `Dialect`, and the viewer can
/// never mutate the document store. Uses the real framework testkit helpers (w0-f gap 2 closure), not
/// local stand-ins.
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn fem2d_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::fem2d::Fem2dViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn fem2d_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::fem2d::Fem2dPlayApp, crate::viewer::fem2d::Fem2dViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::fem3d::Fem3dViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::fem3d::Fem3dPlayApp, crate::viewer::fem3d::Fem3dViewer>();
    }
}
//#endregion 🧪️SurfaceTests
