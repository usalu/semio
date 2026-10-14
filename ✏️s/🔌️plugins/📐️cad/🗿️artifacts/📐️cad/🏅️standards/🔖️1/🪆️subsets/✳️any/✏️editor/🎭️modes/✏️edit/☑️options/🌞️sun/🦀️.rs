//! 🌞️ Edit-mode window option — the sun/environment control group. App-global (one `"cad"` measure
//! id shared by every pane), exposed on each world-3d window's chrome.

use semio_s_artifact_cad_cad_kit::{cad_window_action, CadPlayRuntime};
use semio_framework_plugin::{world3d_sun_measures, WindowMeasure};

pub fn measure(runtime: &CadPlayRuntime, is_de: bool) -> WindowMeasure {
    world3d_sun_measures("cad", &runtime.sun, is_de, |action, args| cad_window_action(action, args))
}
