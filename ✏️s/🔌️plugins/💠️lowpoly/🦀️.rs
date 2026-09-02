//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the lowpoly editor and viewer surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum LowpolyApps: PluginApp {
        LowpolyEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::lowpoly::LowpolyPlayApp>>),
        LowpolyViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::lowpoly::LowpolyViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `LowpolyPlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by the `.editor(…)` call below.
///
/// 🚧️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1: `.document_app::<X>(…)`
/// is deleted, not deprecated. `.editor::<E>(…)` registers the mutation-capable surface (the former
/// sole app), `.viewer::<V>(…)` the new genuinely read-only surface — see `👁️viewer/🦀️.rs`
/// for why it is not a thin wrapper around the editor.
pub fn plugin() -> Result<Plugin<LowpolyApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<LowpolyApps>::builder("lowpoly")
        .label("Lowpoly")
        .version("0.1.0")
        .artifact(crate::artifacts::lowpoly::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::lowpoly::LowpolyPlayApp>(crate::editor::lowpoly::create_lowpoly_app())
        .editor_mutation_roster::<crate::editor::lowpoly::LowpolyPlayApp>()
        .viewer::<crate::viewer::lowpoly::LowpolyViewer>(crate::viewer::lowpoly::create_lowpoly_viewer())
        .viewer_mutation_roster::<crate::viewer::lowpoly::LowpolyViewer>()
        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M5 — `.activation(…)`/`.execution(…)`/
        // `.requests(…)` (`📓️design-abi.md` §3/§6), same shape M0/M1 already landed.
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::lowpoly::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("documents.write".into()),
            scope: "plugin".into(),
            reason: "persist lowpoly mesh-edit/paint/UV editor edits to the open document".into(),
            optional: false,
        })
        .try_build()
}

#[cfg(test)]
mod surface_tests {
    /// 🧪️ Contract §2.5 (closed by W0-F, `📓️w0-f-report.md` Gap 2): the real framework testkit
    /// functions now exist — this packet uses them directly rather than the pilot's local stand-ins.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn lowpoly_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::lowpoly::LowpolyViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn lowpoly_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::lowpoly::LowpolyPlayApp, crate::viewer::lowpoly::LowpolyViewer>();
    }
}
