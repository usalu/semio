//! 🔄️ Transform utility options — gumball handle toggles for the fem2d canvas windows.

use crate::editor::fem2d::interaction::canvas_gesture::{fem2d_gesture_window_id_for_render, FEM2D_UTILITY_TRANSFORM};
use crate::editor::fem2d::interaction::gumball::{gumball_config_for_window, set_gumball_flag};
use crate::editor::fem2d::modes::edit::windows::model;
use crate::editor::fem2d::terminology::Fem2dLabels;
use crate::editor::fem2d::{fem2d_measure_action, FEM2D_PLAY_CONTROLLER_ID};
use dsl::json;
use semio_framework_plugin::{ViewModel, WindowMeasure};

pub const UTILITY_ID: &str = FEM2D_UTILITY_TRANSFORM;

fn gumball_window_id(view: &ViewModel) -> String {
    fem2d_gesture_window_id_for_render(view, model::BODY_KEY).or_else(|| view.window_id.clone()).unwrap_or_else(|| "fem2d-model".into())
}

/// 🎛️ Utility Options when the Transform utility is active.
pub fn measure(view: &ViewModel, labels: &Fem2dLabels) -> WindowMeasure {
    let window_id = gumball_window_id(view);
    let config = gumball_config_for_window(&window_id);
    WindowMeasure::Group {
        id: format!("{FEM2D_PLAY_CONTROLLER_ID}-utility-options-transform"),
        label: String::new(),
        default_open: Some(true),
        active_utility_id: Some(UTILITY_ID.into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle {
                id: "fem2d-transform-move".into(),
                icon_id: "move-3d".into(),
                label: Some(labels.move_flag.into()),
                pressed: config.move_axes,
                text: None,
                on_change: fem2d_measure_action("setTransformGumballFlag", Some(json!({ "flag": "move" }))),
            },
            WindowMeasure::Toggle {
                id: "fem2d-transform-rotate".into(),
                icon_id: "rotate-cw".into(),
                label: Some(labels.rotate_flag.into()),
                pressed: config.rotate,
                text: None,
                on_change: fem2d_measure_action("setTransformGumballFlag", Some(json!({ "flag": "rotate" }))),
            },
            WindowMeasure::Toggle {
                id: "fem2d-transform-scale-axes".into(),
                icon_id: "scaling".into(),
                label: Some(labels.scale_axes_flag.into()),
                pressed: config.scale_axes,
                text: None,
                on_change: fem2d_measure_action("setTransformGumballFlag", Some(json!({ "flag": "scaleAxes" }))),
            },
            WindowMeasure::Toggle {
                id: "fem2d-transform-scale-uniform".into(),
                icon_id: "expand".into(),
                label: Some(labels.scale_uniform_flag.into()),
                pressed: config.scale_uniform,
                text: None,
                on_change: fem2d_measure_action("setTransformGumballFlag", Some(json!({ "flag": "scaleUniform" }))),
            },
        ],
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
