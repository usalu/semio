//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.editor()`/`.viewer()` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the old single `.document_app(…)` call —
/// `ShootingPlayApp::app_schema()` still registers its own CONFIG/PRESENCE schema automatically.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining, `📓️design-abi.md` §3/§6) are this
/// crate's migration proof, mirroring `🗒️note`'s shape.
pub async fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("shooting")
        .label("Shooting")
        .version("0.1.0")
        .artifact(crate::artifacts::shooting::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::shooting::ShootingPlayApp>(crate::editor::shooting::create_shooting_app())
        .editor_mutation_roster::<crate::editor::shooting::ShootingPlayApp>()
        .viewer::<crate::viewer::shooting::ShootingViewer>(crate::viewer::shooting::create_shooting_viewer())
        .viewer_mutation_roster::<crate::viewer::shooting::ShootingViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::shooting::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist shooting edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 promises
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
    //! new_viewer}` — landed for real as of this packet (unlike the cad pilot, which had to write local
    //! stand-ins), so these call the canonical framework versions directly.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn shooting_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::shooting::ShootingViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn shooting_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::shooting::ShootingPlayApp, crate::viewer::shooting::ShootingViewer>();
    }
}
//#endregion 🧪️SurfaceTests
