//! 🗂️ `aussuchen` pane — the demonstrator's entwerfen-mit-bestand selection surface, served by
//! 🪵️sourcing's `sourcing-curate` app. Curation documents carry no geometry, so the pane registers a
//! document codec only — no mesh/solid/dwg handlers.
//!
//! See <https://github.com/usalu/semio/issues/2510> for the bundle rationale.

use semio_framework_plugin::Plugin;

use sourcing::apps::curate::{create_sourcing_curate_app, SourcingCurateApp};
use sourcing::artifacts::curate::SOURCING_CURATE_SCHEMA;

/// 🔌️ Binds the curate app's document codec into the plugin runtime.
pub fn register_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<SourcingCurateApp>(SOURCING_CURATE_SCHEMA);
}

/// 🎪️ Adds the pane's app to the shared demonstrator bundle.
pub fn register_app(bundle: Plugin) -> Plugin {
    bundle.register_document_app::<SourcingCurateApp>(create_sourcing_curate_app())
}
