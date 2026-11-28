//! 🔄️ Transform utility options — gumball handle toggles for the fem2d canvas windows.

use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::canvas_gesture::FEM2D_UTILITY_TRANSFORM;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::gumball::Fem2dGumballConfig;
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::Fem2dLabels;
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::fem2d_measure_action;
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::FEM2D_PLAY_CONTROLLER_ID;
use semio_framework_pack_json::json;
use semio_plugin::WindowMeasure;

pub const UTILITY_ID: &str = FEM2D_UTILITY_TRANSFORM;

/// 🎛️ Utility Options when the Transform utility is active, read from the model window's gumball config.
pub fn measure(config: &Fem2dGumballConfig, labels: &Fem2dLabels) -> WindowMeasure {
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
                icon_id: "maximize-2".into(),
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
