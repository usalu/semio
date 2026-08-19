//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `LowpolyPlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by the `.editor(…)` call below.
///
/// 🚧️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1: `.document_app::<X>(…)`
/// is deleted, not deprecated. `.editor::<E>(…)` registers the mutation-capable surface (the former
/// sole app), `.viewer::<V>(…)` the new genuinely read-only surface — see `👁️viewer/🦀️component.rs`
/// for why it is not a thin wrapper around the editor.
pub async fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("lowpoly")
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
