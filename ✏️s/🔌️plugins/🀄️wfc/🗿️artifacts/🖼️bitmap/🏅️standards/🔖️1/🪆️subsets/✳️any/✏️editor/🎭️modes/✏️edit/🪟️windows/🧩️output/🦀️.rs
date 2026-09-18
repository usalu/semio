//! 🧩️ Bitmap editor — the OUTPUT window: the INFERRED bitmap, read-only. Its pixels come from the
//! app transient's solve cache, never from the document: the collapse is an inference and the
//! snapshot has no field to hold it. Before the first solve, and after a contradiction, the pane
//! renders the pin overlay over an empty canvas rather than a stale image.

use crate::editor::bitmap::modes::edit::windows::output::config::BitmapOutputWindowConfig;
use crate::editor::bitmap::transient::BitmapTransient;
use crate::schema::snapshot::decode_base64;
use crate::BitmapSnapshot;
use semio_framework_plugin::{scene_surface, ActionDefinition, ActionKind, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;

//#region 🔖️Constants
pub const WFC_BITMAP_WINDOW_OUTPUT: &str = "wfc-bitmap-output";
pub const BODY_KEY: &str = "wfc.bitmap.output";
const SURFACE_ID: &str = "wfc.bitmap.output";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::bitmap::create_bitmap_editor`.
pub fn definition() -> WindowKindDefinition {
    let mut definition = WindowKindDefinition {
        id: WFC_BITMAP_WINDOW_OUTPUT.into(),
        label: LocalizedLabel::native("Output", "Ausgabe"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "grid-3x3".into(),
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
        ActionDefinition::bounded_catalog("solve", LocalizedLabel::native("Solve", "Berechnen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("resize-output", LocalizedLabel::native("Resize Output", "Ausgabe skalieren"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("pin-pixel", LocalizedLabel::native("Pin Pixel", "Pixel anheften"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("unpin-pixel", LocalizedLabel::native("Unpin Pixel", "Pixel lösen"), ActionKind::Mutation),
    ]);
    for action in &mut definition.actions {
        action.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
    }
    definition
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🎬️ Encodes the inferred bitmap behind the semantic surface contract. A cached solve whose extent
/// no longer matches the document's own output spec is DISCARDED rather than reshaped: a stale
/// buffer stretched over a new extent would look like a real answer.
pub fn render(document: &BitmapSnapshot, transient: &BitmapTransient, config: &BitmapOutputWindowConfig) -> UiAssemblyResult<BuiltNode> {
    let fresh = transient.output_width == document.output.width && transient.output_height == document.output.height;
    let indices = if fresh { transient.output_pixels.as_deref().and_then(decode_base64).unwrap_or_default() } else { Vec::new() };
    let pins: &[crate::schema::snapshot::BitmapPinnedPixel] = if config.show_pins { &document.pinned } else { &[] };
    let layers = crate::bitmap_layers_json("out", document.output.width, document.output.height, &document.input.palette, &indices, pins);
    let scene = Canvas2dScene::base(0.0, 0.0, config.zoom, layers);
    scene_surface(SURFACE_ID, ContractSurfaceKind::Canvas2d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
