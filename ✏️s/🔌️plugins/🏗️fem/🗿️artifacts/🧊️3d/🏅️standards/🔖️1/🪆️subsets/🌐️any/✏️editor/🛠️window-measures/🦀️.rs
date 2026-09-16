//! 🎛️ Per-window chrome measures of the fem3d model window: the Transform utility's handle options.

use crate::editor::fem3d::modes::edit::windows::model::config::Fem3dGumballConfig;
use crate::editor::fem3d::terminology::Fem3dLabels;
use semio_framework_plugin::WindowMeasure;

/// 🎛️ Window chrome measures for the model window, read from its captured gumball config.
pub fn fem3d_window_measures(config: &Fem3dGumballConfig, labels: &Fem3dLabels) -> Vec<WindowMeasure> {
    vec![crate::editor::fem3d::options::transform::measure(config, labels)]
}
