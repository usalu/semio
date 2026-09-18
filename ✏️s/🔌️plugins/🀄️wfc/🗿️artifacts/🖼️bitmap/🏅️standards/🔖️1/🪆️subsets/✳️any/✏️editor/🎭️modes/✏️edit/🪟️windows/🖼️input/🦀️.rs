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
use semio_framework_plugin::{scene_surface, ActionArgDef, ActionDefinition, ActionKind, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;

//#region 🔖️Constants
pub const WFC_BITMAP_WINDOW_INPUT: &str = "wfc-bitmap-input";
pub const BODY_KEY: &str = "wfc.bitmap.input";
const SURFACE_ID: &str = "wfc.bitmap.input";
//#endregion 🔖️Constants

//#region 🔖️ActionArgs
/// 📝️ One sampled cell — the staged form both stroke verbs take.
fn cell_args() -> Vec<ActionArgDef> {
    vec![
        ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required().default_value(&0.0),
        ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required().default_value(&0.0),
    ]
}

/// 🎨️ One palette entry — index plus the four 0..255 channels.
fn palette_color_args() -> Vec<ActionArgDef> {
    vec![
        ActionArgDef::number("index", LocalizedLabel::native("Index", "Index")).required().default_value(&0.0),
        ActionArgDef::number("r", LocalizedLabel::native("Red", "Rot")).required().default_value(&0.0),
        ActionArgDef::number("g", LocalizedLabel::native("Green", "Grün")).required().default_value(&0.0),
        ActionArgDef::number("b", LocalizedLabel::native("Blue", "Blau")).required().default_value(&0.0),
        ActionArgDef::number("a", LocalizedLabel::native("Alpha", "Alpha")).required().default_value(&255.0),
    ]
}
//#endregion 🔖️ActionArgs

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
        ActionDefinition {
            args: vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required().default_value(&0.0),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required().default_value(&0.0),
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required().default_value(&1.0),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required().default_value(&1.0),
                ActionArgDef::text("pixels", LocalizedLabel::native("Pixels (base64)", "Pixel (Base64)")).required(),
            ],
            ..ActionDefinition::bounded_catalog("set-input-pixels", LocalizedLabel::native("Paint Pixels", "Pixel malen"), ActionKind::Mutation)
        },
        ActionDefinition {
            args: vec![
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required().default_value(&16.0),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required().default_value(&16.0),
            ],
            ..ActionDefinition::bounded_catalog("resize-input", LocalizedLabel::native("Resize Input", "Eingabe skalieren"), ActionKind::Mutation)
        },
        ActionDefinition { args: palette_color_args(), ..ActionDefinition::bounded_catalog("add-palette-color", LocalizedLabel::native("Add Palette Colour", "Palettenfarbe hinzufügen"), ActionKind::Mutation) },
        ActionDefinition { args: palette_color_args(), ..ActionDefinition::bounded_catalog("change-palette-color", LocalizedLabel::native("Change Palette Colour", "Palettenfarbe ändern"), ActionKind::Mutation) },
        ActionDefinition {
            args: vec![ActionArgDef::number("index", LocalizedLabel::native("Index", "Index")).required().default_value(&0.0)],
            ..ActionDefinition::bounded_catalog("remove-palette-color", LocalizedLabel::native("Remove Palette Colour", "Palettenfarbe entfernen"), ActionKind::Mutation)
        },
        ActionDefinition {
            args: vec![ActionArgDef::number("index", LocalizedLabel::native("Colour", "Farbe")).required().default_value(&0.0)],
            ..ActionDefinition::bounded_catalog("set-active-color", LocalizedLabel::native("Set Active Colour", "Aktive Farbe setzen"), ActionKind::Mutation)
        },
        ActionDefinition { args: cell_args(), ..ActionDefinition::bounded_catalog("stroke-begin", LocalizedLabel::native("Begin Stroke", "Strich beginnen"), ActionKind::Mutation) },
        ActionDefinition { args: cell_args(), ..ActionDefinition::bounded_catalog("stroke-extend", LocalizedLabel::native("Extend Stroke", "Strich fortsetzen"), ActionKind::Mutation) },
        ActionDefinition::bounded_catalog("stroke-commit", LocalizedLabel::native("Commit Stroke", "Strich übernehmen"), ActionKind::Mutation),
        ActionDefinition {
            args: vec![
                ActionArgDef::number("patternSize", LocalizedLabel::native("Pattern Size", "Mustergröße")).required().default_value(&3.0),
                ActionArgDef::number("symmetry", LocalizedLabel::native("Symmetry", "Symmetrie")).required().default_value(&8.0),
                ActionArgDef::toggle("periodicInput", LocalizedLabel::native("Periodic Input", "Periodische Eingabe")).default_value(&true),
                ActionArgDef::number("ground", LocalizedLabel::native("Ground", "Boden")),
            ],
            ..ActionDefinition::bounded_catalog("change-model", LocalizedLabel::native("Change Model", "Modell ändern"), ActionKind::Mutation)
        },
        ActionDefinition {
            args: vec![ActionArgDef::number("seed", LocalizedLabel::native("Seed", "Seed")).required().default_value(&1.0)],
            ..ActionDefinition::bounded_catalog("change-seed", LocalizedLabel::native("Change Seed", "Seed ändern"), ActionKind::Mutation)
        },
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
