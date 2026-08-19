//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M3, `📓️design-abi.md`
/// §5/§6) are this crate's proof-of-migration: the host activates one instance whenever a
/// `"computation.flow"` artifact (`crate::artifacts::flow::artifact_kind().id`) is opened, this plugin's
/// own actor runs `Isolated` (its 9 `🧩️extensions/` run `Linked` instead — see each extension's own
/// `bundle()`), and it asks the broker for document write access to persist edits.
pub async fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("flow")
        .label("Flow")
        .version("0.1.0")
        .artifact(crate::artifacts::flow::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::flow::FlowPlayApp>(crate::editor::flow::create_flow_app())
        .editor_mutation_roster::<crate::editor::flow::FlowPlayApp>()
        .viewer::<crate::viewer::flow::FlowViewer>(crate::viewer::flow::create_flow_viewer())
        .viewer_mutation_roster::<crate::viewer::flow::FlowViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::flow::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist flow graph edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use crate::editor::flow::FlowPlayApp;
    use crate::viewer::flow::FlowViewer;

    /// 👁️ A viewer instance never mutates the document store, even when dispatched.
    #[test]
    async fn flow_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<FlowViewer>();
    }

    /// 🤝️ Editor and viewer surfaces agree on the artifact dialect they address.
    #[test]
    async fn flow_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<FlowPlayApp, FlowViewer>();
    }
}
//#endregion 🧪️SurfaceTests
