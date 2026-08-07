//! 🧩️ `aggregator` pane — the demonstrator's entwerfen-mit-bestand aggregation surface, served by
//! 🧩️puzzle's `puzzle3d-play` app. Puzzle publishes its whole host-export set as one
//! `register_puzzle3d_exports()` entry point, so this pane owns no codec wiring of its own.
//!
//! See <https://github.com/usalu/semio/issues/2510> for the bundle rationale.

use semio_framework_plugin::Plugin;

use puzzle::apps::puzzle3d::{create_puzzle3d_app, register_puzzle3d_exports, Puzzle3dPlayApp};

/// 🔌️ Delegates to puzzle's own host-export registration.
pub fn register_exports() {
    register_puzzle3d_exports();
}

/// 🎪️ Adds the pane's app to the shared demonstrator bundle.
pub fn register_app(bundle: Plugin) -> Plugin {
    bundle.register_document_app::<Puzzle3dPlayApp>(create_puzzle3d_app())
}
