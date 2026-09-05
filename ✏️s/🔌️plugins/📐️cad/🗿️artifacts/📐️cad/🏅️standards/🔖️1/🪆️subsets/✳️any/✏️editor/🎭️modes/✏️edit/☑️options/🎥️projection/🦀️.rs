//! 🎥️ Edit-mode window option — the classical projection control group (orthographic/axonometric/
//! oblique/perspective), bound per pane to that pane's own camera.
//!
//! 🧭️ The four world-3d windows of this mode share one option implementation each, parameterized by
//! pane, rather than each window carrying four byte-identical copies — so the `☑️options` nodes sit
//! at the mode level. See the plugin's migration ticket for the taxonomy note.

use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::cad_camera_projection_config;
use crate::artifacts::cad::CadPaneId;
use crate::editor::cad::{cad_pane_camera_runtime, cad_window_action, CadPlayRuntime};
use semio_framework_plugin::{world3d_projection_measures, WindowMeasure};

pub fn measure(runtime: &CadPlayRuntime, pane: CadPaneId) -> WindowMeasure {
    world3d_projection_measures(&format!("cad-{}", pane.model_definition_id()), &cad_camera_projection_config(cad_pane_camera_runtime(runtime, pane)), |action, args| cad_window_action(action, semio_framework::optional_json_to_dsl(args)))
}
