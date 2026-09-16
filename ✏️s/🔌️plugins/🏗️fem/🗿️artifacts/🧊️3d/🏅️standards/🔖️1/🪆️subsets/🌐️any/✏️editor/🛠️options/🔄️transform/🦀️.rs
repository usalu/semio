//! 🔄️ Transform utility options — the gumball handle toggles of the focused fem3d model window.
//! Read from that window's captured config (a panel/measure projection binds to the focused pane),
//! written through `setTransformGumballFlag`.

use crate::editor::fem3d::interaction::FEM3D_UTILITY_TRANSFORM;
use crate::editor::fem3d::modes::edit::windows::model::config::Fem3dGumballConfig;
use crate::editor::fem3d::terminology::Fem3dLabels;
use crate::editor::fem3d::{fem3d_measure_action, FEM3D_PLAY_CONTROLLER_ID};
use dsl::json;
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
mod tests {
    use super::*;
    use semio_framework_plugin::ViewModel;

    #[test]
    fn transform_options_group_is_tagged_for_transform_utility() {
        let labels = crate::editor::fem3d::terminology::fem3d_labels(&ViewModel::default());
        let group = measure(&Fem3dGumballConfig::default(), labels);
        let WindowMeasure::Group { id, active_utility_id, children, .. } = group else { panic!("group") };
        assert_eq!(id, format!("{FEM3D_PLAY_CONTROLLER_ID}-utility-options-transform"));
        assert_eq!(active_utility_id.as_deref(), Some(UTILITY_ID));
        assert_eq!(children.len(), 5);
    }

    #[test]
    fn gumball_flags_toggle_and_refuse_unknown_names() {
        let mut config = Fem3dGumballConfig::default();
        config.set_flag("rotate", Some(false)).expect("set");
        assert!(!config.rotate);
        config.set_flag("rotate", None).expect("toggle");
        assert!(config.rotate);
        assert!(config.set_flag("mirror", None).is_err());
        let off = Fem3dGumballConfig { move_axes: false, move_planes: false, rotate: false, scale_axes: false, scale_uniform: false };
        assert!(!off.any());
    }
}
