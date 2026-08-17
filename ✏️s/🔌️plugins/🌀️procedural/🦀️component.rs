//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::{FlowExtensionDeclaration, FlowExtensionExecutableIdentity, FlowExtensionManifest, HostMediaHandlerDeclaration, Plugin};

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("procedural")
        .label("Procedural")
        .version("0.1.0")
        .artifact(crate::artifacts::procedural2d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::procedural3d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .host_media_handler(HostMediaHandlerDeclaration::mesh_dwg_bridge(
            "s.procedural.host-media.mesh-dwg",
            crate::artifacts::procedural3d::artifact_kind(),
            crate::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA,
            crate::editor::procedural3d::procedural3d_document_from_mesh,
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
        .editor::<crate::editor::procedural2d::Procedural2dPlayApp>(crate::editor::procedural2d::create_procedural2d_app())
        .editor_mutation_roster::<crate::editor::procedural2d::Procedural2dPlayApp>()
        .viewer::<crate::viewer::procedural2d::Procedural2dViewer>(crate::viewer::procedural2d::create_procedural2d_viewer())
        .viewer_mutation_roster::<crate::viewer::procedural2d::Procedural2dViewer>()
        .editor::<crate::editor::procedural3d::Procedural3dPlayApp>(crate::editor::procedural3d::create_procedural3d_app())
        .editor_mutation_roster::<crate::editor::procedural3d::Procedural3dPlayApp>()
        .viewer::<crate::viewer::procedural3d::Procedural3dViewer>(crate::viewer::procedural3d::create_procedural3d_viewer())
        .viewer_mutation_roster::<crate::viewer::procedural3d::Procedural3dViewer>()
        // 🚧️ assembly's editor/viewer are authored (`🗿️artifacts/🧩️assembly/…/{✏️editor,👁️viewer}/`) but
        // not yet mounted in `📦️glue.rs` or registered here: `ArtifactEditor`/`ArtifactViewer`'s own
        // trait bounds (`Snapshot: ArtifactDsl + ArtifactPack`, `Mutation`/`Command`: `OpText`/`OpBinary`)
        // are unsatisfied until assembly's schema gains its missing artifact-facet descriptor + leaf
        // set — see `📓️w2-p5-assembly-notes.md`. Wire once that lands.
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use crate::editor::procedural2d::Procedural2dPlayApp;
    use crate::editor::procedural3d::Procedural3dPlayApp;
    use crate::viewer::procedural2d::Procedural2dViewer;
    use crate::viewer::procedural3d::Procedural3dViewer;

    /// 👁️ A viewer instance never mutates the document store, even when dispatched.
    #[test]
    fn procedural2d_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<Procedural2dViewer>();
    }
    #[test]
    fn procedural3d_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<Procedural3dViewer>();
    }

    /// 🤝️ Editor and viewer surfaces agree on the artifact dialect they address.
    #[test]
    fn procedural2d_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<Procedural2dPlayApp, Procedural2dViewer>();
    }
    #[test]
    fn procedural3d_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<Procedural3dPlayApp, Procedural3dViewer>();
    }
}
//#endregion 🧪️SurfaceTests
