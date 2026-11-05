//! ✏️ Lowpoly play app — the `edit` mode: the default single-window mesh-editing layout (Model only).
//! The `paint` mode (sibling `🎨️paint/`) adds the UV window via its own named layout.


//#region 🧩️Modules
#[path = "."]
pub mod windows {
    #[path = "🪟️windows/🌐️model/🦀️.rs"]
    pub mod model;
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod lowpoly {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod paint {
                #[path = "../🎨️paint/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../🎨️paint/🪟️windows/🖼️uv/🦀️.rs"]
                    pub mod uv;
                }
            }
        }
    }
}
//#endregion 🧩️Modules

use semio_s_artifact_lowpoly_lowpoly_config::LowpolyConfig;
use semio_s_artifact_lowpoly_lowpoly_kit::lowpoly_window_action;
use semio_s_artifact_lowpoly_lowpoly_view::{editor::lowpoly::terminology::LowpolyLabels, LowpolyView, MESH_GRANULARITY_OBJECT, MESH_INTERACTION_DOMAIN};
use semio_plugin::{WindowEngagement, WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible, WindowEngagementStatus, WindowMeasure};
use semio_ui_locale::LabelText;

use crate::windows::model;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::ModeDefinition;
use semio_plugin::WindowLayout;

pub const LOWPOLY_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::lowpoly::create_lowpoly_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: LOWPOLY_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    lowpoly_layout(&[model::LOWPOLY_PLAY_WINDOW_MAIN], &[100.0])
}
/// 🪟️ Layout structure leaves window titles to their localized window-kind definitions.
pub fn lowpoly_layout(window_ids: &[&str], sizes: &[f64]) -> WindowLayout {
    use semio_plugin::{create_window_layout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode};
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
        kind: "row".into(), size: None,
        children: window_ids.iter().enumerate().map(|(index, id)| WindowLayoutChild::Stack(WindowLayoutStackNode {
            kind: "stack".into(), size: sizes.get(index).copied(), active_window_kind_id: None,
            children: vec![create_window_layout(*id, None, None, None)],
        })).collect(),
    }) }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️SharedMeasures
