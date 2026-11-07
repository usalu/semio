//! 🪧️ Bearbeiten pane smoke target: the shared app harness plus the pane-smoke law alone, so the pane is provable without the full unit tree.

#![recursion_limit = "512"]
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
#[path = "../🧰️support/🦀️.rs"]
mod context;
use semio_s_artifact_process_process3d_commands::step::add_step;
use semio_s_artifact_process_process3d_editor::*;
use semio_s_artifact_process_process3d_modes::windows::workpiece::{self, PROCESS_3D_PLAY_BODY_MAIN};
use semio_plugin::{artifact_app_laws, EditorApp, PluginApp};
use crate::context::unseeded_app_with_registry as app;

#[path = "../🔬️unit/🪧️pane-smoke/🦀️.rs"]
mod pane_smoke;

/// 📥️ LAW: the host delivers extension contributions as the declared `setContributions` command; the UI dispatch door resolves it through the command registry, so the seeded app boots and a second delivery settles.
#[semio_framework_async_macros::async_test]
async fn host_delivered_set_contributions_dispatches_through_the_command_registry() {
    let mut app = crate::context::app_with_registry();
    crate::context::dispatch(&mut app, Process3dCommand::SetContributions(semio_s_artifact_process_process3d_commands::contribution::set_contributions::SetContributions { json: "[]".into() }));
    app.snapshot().expect("the app keeps a readable snapshot after the delivery");
}
