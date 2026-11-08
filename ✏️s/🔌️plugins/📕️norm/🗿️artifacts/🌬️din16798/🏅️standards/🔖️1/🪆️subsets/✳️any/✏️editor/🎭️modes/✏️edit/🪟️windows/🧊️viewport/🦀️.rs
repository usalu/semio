//! 🧊️ DIN EN 16798 play app — the 3D viewport window: the projected indoor environment with rooms coloured by comfort category, ventilation systems and the drag grips of the selection.

use semio_plugin::{BuiltNode, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
use semio_s_artifact_norm_din16798_scene::live_scene_of;
use semio_s_plugin_norm_app_surface::direct_manipulation::InsertSpec;
use semio_s_plugin_norm_app_surface::viewport3d::lane_overlays::{LaneOverlays, PaletteEntry};
use semio_s_plugin_norm_app_surface::viewport3d::toolbar::{render_viewport_panel, ViewportView};
use semio_s_plugin_norm_app_surface::{interaction_domain_id, LaneFrame, NormSelection};
use semio_s_plugin_norm_compliance::CheckReport;
use semio_s_plugin_norm_structural_scene::{EntityRef, Handle, StructuralScene};

//#region 🔖️Constants
pub const WINDOW_VIEWPORT: &str = "norm-din16798-viewport";
pub const BODY_VIEWPORT: &str = "norm.din16798.play.viewport";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::create_din16798_app`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::viewport_window_definition(crate::VARIANT, LocalizedLabel::native("Model", "Modell"), BODY_VIEWPORT, "box")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧱️ The scene of one document revision; the live viewport projects the preview document, so a drag re-projects it at frame rate.
pub fn scene_of(document: &Din16798Snapshot, report: &CheckReport) -> StructuralScene {
    live_scene_of(document, report)
}

/// 🧱️ The scene projected from scratch (no retained fragments): the baseline of the incremental-rebuild gate of the latency bench.
pub fn full_scene_of(document: &Din16798Snapshot, report: &CheckReport) -> StructuralScene {
    semio_s_artifact_norm_din16798_scene::full_scene_of(document, report)
}

/// 🕹️ The drag handles of the selected entities of `document`.
pub fn handles_of(document: &Din16798Snapshot, selection: &[EntityRef]) -> Vec<Handle> {
    semio_s_artifact_norm_din16798_scene::handles(document, selection)
}

/// 🖐️ The grips the other users hold, resolved from the handles of every entity; empty while nobody drags.
fn peer_handles_of(document: &Din16798Snapshot, scene: &StructuralScene, frame: &LaneFrame) -> Vec<Handle> {
    if frame.transient.peers.is_empty() {
        return Vec::new();
    }
    let all: Vec<EntityRef> = scene.entities.iter().map(|entity| entity.entity.clone()).collect();
    handles_of(document, &all).into_iter().filter(|handle| frame.transient.peers.iter().any(|peer| peer.drag.handle == handle.id)).collect()
}

/// ➕ The insert palette: one tool per entity kind of this standard.
pub fn insert_specs() -> Vec<InsertSpec> {
    semio_s_artifact_norm_din16798_scene::ENTITY_KINDS.iter().map(|kind| InsertSpec { id: kind.id.to_string(), collection: kind.collection.to_string(), kind: kind.scene_kind, label_en: kind.label_en.to_string(), label_de: kind.label_de.to_string() }).collect()
}

/// 🧊️ Projects the document and lowers it to the `world-3d` surface; `selection` is the resolved selection whose scene entities are highlighted, `frame` the live lanes: grips of the selection, the dimension-line entry of the open drag, the insert ghost and the selection of the other users.
pub fn render(document: &Din16798Snapshot, report: &CheckReport, selection: &NormSelection, frame: &LaneFrame, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    render_with(document, report, selection, &ViewportView { lanes: LaneOverlays::of_frame(frame, &interaction_domain_id(crate::VARIANT)), lane: frame.transient.view.clone(), ..ViewportView::default() }, locale)
}

/// 🎞️ The same window at an explicit view: the persisted local-only configuration and the ephemeral playback state, with the field picker and playback toolbar above the surface; the grips of the selection are resolved here from the projected scene.
pub fn render_with(document: &Din16798Snapshot, report: &CheckReport, selection: &NormSelection, view: &ViewportView, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let scene = scene_of(document, report);
    let selected = selection.highlights(scene.entities.iter().map(|e| (e.entity.kind, e.entity.id.as_str())));
    let picked: Vec<EntityRef> = scene.entities.iter().filter(|e| selection.contains(&e.entity.id)).map(|e| e.entity.clone()).collect();
    let view = ViewportView { lanes: LaneOverlays { handles: handles_of(document, &picked), peer_handles: peer_handles_of(document, &scene, &view.lanes.frame), palette: insert_specs().into_iter().map(|spec| PaletteEntry { id: spec.id, label_en: spec.label_en, label_de: spec.label_de }).collect(), frame: view.lanes.frame.clone(), focused: view.lanes.focused.clone(), prefs: view.lanes.prefs }, ..view.clone() };
    render_viewport_panel(BODY_VIEWPORT, &scene, &view, &selected, &interaction_domain_id(crate::VARIANT), crate::CONTROLLER_ID, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
