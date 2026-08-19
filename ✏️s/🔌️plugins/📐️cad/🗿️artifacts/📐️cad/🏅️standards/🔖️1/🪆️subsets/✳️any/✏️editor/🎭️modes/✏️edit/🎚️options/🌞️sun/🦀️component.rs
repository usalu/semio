//! 🌞️ Edit-mode window option — the sun/environment control group. App-global (one `"cad"` measure
//! id shared by every pane), exposed on each world-3d window's chrome.

use crate::editor::cad::{cad_action, CadPlayRuntime};
use semio_framework_plugin::{world3d_sun_measures, WindowMeasure};

pub async fn measure(runtime: &CadPlayRuntime) -> WindowMeasure {
    world3d_sun_measures("cad", &runtime.sun, cad_action)
}
