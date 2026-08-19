//! 🕹️ Edit-mode window option — the Dislocate gumball's Move and Rotate handle toggles, shown only
//! while the window that owns them has the Dislocate utility active.

use crate::editor::cad::config::CadDislocateOptions;
use crate::editor::cad::{cad_action, CAD_DISLOCATE_UTILITY_ID};
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

/// 🎛️ Move and Rotate handle groups shown only while this window owns the Dislocate utility.
pub async fn measure(options: CadDislocateOptions, is_de: bool) -> WindowMeasure {
    WindowMeasure::Group {
        id: "cad-play-utility-options-dislocate".into(),
        label: String::new(),
        default_open: Some(true),
        active_utility_id: Some(CAD_DISLOCATE_UTILITY_ID.into()),
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
                id: "cad-dislocate-move".into(),
                icon_id: "move-3d".into(),
                label: Some(if is_de { "Verschieben" } else { "Move" }.into()),
                pressed: options.move_enabled,
                text: None,
                on_change: cad_action("setDislocateOption", Some(json!({ "option": "move" }))),
            },
            WindowMeasure::Toggle {
                id: "cad-dislocate-rotate".into(),
                icon_id: "rotate-cw".into(),
                label: Some(if is_de { "Drehen" } else { "Rotate" }.into()),
                pressed: options.rotate_enabled,
                text: None,
                on_change: cad_action("setDislocateOption", Some(json!({ "option": "rotate" }))),
            },
        ],
    }
}
