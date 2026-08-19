//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M4, `📓️design-abi.md`
/// §5/§6) are this crate's proof-of-migration: the host activates one instance whenever a
/// `"3d.process"` artifact (`crate::artifacts::process3d::artifact_kind().id`) is opened, this
/// plugin's own actor runs `Isolated` (its 4 `🧩️extensions/` — metal, robotic, concrete, wood — run
/// `Declarative` instead, see each extension's own `bundle()`), and it asks the broker for document
/// write access to persist edits.
pub async fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("process")
        .label("Process")
        .version("0.1.0")
        .artifact(crate::artifacts::process3d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::process3d::Process3dPlayApp>(crate::editor::process3d::create_process3d_app())
        .editor_mutation_roster::<crate::editor::process3d::Process3dPlayApp>()
        .viewer::<crate::viewer::process3d::Process3dViewer>(crate::viewer::process3d::create_process3d_viewer())
        .viewer_mutation_roster::<crate::viewer::process3d::Process3dViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::process3d::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist process3d machine-assignment edits to the open document".into(), optional: false })
        .try_build()
}
