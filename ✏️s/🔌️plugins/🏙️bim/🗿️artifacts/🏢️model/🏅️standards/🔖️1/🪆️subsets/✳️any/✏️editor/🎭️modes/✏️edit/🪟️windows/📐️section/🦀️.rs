//! 📐️ BIM section window: one authored section or elevation view on a `Canvas2d` surface. It paints the `view-linework` inference of the configured view: the poché of the solids the plane cuts, what lies behind the
//! plane with its hidden lines removed, and the storey levels as datum lines. The drawing is in the coordinates of the view, `u` running to the right along the plane and the elevation above the building datum running up, and
//! the selection is highlighted by element id. Nothing here computes geometry: a view without linework paints nothing, and the plane itself is authored in the view, never in this window.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use self::config::BimSectionWindowConfig;
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::{canvas_surface, meta_record, text_record};
use crate::editor::bim::terminology::BimLabels;
use crate::{ModelInference, ModelSnapshot, ViewPlane};
use semio_framework_plugin::DslValue;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "bim-edit-section";
pub const BODY_KEY: &str = "bim.edit.section";
const SURFACE_ID: &str = "bim.edit.section2d/section";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: crate::editor::bim::utilities::initial(),
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native(BimLabels::NATIVE_EN.window_section.as_str(), BimLabels::NATIVE_DE.window_section.as_str()),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "scissors".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: crate::editor::bim::utilities::for_window(WINDOW_KIND_ID),
        interactions: vec![InteractionRef::new(BIM_ELEMENT_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️View
/// 📐️ Every section and elevation view of the model in browser order: by building, sections before elevations, then name.
pub fn vertical_views(snapshot: &ModelSnapshot) -> Vec<String> {
    let mut rows: Vec<(&String, bool, &String, &String)> = snapshot.views.iter().filter(|(_, view)| view.kind.is_vertical() && view.plane.is_some()).map(|(id, view)| (&view.building, view.kind == crate::ViewKind::Elevation, &view.name, id)).collect();
    rows.sort();
    rows.into_iter().map(|row| row.3.clone()).collect()
}

/// 📐️ The view a section window shows: its configured one while it still is a section or elevation, else the first one in browser order; none when the model has no such view.
pub fn active_view(snapshot: &ModelSnapshot, config: &BimSectionWindowConfig) -> Option<String> {
    if snapshot.views.get(&config.view).is_some_and(|view| view.kind.is_vertical() && view.plane.is_some()) {
        return Some(config.view.clone());
    }
    vertical_views(snapshot).into_iter().next()
}

/// 📐️ The plane of the view a section window shows.
pub fn plane_of(snapshot: &ModelSnapshot, config: &BimSectionWindowConfig) -> Option<ViewPlane> {
    active_view(snapshot, config).and_then(|id| snapshot.views.get(&id)).and_then(|view| view.plane)
}
//#endregion 🔖️View

//#region 🔖️Render
/// 🎨️ The records of the section: the layers of the view's drawing (selection in the accent colour), then the utility meta record.
pub fn records(drawing: Option<&crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework>, selection: &[String], utility: &str, labels: &BimLabels) -> Vec<DslValue> {
    let mut records = vec![meta_record(utility)];
    match drawing {
        Some(drawing) => records.extend(crate::render::plan::layers(&drawing.lines, selection)),
        None => records.push(text_record("empty", (0.0, 0.0), labels.empty_section.as_str(), crate::render::plan::TEXT_SIZE * 2.0, [0.22, 0.24, 0.28, 1.0])),
    }
    records
}

/// 📐️ Renders the section window.
#[allow(clippy::too_many_arguments)]
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimSectionWindowConfig, selection: &[String], utility: &str, revision: u32, labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    render_over(snapshot, inference, config, selection, utility, revision, labels, &[])
}

/// 📐️ [`render`] with the records of the authoring overlay (storey top handles, the gesture preview) painted last, over everything.
#[allow(clippy::too_many_arguments)]
pub fn render_over(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimSectionWindowConfig, selection: &[String], utility: &str, revision: u32, labels: &BimLabels, overlay: &[DslValue]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let drawing = active_view(snapshot, config).and_then(|view| inference.view_linework.get(&view));
    let framing = drawing.filter(|_| !config.framed).and_then(|drawing| crate::render::plan::canvas_bounds(&drawing.lines)).map(|bounds| semio_framework_plugin::Canvas2dFraming { revision, bounds, padding: 48.0 });
    let mut records = records(drawing, selection, utility, labels);
    records.extend(overlay.iter().cloned());
    canvas_surface(SURFACE_ID, config.viewport, framing, &records)
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
