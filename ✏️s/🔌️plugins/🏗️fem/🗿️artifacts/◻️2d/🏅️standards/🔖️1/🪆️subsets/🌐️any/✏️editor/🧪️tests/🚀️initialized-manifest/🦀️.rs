//! 🚀️ The one manifest every mounting fem2d law passes to the app registry: it runs the same session `initialize()` as the hub composition, so the solve job kind resolves and no law can mount an editor that stays on "Preparing analysis".

use semio_plugin::App;
use semio_s_artifact_fem_2d_editor::editor::fem2d::create_fem2d_app;

/// 🚀️ The initialized fem2d manifest, see [`semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::initialize`].
pub fn manifest() -> App {
    semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::initialize();
    App { definition: create_fem2d_app(), examples: Vec::new() }
}
