//! 🧊️ BIM model viewer world window: the read-only World3d surface of the `element-solids` inference, built by the scene builder the editor
//! uses too (`crate::render::world`, mounted at the artifact level; this file imports nothing from the sibling editor surface). It owns the
//! window configuration (orbit, projection preset bank, hidden storeys) and its chrome (projection tree, storey visibility toggles); the
//! camera and the other configuration verbs travel the retained window-config route declared by the viewer root.

use crate::render::world::{scene, WorldView};
use crate::viewer::bim::terminology::BimViewerLabels;
use crate::{ModelInference, ModelSnapshot};
use semio_framework_plugin::{scene_surface, world3d_projection_measures, ActionDescriptor, BuiltNode, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowMeasure, WindowOptions};
use semio_framework_ui_locale::LocalizedLabel;

#[path = "🎚️config/🦀️.rs"]
pub mod config;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "bim-view-world";
pub const BODY_KEY: &str = "bim.view.world";
const SURFACE_ID: &str = "bim.view.world3d/world";
const MEASURE_PREFIX: &str = "bim-view";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::bim::create_bim_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: WINDOW_KIND_ID.into(),
        label: BimViewerLabels::localized(|labels| labels.window_world),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "bim-world".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ The framework selection and hover marks a render paints.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Marks {
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
}

/// 👁️ Pure `ModelSnapshot × ModelInference × window configuration × marks -> World3d surface`.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, config: &config::BimViewerWorldWindowConfig, marks: &Marks) -> UiAssemblyResult<BuiltNode> {
    let view = WorldView { orbit: config.framed.then_some(&config.orbit), projection: &config.projection, hidden_storeys: &config.hidden_storeys, selected: &marks.selected, hovered: &marks.hovered };
    scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::World3d, &scene(snapshot, &inference.element_solids, &view))
}
//#endregion 🔖️Render

//#region 🔖️Measures
/// 🎚️ The window chrome: the projection preset tree and one visibility toggle per storey.
pub fn measures(snapshot: &ModelSnapshot, config: &config::BimViewerWorldWindowConfig, labels: &BimViewerLabels, action: impl Fn(&str, Option<semio_framework_plugin::DslValue>) -> ActionDescriptor + Copy) -> Vec<WindowMeasure> {
    let toggles = crate::render::storeys(snapshot)
        .into_iter()
        .map(|row| WindowMeasure::Toggle {
            id: format!("{MEASURE_PREFIX}-measure-storey-{}", row.id),
            icon_id: "layers".into(),
            label: Some(row.name),
            pressed: config.shows(&row.id),
            text: None,
            on_change: action("setStoreyVisible", Some(semio_framework_plugin::DslValue::object([("storey".to_string(), semio_framework_plugin::DslValue::String(row.id))]))),
        })
        .collect();
    vec![
        world3d_projection_measures(MEASURE_PREFIX, &config.projection, action),
        WindowMeasure::Group {
            id: format!("{MEASURE_PREFIX}-measure-storeys"),
            label: labels.storeys.into(),
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
            children: toggles,
        },
    ]
}
//#endregion 🔖️Measures

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
