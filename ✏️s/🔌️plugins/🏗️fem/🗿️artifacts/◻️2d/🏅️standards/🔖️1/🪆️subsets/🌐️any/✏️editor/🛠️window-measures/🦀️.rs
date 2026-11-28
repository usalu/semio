//! 🎛️ Per-window chrome measures of the fem2d model window.

use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::gumball::Fem2dGumballConfig;
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::Fem2dLabels;
use semio_plugin::WindowMeasure;

/// 🎛️ Window chrome measures of the model window, read from its captured gumball config.
pub fn fem2d_window_measures(config: &Fem2dGumballConfig, labels: &Fem2dLabels) -> Vec<WindowMeasure> {
    vec![crate::editor::fem2d::options::transform::measure(config, labels)]
}
