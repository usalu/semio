//! 🧲️ Lowpoly play app — the composable transform gumball's window-chrome group: one toggle per handle
//! group (move / rotate / scale), all shown at once by default. The flags live in
//! `LowpolyConfig::utility_params_json` (`gumballMove`/`gumballRotate`/`gumballScale`) beside the snap
//! grid, so a toggle is one `setUtilityParam` and the Model scene's `gumballConfig` reads them back.

use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::lowpoly_window_action;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{utility_param_bool, utility_params_value, GUMBALL_MOVE_PARAM, GUMBALL_ROTATE_PARAM, GUMBALL_SCALE_PARAM};
use semio_framework_plugin::{LabelText, WindowMeasure};

/// 🎛️ The three handle-group switches as the scene publishes them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GumballHandles {
    pub r#move: bool,
    pub rotate: bool,
    pub scale: bool,
}

impl GumballHandles {
    pub fn from_config(config: &LowpolyConfig) -> Self {
        let params = utility_params_value(config);
        Self { r#move: utility_param_bool(&params, GUMBALL_MOVE_PARAM, true), rotate: utility_param_bool(&params, GUMBALL_ROTATE_PARAM, true), scale: utility_param_bool(&params, GUMBALL_SCALE_PARAM, true) }
    }
}

fn handle_toggle(id: &str, icon: &str, label: LabelText, key: &str, pressed: bool) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: format!("lowpoly-gumball-{id}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed,
        text: None,
        on_change: lowpoly_window_action("setUtilityParam", Some((&dsl::DslValue::object([("key".to_string(), dsl::DslValue::String(key.to_string()))])).into())),
    }
}

/// 🎛️ The live chrome measure for this option.
pub fn measure(config: &LowpolyConfig, labels: &LowpolyLabels) -> WindowMeasure {
    let handles = GumballHandles::from_config(config);
    WindowMeasure::Group {
        id: "lowpoly-gumball".into(),
        label: labels.gumball.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            handle_toggle("move", "move", labels.gumball_move, GUMBALL_MOVE_PARAM, handles.r#move),
            handle_toggle("rotate", "rotate-cw", labels.gumball_rotate, GUMBALL_ROTATE_PARAM, handles.rotate),
            handle_toggle("scale", "scaling", labels.gumball_scale, GUMBALL_SCALE_PARAM, handles.scale),
        ],
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