/// 🎛️ Collects every window-chrome measure from the app-level `🛠️options/*` shared by both windows
/// (Model + UV expose an identical set — see this file's top-level doc comment).
pub fn lowpoly_window_measures(config: &LowpolyConfig, labels: &LowpolyLabels, select: &semio_s_artifact_lowpoly_lowpoly_options::select::SelectState) -> Vec<WindowMeasure> {
    use semio_s_artifact_lowpoly_lowpoly_options as options;
    vec![
        options::show_edges::measure(config, labels),
        options::sun::measure(config, labels),
        options::snap::measure(config, labels),
        options::gumball::measure(config, labels),
        options::select::measure(config, labels, select),
        options::paint_params_brush::measure(config, labels),
        options::paint_params_eraser::measure(config, labels),
    ]
}
//#endregion 🔖️SharedMeasures
//#region 🔖️SharedEngagement
/// 🎛️ The window engagement (options/status/input/possible-engagements) shared byte-identically by both
/// windows — see this file's top-level doc comment.
pub fn lowpoly_window_engagement(view: LowpolyView<'_>, active_utility: &str, labels: &LowpolyLabels) -> WindowEngagement {
    lowpoly_window_engagement_with_selection(view, active_utility, labels, &semio_s_artifact_lowpoly_lowpoly_options::select::SelectState::default(), 0)
}
/// 🎯️ The engagement with the mesh domain's LIVE state: which granularity the next pick addresses, how
/// many components are selected, and which gumball handle groups are on. These switches live in the
/// engagement's quick-action rail (the always-visible strip that yields to chrome panels) rather than
/// only in Window Options, which the framework lets a right-anchored panel cover by design.
pub fn lowpoly_window_engagement_with_selection(view: LowpolyView<'_>, active_utility: &str, labels: &LowpolyLabels, select: &semio_s_artifact_lowpoly_lowpoly_options::select::SelectState, selected_components: usize) -> WindowEngagement {
    let config = view.config;
    let handles = semio_s_artifact_lowpoly_lowpoly_options::gumball::GumballHandles::from_config(config);
    let granularity = |id: &str, icon: &str, label: LabelText, granularity_id: &str| WindowEngagementOption {
        id: format!("lowpoly.opt.select-{id}"),
        label: Some(label.into()),
        icon_id: Some(icon.into()),
        pressed: Some(select.granularity == granularity_id),
        disabled: None,
        action: Some(lowpoly_window_action(
            "setInteractionGranularity",
            Some(semio_framework_value::DslValue::object([("domainId".to_string(), semio_framework_value::DslValue::String(MESH_INTERACTION_DOMAIN.to_string())), ("granularityId".to_string(), semio_framework_value::DslValue::String(granularity_id.to_string()))])),
        )),
    };
    let gumball = |id: &str, icon: &str, label: LabelText, key: &str, pressed: bool| WindowEngagementOption {
        id: format!("lowpoly.opt.gumball-{id}"),
        label: Some(label.into()),
        icon_id: Some(icon.into()),
        pressed: Some(pressed),
        disabled: None,
        action: Some(lowpoly_window_action("setUtilityParam", Some(semio_framework_value::DslValue::object([("key".to_string(), semio_framework_value::DslValue::String(key.to_string())), ("value".to_string(), semio_framework_value::DslValue::Bool(!pressed))])))),
    };
    let utility = match active_utility {
        "move" => labels.gumball_move,
        "rotate" => labels.gumball_rotate,
        "scale" => labels.gumball_scale,
        "brush" => labels.brush_group,
        "eraser" => labels.eraser,
        "fill" => labels.fill,
        "eyedropper" => labels.eyedropper,
        _ => labels.utility,
    };
    let status_granularity = match (select.granularity.as_str(), selected_components == 1) {
        ("vertex", true) => labels.vertex,
        ("edge", true) => labels.edge,
        ("face", true) => labels.face,
        ("vertex", false) => labels.vertices,
        ("edge", false) => labels.edges,
        ("face", false) => labels.faces,
        (_, true) => labels.mesh,
        _ => labels.meshes,
    };
    let status = if selected_components > 0 { format!("{} · {selected_components} {} {}", utility.as_str(), status_granularity.as_str(), labels.selected.as_str()) } else { utility.as_str().to_string() };
    WindowEngagement {
        session_active: Some(true),
        options: Some(vec![
            granularity("mesh", "box", labels.mesh, MESH_GRANULARITY_OBJECT),
            granularity("vertex", "circle", labels.vertex, "vertex"),
            granularity("edge", "minus", labels.edge, "edge"),
            granularity("face", "square", labels.face, "face"),
            gumball("move", "move", labels.gumball_move, semio_s_artifact_lowpoly_lowpoly_view::GUMBALL_MOVE_PARAM, handles.r#move),
            gumball("rotate", "rotate-cw", labels.gumball_rotate, semio_s_artifact_lowpoly_lowpoly_view::GUMBALL_ROTATE_PARAM, handles.rotate),
            gumball("scale", "scaling", labels.gumball_scale, semio_s_artifact_lowpoly_lowpoly_view::GUMBALL_SCALE_PARAM, handles.scale),
            WindowEngagementOption { id: "lowpoly.opt.snap".into(), label: Some(labels.snap.into()), icon_id: Some("magnet".into()), pressed: None, disabled: None, action: Some(lowpoly_window_action("snap", None)) },
            WindowEngagementOption { id: "lowpoly.opt.smooth".into(), label: Some(labels.smooth.into()), icon_id: Some("sun".into()), pressed: None, disabled: None, action: Some(lowpoly_window_action("toggleSmooth", None)) },
            WindowEngagementOption {
                id: "lowpoly.opt.show-edges".into(),
                label: Some(labels.show_edges.into()),
                icon_id: Some("grid-3x3".into()),
                pressed: Some(config.show_edges),
                disabled: None,
                action: Some(lowpoly_window_action("toggleShowEdges", None)),
            },
        ]),
        input: Some(WindowEngagementInput {
            id: Some("lowpoly-engagement".into()),
            value: Some(config.engagement_input.clone()),
            placeholder: Some(labels.command_placeholder.as_str().into()),
            disabled: None,
            on_change: Some(lowpoly_window_action("engagementInput", None)),
            on_submit: Some(lowpoly_window_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "lowpoly-status".into(), text: status }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible { id: "lowpoly.eng.extrude".into(), label: labels.extrude.into(), detail: None, action: Some(lowpoly_window_action("extrude", None)) },
            WindowEngagementPossible { id: "lowpoly.eng.triangulate".into(), label: labels.triangulate.into(), detail: None, action: Some(lowpoly_window_action("triangulate", None)) },
        ]),
    }
}
//#endregion 🔖️SharedEngagement
