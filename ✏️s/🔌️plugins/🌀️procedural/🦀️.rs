//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, FlowExtensionDeclaration, FlowExtensionExecutableIdentity, FlowExtensionManifest, HostMediaHandlerDeclaration, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the procedural 2D and 3D surfaces.
    pub enum ProceduralApps: PluginApp {
        Generation2dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_procedural_generation2d::editor::generation2d::Generation2dPlayApp>>),
        Generation2dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_procedural_generation2d::viewer::generation2d::Generation2dViewer>>),
        Generation3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dPlayApp>>),
        Generation3dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_procedural_generation3d::viewer::generation3d::Generation3dViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin<ProceduralApps>, PluginAssemblyError> {
    semio_s_artifact_procedural_assembly::standards::v1::subsets::any::schema::inferences::register_assembly_inference_factory(&ActionBus::production())
        .map_err(|error| PluginAssemblyError::new("assembly-inference-factory", error.to_string()))?;
    Plugin::<ProceduralApps>::builder("procedural")
        .label("Procedural")
        .version("0.1.0")
        .package_id("semio:procedural")
        .routed_inference(semio_s_artifact_procedural_assembly::standards::v1::subsets::any::schema::inferences::assembly_inference_metadata())
        .artifact(semio_s_artifact_procedural_generation2d::declaration().map_err(PluginAssemblyError::definition)?)
        .artifact(semio_s_artifact_procedural_generation3d::declaration().map_err(PluginAssemblyError::definition)?)
        .host_media_handler(HostMediaHandlerDeclaration::mesh_import(
            "s.procedural.host-media.mesh-import",
            semio_s_artifact_procedural_generation3d::artifact_kind(),
            semio_s_artifact_procedural_generation3d::GENERATION_3D_SCHEMA,
            semio_s_artifact_procedural_generation3d::editor::generation3d::generation3d_document_from_mesh,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.brep",
            FlowExtensionManifest::new("brep", "Brep", "0.3.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.brep", "semio.s.plugin.flow.extension.brep", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.math",
            FlowExtensionManifest::new("math", "Math", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.math", "semio.s.plugin.flow.extension.math", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.primitive",
            FlowExtensionManifest::new("core", "Core", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.primitive", "semio.s.plugin.flow.extension.primitive", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.logic",
            FlowExtensionManifest::new("logic", "Logic", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.logic", "semio.s.plugin.flow.extension.logic", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.dictionary",
            FlowExtensionManifest::new("dictionary", "Dictionary", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.dictionary", "semio.s.plugin.flow.extension.dictionary", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.list",
            FlowExtensionManifest::new("list", "List", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.list", "semio.s.plugin.flow.extension.list", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.text",
            FlowExtensionManifest::new("text", "Text", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.text", "semio.s.plugin.flow.extension.text", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.draw",
            FlowExtensionManifest::new("draw", "Draw", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.draw", "semio.s.plugin.flow.extension.draw", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.bim",
            FlowExtensionManifest::new("bim", "Bim", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.bim", "semio.s.plugin.flow.extension.bim", "register")?,
        )?)
        .editor_with_examples::<semio_s_artifact_procedural_generation2d::editor::generation2d::Generation2dPlayApp>(semio_s_artifact_procedural_generation2d::editor::generation2d::create_generation2d_app(), vec![semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::examples::demo::source()])
        .editor_mutation_roster::<semio_s_artifact_procedural_generation2d::editor::generation2d::Generation2dPlayApp>()
        .viewer::<semio_s_artifact_procedural_generation2d::viewer::generation2d::Generation2dViewer>(semio_s_artifact_procedural_generation2d::viewer::generation2d::create_generation2d_viewer())
        .viewer_mutation_roster::<semio_s_artifact_procedural_generation2d::viewer::generation2d::Generation2dViewer>()
        .editor_with_examples::<semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dPlayApp>(semio_s_artifact_procedural_generation3d::editor::generation3d::create_generation3d_app(), semio_s_artifact_procedural_generation3d::editor::generation3d::examples())
        .editor_mutation_roster::<semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dPlayApp>()
        .viewer::<semio_s_artifact_procedural_generation3d::viewer::generation3d::Generation3dViewer>(semio_s_artifact_procedural_generation3d::viewer::generation3d::create_generation3d_viewer())
        .viewer_mutation_roster::<semio_s_artifact_procedural_generation3d::viewer::generation3d::Generation3dViewer>()
        // 🚧️ assembly's editor/viewer are authored (`🗿️artifacts/🧩️assembly/…/{✏️editor,👁️viewer}/`) but
        // not yet mounted in `🦀️.rs` or registered here: `ArtifactEditor`/`ArtifactViewer`'s own
        // trait bounds (`Snapshot: ArtifactDsl + ArtifactPack`, `Mutation`/`Command`: `OpText`/`OpBinary`)
        // are unsatisfied until assembly's schema gains its missing artifact-facet descriptor + leaf
        // set — see `📓️w2-p5-assembly-notes.md`. Wire once that lands.
        //
        // 🧬️ Assembly's editor remains unmounted, but its schema-owned `semio.infer` WFC factory
        // is registered above on the production action bus and needs no artifact surface.
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_procedural_generation2d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_procedural_generation3d::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("documents.write".into()),
            scope: "plugin".into(),
            reason: "persist generation2d/generation3d editor edits (flow graph parameter/node changes) to the open document".into(),
            optional: false,
        })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin, ProceduralApps);
