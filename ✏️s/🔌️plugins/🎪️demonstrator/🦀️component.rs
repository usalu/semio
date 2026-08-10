//! 🔌️ Plugin root contract for the demonstrator multi-pane bundle.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the demonstrator plugin via pane registration.
pub fn plugin() -> Plugin {
    crate::artifacts::playground::engine::register();
    crate::panes::bundle()
}
