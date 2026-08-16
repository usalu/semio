//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("flow")
        .label("Flow")
        .version("0.1.0")
        .setup(crate::apps::flow::register)
        .document_app::<crate::apps::flow::FlowPlayApp>(crate::apps::flow::create_flow_app())
        .try_build()
}
