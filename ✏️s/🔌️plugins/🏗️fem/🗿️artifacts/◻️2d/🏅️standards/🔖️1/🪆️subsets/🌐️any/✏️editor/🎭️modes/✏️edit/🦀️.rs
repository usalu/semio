//! command addresses (kind, camera, body key). The mode owns both windows, so the interaction layer below them never
//! ✏️ Fem2d play app — the `edit` mode: its id and the lookups that resolve which of its two Canvas2d windows a
//! names a window.

pub const MODE_ID: &str = "edit";

use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model as model_window;
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results as results_window;
use semio_framework_os_kernel::Viewport2d;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::ViewModel;

//#region 🔖️WindowCamera
/// 🪟️ The kind id of the window one command is addressed to.
pub fn fem2d_addressed_window_kind<'a>(view: &'a ViewModel, fault: &str) -> Result<&'a str, Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| Fault::from(format!("{fault}.window-required")))?;
    view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| Fault::from(format!("{fault}.window-stale")))
}

/// 🎥️ The camera the addressed Canvas2d window is currently drawn with — both window kinds keep it
/// in their own persisted window config, so a pick must read the one it was produced under.
pub fn fem2d_addressed_camera(cfg: &ConfigView<'_, NoConfig>, view: &ViewModel, fault: &str) -> Result<Viewport2d, Fault> {
    match fem2d_addressed_window_kind(view, fault)? {
        model_window::WINDOW_KIND_ID => Ok(model_window::config::current(cfg).camera),
        results_window::WINDOW_KIND_ID => Ok(results_window::config::current(cfg).camera),
        _ => Err(Fault::from(format!("{fault}.window-kind"))),
    }
}

/// 🪟️ The body key of the window one command is addressed to.
pub fn fem2d_addressed_body_key(view: &ViewModel, fault: &str) -> Result<&'static str, Fault> {
    match fem2d_addressed_window_kind(view, fault)? {
        model_window::WINDOW_KIND_ID => Ok(model_window::BODY_KEY),
        results_window::WINDOW_KIND_ID => Ok(results_window::BODY_KEY),
        _ => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("fem.canvas.window-kind"), format!("{fault}: the addressed window has the wrong kind"))),
    }
}
//#endregion 🔖️WindowCamera
