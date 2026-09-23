//! 🧩 Bitmap editor — the OUTPUT window: the INFERRED bitmap, read-only. While a fill tool run is
//! live its tick payload is painted (with an explicit decided mask). Otherwise the pane reads the
//! SetSolve transient. A contradiction paints a labelled empty canvas rather than a silent black square.

use crate::editor::bitmap::modes::edit::tools::fill::{self as fill_tool, BitmapFillPayload};
use crate::editor::bitmap::modes::edit::windows::output::config::BitmapOutputWindowConfig;
use crate::editor::bitmap::transient::BitmapTransient;
use crate::schema::snapshot::{decode_base64, BitmapColor};
use crate::BitmapSnapshot;
use semio_framework_plugin::{scene_surface, ActionArgDef, ActionDefinition, ActionKind, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, ToolRunView, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;

//#region 🔖️Constants
pub const WFC_BITMAP_WINDOW_OUTPUT: &str = "wfc-bitmap-output";
pub const BODY_KEY: &str = "wfc.bitmap.output";
const SURFACE_ID: &str = "wfc.bitmap.output";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧩 Stitched into the editor manifest by `crate::editor::bitmap::create_bitmap_editor`.
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
        ActionDefinition::bounded_catalog("commit-fill-solve", LocalizedLabel::native("Commit Fill Solve", "Füllen übernehmen"), ActionKind::Mutation),
        ActionDefinition {
            args: vec![
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required().default_value(&24.0),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required().default_value(&24.0),
                ActionArgDef::toggle("periodic", LocalizedLabel::native("Periodic", "Periodisch")).default_value(&true),
            ],
            ..ActionDefinition::bounded_catalog("resize-output", LocalizedLabel::native("Resize Output", "Ausgabe skalieren"), ActionKind::Mutation)
        },
        ActionDefinition {
            args: vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required().default_value(&0.0),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required().default_value(&0.0),
                ActionArgDef::number("color", LocalizedLabel::native("Colour", "Farbe")).required().default_value(&0.0),
            ],
            ..ActionDefinition::bounded_catalog("pin-pixel", LocalizedLabel::native("Pin Pixel", "Pixel anheften"), ActionKind::Mutation)
        },
        ActionDefinition {
            args: vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required().default_value(&0.0),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required().default_value(&0.0),
            ],
            ..ActionDefinition::bounded_catalog("unpin-pixel", LocalizedLabel::native("Unpin Pixel", "Pixel lösen"), ActionKind::Mutation)
        },
    ]);
    for action in &mut definition.actions {
        action.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
    }
    definition
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🎬️ Encodes the inferred bitmap behind the semantic surface contract. A live fill payload wins over
/// the finished SetSolve cache. A cached solve whose extent no longer matches the document's own
/// output spec is DISCARDED rather than reshaped.
pub fn render(document: &BitmapSnapshot, transient: &BitmapTransient, config: &BitmapOutputWindowConfig, tool_run: Option<&ToolRunView>) -> UiAssemblyResult<BuiltNode> {
    let scene = Canvas2dScene::base(0.0, 0.0, config.zoom, render_layers_fingerprint(document, transient, config, tool_run));
    scene_surface(SURFACE_ID, ContractSurfaceKind::Canvas2d, &scene)
}

/// 🏃️ Non-terminal fill run whose tick payload still decodes.
pub fn live_fill_payload(tool_run: Option<&ToolRunView>) -> Option<BitmapFillPayload> {
    let run = tool_run.filter(|run| run.tool_id == fill_tool::TOOL_ID && !run.state.is_terminal())?;
    let bytes = run.payload.as_ref()?;
    let text = std::str::from_utf8(bytes).ok()?;
    BitmapFillPayload::decode_json(text)
}

/// 🎨 Paint decided cells, then the burst. A collapse paints its colour even before the cell is
/// singleton. A discard paints that tried colour as a marker, including when the search has already
/// undone it, so the thinking stays on screen.
pub fn layers_from_fill_payload(document: &BitmapSnapshot, payload: &BitmapFillPayload, pins: &[crate::schema::snapshot::BitmapPinnedPixel]) -> String {
    let (indices, mask) = payload.render_indices();
    let cells = (payload.width as usize).saturating_mul(payload.height as usize);
    if indices.len() != cells || mask.len() != cells || payload.width == 0 {
        return crate::bitmap_layers_json("out", document.output.width, document.output.height, &document.input.palette, &[], pins);
    }
    let mut palette = document.input.palette.clone();
    let empty = u8::try_from(palette.len()).unwrap_or(u8::MAX);
    palette.push(BitmapColor { r: 0, g: 0, b: 0, a: 0 });
    let discard = u8::try_from(palette.len()).unwrap_or(u8::MAX);
    palette.push(BitmapColor { r: 255, g: 40, b: 160, a: 255 });
    let mut painted = vec![empty; cells];
    for (index, decided) in mask.iter().enumerate() {
        if *decided {
            painted[index] = indices[index];
        }
    }
    let mut markers = pins.to_vec();
    let mut undone = vec![false; cells];
    for event in &payload.trace {
        let index = event.index as usize;
        if index >= cells {
            continue;
        }
        if event.color <= u32::from(u8::MAX) {
            painted[index] = event.color as u8;
        }
        undone[index] = event.discarded;
    }
    for (index, discarded) in undone.iter().enumerate() {
        if *discarded {
            let x = index as u32 % payload.width;
            let y = index as u32 / payload.width;
            markers.push(crate::schema::snapshot::BitmapPinnedPixel { x, y, color: u32::from(discard) });
        }
    }
    crate::bitmap_layers_json("out", payload.width, payload.height, &palette, &painted, &markers)
}

/// 🩺 Contradiction answer: extent plus a distinct overlay id the empty canvas does not carry.
pub fn contradiction_layers(document: &BitmapSnapshot, pins: &[crate::schema::snapshot::BitmapPinnedPixel]) -> String {
    let mut layers = crate::bitmap_layers_json("out", document.output.width, document.output.height, &document.input.palette, &[], pins);
    if layers.ends_with(']') {
        layers.pop();
        if layers.len() > 1 && !layers.ends_with('[') {
            layers.push(',');
        }
        layers.push_str(r#"{"id":"out-contradiction","kind":"path","segments":[],"fill":{"kind":"solid","color":[0.85,0.2,0.2,0.35]}}]"#);
    }
    layers
}

/// 🧪 Layer JSON fingerprint used by tests to tell empty, partial and finished paints apart.
pub fn render_layers_fingerprint(document: &BitmapSnapshot, transient: &BitmapTransient, config: &BitmapOutputWindowConfig, tool_run: Option<&ToolRunView>) -> String {
    let pins: &[crate::schema::snapshot::BitmapPinnedPixel] = if config.show_pins { &document.pinned } else { &[] };
    if let Some(payload) = live_fill_payload(tool_run) {
        return layers_from_fill_payload(document, &payload, pins);
    }
    if transient.contradiction && transient.output_width == document.output.width && transient.output_height == document.output.height && transient.output_pixels.is_none() {
        return contradiction_layers(document, pins);
    }
    let fresh = transient.output_width == document.output.width && transient.output_height == document.output.height;
    let indices = if fresh { transient.output_pixels.as_deref().and_then(decode_base64).unwrap_or_default() } else { Vec::new() };
    crate::bitmap_layers_json("out", document.output.width, document.output.height, &document.input.palette, &indices, pins)
}
//#endregion 🔖️Render

//#region 🧪Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪Tests
