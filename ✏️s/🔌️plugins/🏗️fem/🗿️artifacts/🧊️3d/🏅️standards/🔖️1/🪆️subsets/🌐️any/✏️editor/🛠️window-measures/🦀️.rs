//! 🎛️ Per-window chrome measures of the fem3d model window: the Transform utility's handle options.

use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::gumball::Fem3dGumballConfig;
use semio_s_artifact_fem_3d_editor_terminology::editor::fem3d::terminology::Fem3dLabels;
use semio_framework_plugin::WindowMeasure;

/// 🎛️ Window chrome measures for the model window, read from its captured gumball config.
pub fn fem3d_window_measures(config: &Fem3dGumballConfig, labels: &Fem3dLabels) -> Vec<WindowMeasure> {
    vec![crate::editor::fem3d::options::transform::measure(config, labels)]
}
