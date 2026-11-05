//! 👁️ Read-only preview projects admitted current cache and authored pins without solving during render.

use crate::editor::wfc3d::modes::edit::windows::preview as editor_preview;
use crate::Wfc3dSnapshot;
use semio_ui_contract::SurfaceKind as ContractSurfaceKind;
use semio_plugin::scene_surface;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::SurfaceKind;
use semio_plugin::UiAssemblyResult;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowOptions;
use semio_plugin::World3dScene;
use semio_plugin::world3d_selection_json;

//#region 🔖️Constants
pub const WFC_3D_VIEW_WINDOW: &str = "wfc-3d-view";
pub const WFC_3D_VIEW_BODY: &str = "wfc.3d.view";
const WFC_3D_VIEW_SURFACE: &str = "wfc.3d.view";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: WFC_3D_VIEW_WINDOW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: WFC_3D_VIEW_BODY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "box".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Wfc3dSnapshot, cache: &crate::editor::wfc3d::transient::Wfc3dTransient, revision: Option<&[u8;32]>, locale: semio_framework_ui_locale::Locale, terminology: semio_framework_ui_locale::Terminology) -> UiAssemblyResult<BuiltNode> {
    use semio_framework_ui_contract::*;
    let empty = crate::editor::wfc3d::transient::Wfc3dTransient::default();
    let transient = if crate::editor::wfc3d::transient::source_matches(&cache.source_revision, revision) { cache } else { &empty };
    let scene = World3dScene {
        instances_delta_json: Some(editor_preview::instances_delta_json(document, transient)),
        ..World3dScene::base(editor_preview::camera_json(document, 1.0), editor_preview::meshes_json(document), editor_preview::instances_json(document, transient), world3d_selection_json("replace", &[], None))
    };
    let scene = scene_surface(WFC_3D_VIEW_SURFACE, ContractSurfaceKind::World3d, &scene)?;
    let label = if document.slots.is_empty() { LocalizedLabel::native("No slots authored yet.", "Noch keine Plätze angelegt.") }
        else if transient.contradiction { LocalizedLabel::native("No assignment satisfies the current rules.", "Keine Belegung erfüllt die aktuellen Regeln.") }
        else if !transient.assignments.is_empty() { LocalizedLabel::native("Showing the current result.", "Das aktuelle Ergebnis wird angezeigt.") }
        else if document.slots.iter().any(|slot| slot.pinned_tile_id.is_some()) { LocalizedLabel::native("Showing authored pins. No current solve is available.", "Angelegte Fixierungen werden angezeigt. Kein aktuelles Ergebnis verfügbar.") }
        else { LocalizedLabel::native("No current result. Run Fill in the editor.", "Kein aktuelles Ergebnis. Füllen im Editor starten.") };
    let message = label.resolve(terminology, locale);
    let caption = text(Label::try_from(message).map_err(|_| semio_framework_plugin::ui_assembly_error("wfc3d-view.status.label"))?)
        .try_id("wfc.3d.view.status").map_err(|_| semio_framework_plugin::ui_assembly_error("wfc3d-view.status.id"))?
        .try_label(message).map_err(|_| semio_framework_plugin::ui_assembly_error("wfc3d-view.status.aria"))?
        .try_build().map_err(|_| semio_framework_plugin::ui_assembly_error("wfc3d-view.status.build"))?;
    column().try_id("wfc.3d.view.body").map_err(|_| semio_framework_plugin::ui_assembly_error("wfc3d-view.body.id"))?
        .try_children(vec![caption, scene]).map_err(|_| semio_framework_plugin::ui_assembly_error("wfc3d-view.body.children"))?
        .try_build().map_err(|_| semio_framework_plugin::ui_assembly_error("wfc3d-view.body.build"))
}

//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
