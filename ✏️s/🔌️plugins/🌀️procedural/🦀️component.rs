//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::{FlowExtensionDeclaration, FlowExtensionExecutableIdentity, FlowExtensionManifest, HostMediaHandlerDeclaration, Plugin};

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("procedural")
        .label("Procedural")
        .version("0.1.0")
        .artifact(crate::artifacts::procedural2d::declaration())
        .artifact(crate::artifacts::procedural3d::declaration())
        .host_media_handler(HostMediaHandlerDeclaration::mesh_dwg_bridge(
            "s.procedural.host-media.mesh-dwg",
            crate::artifacts::procedural3d::artifact_kind(),
            crate::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA,
            crate::apps::procedural3d::procedural3d_document_from_mesh,
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
        .document_app::<crate::apps::procedural2d::Procedural2dPlayApp>(crate::apps::procedural2d::create_procedural2d_app())
        .document_app::<crate::apps::procedural3d::Procedural3dPlayApp>(crate::apps::procedural3d::create_procedural3d_app())
        .try_build()
}
