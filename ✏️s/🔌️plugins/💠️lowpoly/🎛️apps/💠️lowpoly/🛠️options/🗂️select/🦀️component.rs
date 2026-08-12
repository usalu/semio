//! 🗂️ Lowpoly play app — the always-visible selection method/merge-mode/component-kind window-chrome
//! group (mirrors puzzle 3d's select measures group). Shared verbatim by both windows.

use crate::apps::lowpoly::config::LowpolyConfig;
use crate::apps::lowpoly::lowpoly_action;
use crate::apps::lowpoly::terminology::LowpolyLabels;
use crate::apps::lowpoly::view::selection_targets_from_config;
use semio_framework_plugin::{LabelText, WindowMeasure};
use serde_json::json;

/// 🎯️ One selection-granularity toggle. Selection kinds are a non-exclusive multi-select (mesh + face +
/// edge + vertex can all be active at once), so they are a window-measure toggle group — NOT a
/// single-active utility group.
fn selection_kind_toggle(id: &str, icon: &str, label: LabelText, kind: &str, pressed: bool) -> WindowMeasure {
    WindowMeasure::Toggle { id: format!("lowpoly-select-{id}"), icon_id: icon.into(), label: Some(label.into()), pressed, text: None, on_change: lowpoly_action("toggleSelectionKind", Some(json!({ "kind": kind }))) }
}

/// 🎛️ The live chrome measure for this option.
pub fn measure(config: &LowpolyConfig, labels: &LowpolyLabels) -> WindowMeasure {
    let targets = selection_targets_from_config(config);
    WindowMeasure::Group {
        id: "lowpoly-select".into(),
        label: labels.select.into(),
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
            WindowMeasure::Toggle {
                id: "lowpoly-select-rectangle".into(),
                icon_id: "square".into(),
                label: Some(labels.rectangle.into()),
                pressed: config.selection_method == "rectangle",
                text: None,
                on_change: lowpoly_action("setSelectionMethod", Some(json!({ "method": "rectangle" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-lasso".into(),
                icon_id: "lasso".into(),
                label: Some(labels.lasso.into()),
                pressed: config.selection_method == "lasso",
                text: None,
                on_change: lowpoly_action("setSelectionMethod", Some(json!({ "method": "lasso" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-default".into(),
                icon_id: "mouse-pointer".into(),
                label: Some(labels.selective.into()),
                pressed: config.selection_mode_default == "default",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "default" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-additive".into(),
                icon_id: "plus".into(),
                label: Some(labels.additive.into()),
                pressed: config.selection_mode_default == "additive",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "additive" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-subtractive".into(),
                icon_id: "minus".into(),
                label: Some(labels.subtractive.into()),
                pressed: config.selection_mode_default == "subtractive",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "subtractive" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-invertive".into(),
                icon_id: "arrow-right-left".into(),
                label: Some(labels.invertive.into()),
                pressed: config.selection_mode_default == "invertive",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "invertive" }))),
            },
            selection_kind_toggle("mesh", "box", labels.mesh, "mesh", targets.mesh),
            selection_kind_toggle("face", "square", labels.face, "face", targets.face),
            selection_kind_toggle("edge", "minus", labels.edge, "edge", targets.edge),
            selection_kind_toggle("vertex", "circle", labels.vertex, "vertex", targets.vertex),
        ],
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn select_window_options_mirror_puzzle3d_taxonomy() {
        let measure = measure(&LowpolyConfig::default(), semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>("en-US"));
        let (active_utility_id, children) = match measure {
            WindowMeasure::Group { active_utility_id, children, .. } => (active_utility_id, children),
            other => panic!("expected Group, got {other:?}"),
        };
        assert_eq!(active_utility_id, None, "Select options must always surface in window options");
        let toggle_ids: Vec<&str> = children
            .iter()
            .filter_map(|measure| match measure {
                WindowMeasure::Toggle { id, .. } => Some(id.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(
            toggle_ids,
            vec![
                "lowpoly-select-rectangle",
                "lowpoly-select-lasso",
                "lowpoly-select-mode-default",
                "lowpoly-select-mode-additive",
                "lowpoly-select-mode-subtractive",
                "lowpoly-select-mode-invertive",
                "lowpoly-select-mesh",
                "lowpoly-select-face",
                "lowpoly-select-edge",
                "lowpoly-select-vertex",
            ]
        );

        let mut a = app();
        dispatch(&mut a, LowpolyCommand::SetSelectionMethod(crate::apps::lowpoly::commands::selection::set_selection_method::SetSelectionMethod { value: "lasso".into() }));
        dispatch(&mut a, LowpolyCommand::SetSelectionModeDefault(crate::apps::lowpoly::commands::selection::set_selection_mode_default::SetSelectionModeDefault { value: "additive".into() }));
        let window_measures = a.window_measures();
        let main_measures = window_measures.get(crate::apps::lowpoly::modes::edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN).expect("main window measures");
        let find_toggle = |id: &str| -> Option<bool> {
            main_measures.iter().find_map(|measure| match measure {
                WindowMeasure::Group { id: gid, children, .. } if gid == "lowpoly-select" => children.iter().find_map(|child| match child {
                    WindowMeasure::Toggle { id: tid, pressed, .. } if tid == id => Some(*pressed),
                    _ => None,
                }),
                _ => None,
            })
        };
        assert_eq!(find_toggle("lowpoly-select-lasso"), Some(true));
        assert_eq!(find_toggle("lowpoly-select-rectangle"), Some(false));
        assert_eq!(find_toggle("lowpoly-select-mode-additive"), Some(true));
        assert_eq!(find_toggle("lowpoly-select-mode-default"), Some(false));
    }
}
//#endregion 🧪️Tests
