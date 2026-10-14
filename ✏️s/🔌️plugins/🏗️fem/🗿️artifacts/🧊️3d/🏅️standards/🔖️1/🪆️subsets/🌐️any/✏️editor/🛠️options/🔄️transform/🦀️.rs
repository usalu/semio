//! 🔄️ Transform utility options — the gumball handle toggles of the focused fem3d model window.
//! Read from that window's captured config (a panel/measure projection binds to the focused pane),
//! written through `setTransformGumballFlag`.

use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_UTILITY_TRANSFORM;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::gumball::Fem3dGumballConfig;
use semio_s_artifact_fem_3d_editor_terminology::editor::fem3d::terminology::Fem3dLabels;
use semio_s_artifact_fem_3d_editor_terminology::editor::fem3d::terminology::fem3d_measure_action;
use semio_s_artifact_fem_3d_editor_terminology::editor::fem3d::terminology::FEM3D_PLAY_CONTROLLER_ID;
use semio_framework_pack_json::json;
use semio_framework_plugin::WindowMeasure;

pub const UTILITY_ID: &str = FEM3D_UTILITY_TRANSFORM;

/// 🎛️ Utility Options when the Transform utility is active — one toggle per handle family.
pub fn measure(config: &Fem3dGumballConfig, labels: &Fem3dLabels) -> WindowMeasure {
    let toggle = |id: &str, icon: &str, label: &str, pressed: bool, flag: &str| WindowMeasure::Toggle {
        id: id.into(),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed,
        text: None,
        on_change: fem3d_measure_action("setTransformGumballFlag", Some(json!({ "flag": flag }))),
    };
    WindowMeasure::Group {
        id: format!("{FEM3D_PLAY_CONTROLLER_ID}-utility-options-transform"),
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
            toggle("fem3d-transform-move", "move-3d", labels.move_flag.as_str(), config.move_axes, "move"),
            toggle("fem3d-transform-move-planes", "square", labels.move_planes_flag.as_str(), config.move_planes, "movePlanes"),
            toggle("fem3d-transform-rotate", "rotate-cw", labels.rotate_flag.as_str(), config.rotate, "rotate"),
            toggle("fem3d-transform-scale-axes", "scaling", labels.scale_axes_flag.as_str(), config.scale_axes, "scaleAxes"),
            toggle("fem3d-transform-scale-uniform", "expand", labels.scale_uniform_flag.as_str(), config.scale_uniform, "scaleUniform"),
        ],
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
