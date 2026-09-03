//! 🌞️ Edit-mode window option — the sun/environment control group. App-global (one `"cad"` measure
//! id shared by every pane), exposed on each world-3d window's chrome.

use crate::editor::cad::{cad_window_action, CadPlayRuntime};
use semio_framework_plugin::{world3d_sun_measures, WindowMeasure};

pub fn measure(runtime: &CadPlayRuntime) -> WindowMeasure {
    world3d_sun_measures("cad", &runtime.sun, |action, args| cad_window_action(action, semio_framework::optional_json_to_dsl(args)))
}
