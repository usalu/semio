//! 🧲️ Lowpoly play app — the vertex-snap grid-size window-chrome group. Shared verbatim by both windows.

use crate::apps::lowpoly::config::LowpolyConfig;
use crate::apps::lowpoly::terminology::LowpolyLabels;
use crate::apps::lowpoly::utility_param_slider;
use crate::apps::lowpoly::view::utility_params_value;
use semio_framework_plugin::WindowMeasure;

/// 🎛️ The live chrome measure for this option.
pub fn measure(config: &LowpolyConfig, labels: &LowpolyLabels) -> WindowMeasure {
    let params = utility_params_value(config);
    WindowMeasure::Group {
        id: "lowpoly-measure-snap".into(),
        label: labels.snap.into(),
        default_open: Some(false),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![utility_param_slider("snap", labels.snap_grid, "snapGrid", &params, 0.25, 0.05, 2.0, 0.05)],
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_builds_the_snap_grid_slider() {
        let config = LowpolyConfig::default();
        let m = measure(&config, semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>("en-US"));
        match m {
            WindowMeasure::Group { children, .. } => assert_eq!(children.len(), 1),
            other => panic!("expected Group, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
