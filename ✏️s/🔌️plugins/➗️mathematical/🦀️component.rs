//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)` escape
/// hatch; `.setup()` itself is gone (W1c) — `MathematicalPlayApp::app_schema()` now answers the one
/// thing it used to survive for, registered automatically by `register_document_app` below.
///
/// `.document_app::<…>(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1)
/// is replaced by two independent surfaces: `.editor::<…>(…)` (mutation-capable) and
/// `.viewer::<…>(…)` (read-only) for the same `s.mathematical.mathematical@1/*` dialect.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M1, `📓️design-abi.md` §3/§6): the host
/// activates one instance whenever a `"computation.mathematical"` artifact
/// (`crate::artifacts::mathematical::artifact_kind().id`) is opened, this plugin's actor runs
/// `Isolated` (no cross-plugin extension attachment, the SDK default holds), and it asks the
/// broker for document write access because `MathematicalPlayApp` persists graph edits back to
/// the open document.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("mathematical")
        .label("Mathematical")
        .version("0.1.0")
        .artifact(crate::artifacts::mathematical::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::mathematical::MathematicalPlayApp>(crate::editor::mathematical::create_mathematical_app())
        .editor_mutation_roster::<crate::editor::mathematical::MathematicalPlayApp>()
        .viewer::<crate::viewer::mathematical::MathematicalViewer>(crate::viewer::mathematical::create_mathematical_viewer())
        .viewer_mutation_roster::<crate::viewer::mathematical::MathematicalViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::mathematical::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist mathematical graph edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 🧪️ The editor/viewer pair's own cross-surface guarantees (contract §2.5), using the landed
    //! framework testkit directly: `semio_framework_plugin::testkit::{assert_viewer_never_mutates,
    //! assert_editor_and_viewer_share_dialect, new_viewer}`.
    use crate::editor::mathematical::MathematicalPlayApp;
    use crate::viewer::mathematical::MathematicalViewer;

    #[test]
    fn mathematical_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<MathematicalViewer>();
    }

    #[test]
    fn mathematical_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<MathematicalPlayApp, MathematicalViewer>();
    }

    #[test]
    fn mathematical_viewer_instantiates_through_new_viewer() {
        let _app = semio_framework_plugin::testkit::new_viewer::<MathematicalViewer>();
    }
}
//#endregion 🧪️SurfaceTests
