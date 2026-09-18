//! 🖼️ Bitmap editor — the INPUT window: the authored sample bitmap, painted directly. A canvas-2d
//! surface whose layers are run-merged pixel rectangles filled with the document's own palette
//! colours, so what the pane shows is the exact index buffer the overlapping model will learn from.
//!
//! Why canvas-2d and not paint-2d: `Paint2dScene`'s host contract is a raster DOCUMENT (layer tree
//! plus base64 PNG assets in `assetsJson`), which would force this artifact to carry a PNG encoder
//! purely to display a palette-indexed buffer it already holds exactly. The canvas host's own layer
//! record renders a filled path with an explicit `fill.color`, which reproduces palette indices
//! byte-for-byte with no encoder at all.

use crate::editor::bitmap::modes::edit::windows::input::config::BitmapInputWindowConfig;
use crate::BitmapSnapshot;
use semio_framework_plugin::{scene_surface, ActionDefinition, ActionKind, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;

//#region 🔖️Constants
pub const WFC_BITMAP_WINDOW_INPUT: &str = "wfc-bitmap-input";
pub const BODY_KEY: &str = "wfc.bitmap.input";
const SURFACE_ID: &str = "wfc.bitmap.input";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::bitmap::create_bitmap_editor`.
pub fn definition() -> WindowKindDefinition {
    let mut definition = WindowKindDefinition {
        id: WFC_BITMAP_WINDOW_INPUT.into(),
        label: LocalizedLabel::native("Input", "Eingabe"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "image".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    };
    definition.actions.extend([
        ActionDefinition::bounded_catalog("set-input-pixels", LocalizedLabel::native("Paint Pixels", "Pixel malen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("resize-input", LocalizedLabel::native("Resize Input", "Eingabe skalieren"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("add-palette-color", LocalizedLabel::native("Add Palette Colour", "Palettenfarbe hinzufügen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("change-palette-color", LocalizedLabel::native("Change Palette Colour", "Palettenfarbe ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("remove-palette-color", LocalizedLabel::native("Remove Palette Colour", "Palettenfarbe entfernen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("set-active-color", LocalizedLabel::native("Set Active Colour", "Aktive Farbe setzen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("stroke-begin", LocalizedLabel::native("Begin Stroke", "Strich beginnen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("stroke-extend", LocalizedLabel::native("Extend Stroke", "Strich fortsetzen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("stroke-commit", LocalizedLabel::native("Commit Stroke", "Strich übernehmen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("change-model", LocalizedLabel::native("Change Model", "Modell ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("change-seed", LocalizedLabel::native("Change Seed", "Seed ändern"), ActionKind::Mutation),
    ]);
    for action in &mut definition.actions {
        action.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
    }
    definition
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🎬️ Encodes the authored sample behind the semantic surface contract. A document whose pixel
/// buffer does not decode renders as an EMPTY layer list rather than a plausible-looking wrong
/// image — the honest answer while the only way to reach that state is a hand-edited file.
pub fn render(document: &BitmapSnapshot, config: &BitmapInputWindowConfig) -> UiAssemblyResult<BuiltNode> {
    let indices = document.input.indices().unwrap_or_default();
    let layers = crate::bitmap_layers_json("in", document.input.width, document.input.height, &document.input.palette, &indices, &[]);
    let scene = Canvas2dScene::base(0.0, 0.0, config.zoom, layers);
    scene_surface(SURFACE_ID, ContractSurfaceKind::Canvas2d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
