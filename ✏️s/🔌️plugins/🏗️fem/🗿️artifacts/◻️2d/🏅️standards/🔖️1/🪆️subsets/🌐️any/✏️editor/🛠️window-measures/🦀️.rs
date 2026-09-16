//! 🎛️ Per-window chrome measures shared by the fem2d model and results canvas windows.

use crate::editor::fem2d::terminology::Fem2dLabels;
use semio_framework_plugin::{ViewModel, WindowMeasure};

/// 🎛️ Window chrome measures for each fem2d canvas window instance.
pub fn fem2d_window_measures(view: &ViewModel, labels: &Fem2dLabels) -> Vec<WindowMeasure> {
    vec![crate::editor::fem2d::options::transform::measure(view, labels)]
}
