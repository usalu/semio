//! 🪧️ Standalone Bearbeiten pane smoke target: independent of the stale `process3d_editor_unit` modules.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
#[path = "../🧰️support/🦀️.rs"]
mod context;

use crate::context::app;
use semio_plugin::artifact_app_laws;
use semio_s_artifact_process_process3d_commands::step::add_step;
use semio_s_artifact_process_process3d_editor::*;

#[path = "../🔬️unit/🪧️pane-smoke/🦀️.rs"]
mod pane_smoke;
