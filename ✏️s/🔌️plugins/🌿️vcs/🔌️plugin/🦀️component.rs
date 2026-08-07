//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("vcs")
        .label("VCS")
        .version("0.1.0")
        .setup(crate::artifacts::vcs::engine::register)
        .register_document_app::<crate::apps::vcs::VcsPlayApp>(crate::apps::vcs::create_vcs_app())
        .build()
}
