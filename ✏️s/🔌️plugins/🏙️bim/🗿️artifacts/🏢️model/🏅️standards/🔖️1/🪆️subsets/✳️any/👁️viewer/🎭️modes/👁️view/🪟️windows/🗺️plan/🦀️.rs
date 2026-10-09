//! 🗺️ BIM model viewer plan window: the read-only Canvas2d surface of the `plan-linework` inference of one storey, built by the layer builder
//! the editor uses too (`crate::render::plan`, mounted at the artifact level; this file imports nothing from the sibling editor surface). It
//! owns the window configuration (storey and viewport) and its chrome (storey picker); the camera and the storey verbs travel the retained
//! window-config route declared by the viewer root.

use crate::render::plan::scene;
use crate::viewer::bim::terminology::BimViewerLabels;
use crate::{ModelInference, ModelSnapshot};
use semio_framework_plugin::{scene_surface, ActionDescriptor, BuiltNode, MeasureSelectItem, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowMeasure, WindowOptions};
use semio_framework_ui_locale::LocalizedLabel;

#[path = "🎚️config/🦀️.rs"]
pub mod config;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "bim-view-plan";
pub const BODY_KEY: &str = "bim.view.plan";
const SURFACE_ID: &str = "bim.view.canvas2d/plan";
const MEASURE_STOREY: &str = "bim-view-measure-plan-storey";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::bim::create_bim_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native(BimViewerLabels::NATIVE_EN.window_plan.as_str(), BimViewerLabels::NATIVE_DE.window_plan.as_str()),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "bim-plan".into(),
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
/// 🗺️ Pure `ModelSnapshot × ModelInference × window configuration × selected element ids -> Canvas2d surface` of the plan of the configured storey.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, config: &config::BimViewerPlanWindowConfig, selected: &[String]) -> UiAssemblyResult<BuiltNode> {
    let plan = crate::render::plan_storey(snapshot, &config.storey).and_then(|storey| inference.plan_linework.get(&storey));
    scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Canvas2d, &scene(plan, &config.viewport, config.framed, selected))
}
//#endregion 🔖️Render

//#region 🔖️Measures
/// 🎚️ The window chrome: the storey picker.
pub fn measures(snapshot: &ModelSnapshot, config: &config::BimViewerPlanWindowConfig, labels: &BimViewerLabels, action: impl Fn(&str, Option<semio_framework_plugin::DslValue>) -> ActionDescriptor) -> Vec<WindowMeasure> {
    let items = crate::render::storeys(snapshot).into_iter().map(|row| MeasureSelectItem { id: format!("{MEASURE_STOREY}-{}", row.id), value: row.id, label: row.name }).collect();
    vec![WindowMeasure::Select {
        id: MEASURE_STOREY.into(),
        label: Some(labels.storey.into()),
        value: crate::render::plan_storey(snapshot, &config.storey).unwrap_or_default(),
        items,
        on_change: action("setPlanStorey", None),
    }]
}
//#endregion 🔖️Measures

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
