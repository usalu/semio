//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

fn register_exports() {
    crate::artifacts::procedural2d::engine::register();
    crate::artifacts::procedural3d::engine::register();
    install_linked_flow_extensions();
}

/// 🔗 Registers in-process flow extension operators so eval + tessellate share one brep kernel.
fn install_linked_flow_extensions() {
    use flow::register_linked_flow_extension_installer;
    register_linked_flow_extension_installer("brep", semio_s_plugin_flow_extension_brep::register);
    register_linked_flow_extension_installer("math", semio_s_plugin_flow_extension_math::register);
    register_linked_flow_extension_installer("primitive", semio_s_plugin_flow_extension_primitive::register);
    register_linked_flow_extension_installer("logic", semio_s_plugin_flow_extension_logic::register);
    register_linked_flow_extension_installer("dictionary", semio_s_plugin_flow_extension_dictionary::register);
    register_linked_flow_extension_installer("list", semio_s_plugin_flow_extension_list::register);
    register_linked_flow_extension_installer("text", semio_s_plugin_flow_extension_text::register);
    // Force registry rebuild so linked installers are applied before the first eval tick.
    flow::sync_host_flow_extension_contributions("[]");
}

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("procedural")
        .label("Procedural")
        .version("0.1.0")
        .setup(register_exports)
        .register_document_app::<crate::apps::procedural2d::Procedural2dPlayApp>(crate::apps::procedural2d::create_procedural2d_app())
        .register_document_app::<crate::apps::procedural3d::Procedural3dPlayApp>(crate::apps::procedural3d::create_procedural3d_app())
        .build()
}
