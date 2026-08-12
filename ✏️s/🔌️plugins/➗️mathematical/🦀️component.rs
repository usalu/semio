//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)` escape
/// hatch; `.setup()` itself is gone (W1c) — `MathematicalPlayApp::app_schema()` now answers the one
/// thing it used to survive for, registered automatically by `register_document_app` below.
pub fn plugin() -> Plugin {
    Plugin::builder("mathematical")
        .label("Mathematical")
        .version("0.1.0")
        .artifact(crate::artifacts::mathematical::declaration())
        .register_document_app::<crate::apps::mathematical::MathematicalPlayApp>(crate::apps::mathematical::create_mathematical_app())
        .build()
}
