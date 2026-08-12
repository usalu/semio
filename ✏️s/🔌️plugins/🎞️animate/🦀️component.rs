//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("animate")
        .label("Animate")
        .version("0.1.0")
        .setup(crate::apps::present::register)
        .register_document_app::<crate::apps::present::AnimatePresentPlayApp>(crate::apps::present::create_animate_present_app())
        .build()
}
