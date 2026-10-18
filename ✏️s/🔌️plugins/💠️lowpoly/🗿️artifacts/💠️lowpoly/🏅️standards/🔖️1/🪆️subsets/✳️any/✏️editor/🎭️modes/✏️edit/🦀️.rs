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
use semio_framework_plugin::{WindowEngagement, WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible, WindowEngagementStatus, WindowMeasure};
use semio_framework_ui_locale::LabelText;

use crate::windows::model;
use semio_framework_plugin::create_default_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;

pub const LOWPOLY_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::lowpoly::create_lowpoly_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: LOWPOLY_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[model::LOWPOLY_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Model".into()]))
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
        // 🎛️ An engagement option carries no toggle value, so the press flips the flag it reads.
        action: Some(lowpoly_window_action("setUtilityParam", Some(semio_framework_value::DslValue::object([("key".to_string(), semio_framework_value::DslValue::String(key.to_string())), ("value".to_string(), semio_framework_value::DslValue::Bool(!pressed))])))),
    };
    let status = if selected_components > 0 { format!("{active_utility} · {selected_components} {} {}", select.granularity, labels.selected.as_str()) } else { active_utility.to_string() };
    WindowEngagement {
        session_active: Some(true),
        // 🧰️ The move/rotate/scale transform switcher lives in the framework utility bar (declared via
        // `.utility` + window-level `utilities`); the rail carries what a mesh edit needs at hand: the
        // pick granularity, the gumball handle groups, and the three chrome switches.
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
            placeholder: Some("extrude, inset, mirror, decimate".into()),
            disabled: None,
            on_change: Some(lowpoly_window_action("engagementInput", None)),
            on_submit: Some(lowpoly_window_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the mesh domain's live selection
        // count used to read off `LowpolyConfig`; it is framework-owned `InteractionState` now, and
        // `ArtifactApp::window_engagements` (unlike `handle`/`copy_fragment`/`cut_operations`) is not
        // threaded an `InteractionView` this wave — the status line drops the selection summary rather
        // than reading stale app-local state. Peer/self selection is surfaced generically by the shell.
        status: Some(vec![WindowEngagementStatus { id: "lowpoly-status".into(), text: status }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible { id: "lowpoly.eng.extrude".into(), label: labels.extrude.into(), detail: None, action: Some(lowpoly_window_action("extrude", None)) },
            WindowEngagementPossible { id: "lowpoly.eng.triangulate".into(), label: labels.triangulate.into(), detail: None, action: Some(lowpoly_window_action("triangulate", None)) },
        ]),
    }
}
//#endregion 🔖️SharedEngagement
