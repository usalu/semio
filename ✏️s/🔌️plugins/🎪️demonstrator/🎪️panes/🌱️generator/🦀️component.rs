//! 🌱️ `generator` pane — the demonstrator's entwerfen-mit-bestand generator surface, served by
//! 🌀️procedural's `procedural3d-play` app. Only the 3d half of procedural's host wiring is registered
//! here: the pane boots `procedural3d-play` exclusively, so `procedural`'s 2d app is never reached
//! through this bundle. `🌀️procedural` owns `"3d.procedural"` and self-registers its own mesh/DWG
//! codecs from its own artifact `⚙️engine`; this pane no longer duplicates that registration (see
//! APA ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`,
//! `📓️w3-semio-s-plugin-demonstrator-report.md` — the prior duplicate write raced procedural3d's own
//! registration for the same OS-global key, a load-order-dependent bug this deletion fixes).
//!
//! See <https://github.com/usalu/semio/issues/2510> for the bundle rationale.

use semio_framework_plugin::Plugin;

use procedural::apps::procedural3d::{create_procedural3d_app, Procedural3dPlayApp};
use procedural::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA;

/// 🔌️ Binds the app's document codec into the plugin runtime. Mesh/DWG codec registration for
/// `"3d.procedural"` belongs to `🌀️procedural` alone — see the module docstring.
pub fn register_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Procedural3dPlayApp>(PROCEDURAL_3D_SCHEMA);
}

/// 🎪️ Adds the pane's app to the shared demonstrator bundle.
pub fn register_app(bundle: Plugin) -> Plugin {
    bundle.register_document_app::<Procedural3dPlayApp>(create_procedural3d_app())
}
