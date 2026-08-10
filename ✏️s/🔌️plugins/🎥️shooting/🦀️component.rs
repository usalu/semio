//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("shooting")
        .label("Shooting")
        .version("0.1.0")
        .setup(crate::artifacts::shooting::engine::register)
        .register_document_app::<crate::apps::shooting::ShootingPlayApp>(crate::apps::shooting::create_shooting_app())
        .build()
}
