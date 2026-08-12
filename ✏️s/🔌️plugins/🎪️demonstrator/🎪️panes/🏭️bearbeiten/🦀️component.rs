//! 🏭️ `bearbeiten` pane — the demonstrator's entwerfen-mit-bestand fabrication surface, served by
//! 🏭️process's `process3d-play` app. `🏭️process` owns `"3d.process"` and self-registers its own
//! mesh/DWG codecs from its own artifact `⚙️engine`; this pane no longer duplicates that
//! registration (see APA ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`,
//! `📓️w3-semio-s-plugin-demonstrator-report.md` — the prior duplicate write raced process3d's own
//! registration for the same OS-global key, a load-order-dependent bug this deletion fixes).
//!
//! See <https://github.com/usalu/semio/issues/2510> for the bundle rationale.

use semio_framework_plugin::Plugin;

use process::apps::process3d::{create_process3d_app, Process3dPlayApp};
use process::artifacts::process3d::PROCESS_3D_SCHEMA;

/// 🔌️ Binds the app's document codec into the plugin runtime. Mesh/DWG codec registration for
/// `"3d.process"` belongs to `🏭️process` alone — see the module docstring.
pub fn register_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Process3dPlayApp>(PROCESS_3D_SCHEMA);
}

/// 🎪️ Adds the pane's app to the shared demonstrator bundle.
pub fn register_app(bundle: Plugin) -> Plugin {
    bundle.register_document_app::<Process3dPlayApp>(create_process3d_app())
}
