//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the declaration-owned mathematical surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum MathematicalApps: PluginApp {}
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. Atomic cutover (ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): `.declare_artifact(...)` (new declaration
/// tree) replaces `.artifact(...)`/`.editor::<>()`/`.viewer::<>()` outright — the old channel is
/// NOT kept alongside it (a second parallel registration channel is the compatibility layer this
/// ticket forbids). The old `.artifact(declaration())` channel's `.composers(...)` registered a
/// native composer entry with NO matching `composer` capability row in `definition()` — the exact
/// bug that shipped this plugin's WASM manifest as `assembly-failed` (every `try_build()` call
/// faulted on the capability-row mismatch); `.declare_artifact(...)`'s io hops are typed
/// `Serializer`/`Deserializer` entries validated by `io_register`, not `ComposerEntry` capability
/// rows, so this cutover fixes the manifest as a side effect, not a separate fix.
/// `.editor_mutation_roster()`/`.viewer_mutation_roster()` stay: they are an orthogonal,
/// still-supported opt-in (`contributor.list-artifact-mutations`) the new declaration tree's
/// `SurfaceDeclaration.mutation_roster` field does not yet wire live (`📓️w1-c-report.md`
/// openQuestion 3) — not a second registration of the artifact/schema/io itself.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M1, `📓️design-abi.md` §3/§6): the host
/// activates one instance whenever a `"computation.equation"` artifact
/// (`crate::artifacts::equation::artifact_kind().id`) is opened, this plugin's actor runs
/// `Isolated` (no cross-plugin extension attachment, the SDK default holds), and it asks the
/// broker for document write access because `EquationPlayApp` persists graph edits back to
/// the open document.
pub async fn plugin() -> Result<Plugin<MathematicalApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<MathematicalApps>::builder("mathematical")
        .label("Mathematical")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::equation::artifact())
        .editor_mutation_roster::<crate::editor::equation::EquationPlayApp>()
        .viewer_mutation_roster::<crate::viewer::equation::EquationViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::equation::artifact_kind().id })
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
    use crate::editor::equation::EquationPlayApp;
    use crate::viewer::equation::EquationViewer;

    #[semio_framework_async_macros::async_test]
    async fn equation_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<EquationViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn equation_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<EquationPlayApp, EquationViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn equation_viewer_instantiates_through_new_viewer() {
        let _app = semio_framework_plugin::testkit::new_viewer::<EquationViewer>();
    }
}
//#endregion 🧪️SurfaceTests
