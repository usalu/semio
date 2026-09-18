//! 🔲️ 2D-grid editor — the `grid` window: the interactive `Canvas2d` pane over the authored grid.
//! One bounds layer per cell, drawn at the authored cell size, each layer stating whether the cell
//! is OPEN, PINNED (to which tile) or MASKED. A pointer press runs the armed utility: `pin` writes
//! the active tile into the cell, `mask` cuts it out, `select` only picks.
//!
//! `Board2d` was the original surface and is unreachable in a playground: `Board2dHost` resolves a
//! per-app wasm board session through `resolveAppSurfaceSessionFactory`, and the only registered
//! factory in the whole repo is puzzle's own `BoardSession` cdylib, so every foreign `Board2d` pane
//! throws *"The current app has no registered board session factory."* before it paints a pixel.
//! `Canvas2d` needs no such session — the same host the preview pane already draws through — and a
//! grid of equal cells is a pure integer division away from a pointer sample, so nothing is lost.

use crate::editor::grid2d::window::{effective_camera, Grid2dWindowConfig};
use crate::schema::snapshot::Grid2dSnapshot;
use semio_framework_plugin::{ActionDefinition, ActionKind, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "wfc-grid2d-grid";
pub const BODY_KEY: &str = "wfc.grid2d.grid";
pub const SURFACE_ID: &str = "wfc.grid2d.surface.grid";

/// 🧰️ The three utilities this pane arms, per window INSTANCE (`active_utility_by_window_id`).
pub const UTILITY_SELECT: &str = "select";
pub const UTILITY_PIN: &str = "pin";
pub const UTILITY_MASK: &str = "mask";

/// 🎨️ One colour per cell state, so a hole and a pin never read as the same square.
const COLOR_OPEN: &str = "#334155";
const COLOR_PINNED: &str = "#2563eb";
const COLOR_MASKED: &str = "#7f1d1d";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🖱️ The pointer/camera vocabulary `Canvas2dHost` dispatches at its surface unprompted. Every one
/// of them MUST be declared by the window kind that owns the surface: an undeclared id is dropped
/// by the shell with `refused: undeclared-action`, which turns every hover into a console fault.
pub const CANVAS_ACTIONS: [&str; 5] = ["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "setCamera"];

/// 🖱️ A host-dispatched pointer/camera verb — never a palette entry, since its arguments are
/// authored by the canvas, not by a person.
pub fn canvas_action(id: &str, label: LocalizedLabel) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::View) }
}

/// 🖱️ The five canvas verbs, labelled once for both panes that own a `Canvas2d` surface.
pub fn canvas_actions() -> Vec<ActionDefinition> {
    vec![
        canvas_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt")),
        canvas_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt")),
        canvas_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger gelöst")),
        canvas_action("canvasDoubleClick", LocalizedLabel::native("Canvas Double Click", "Leinwand-Doppelklick")),
        canvas_action("setCamera", LocalizedLabel::native("Sync Camera", "Kamera abgleichen")),
    ]
}

/// 🧱️ Stitched into the app manifest by `crate::editor::grid2d::create_grid2d_editor`.
pub fn definition() -> WindowKindDefinition {
    let mut actions = vec![
        ActionDefinition::bounded_catalog("create-tile", LocalizedLabel::native("Create Tile", "Kachel erstellen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("delete-tile", LocalizedLabel::native("Delete Tile", "Kachel löschen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("change-tile-weight", LocalizedLabel::native("Change Tile Weight", "Kachelgewicht ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("change-tile-media", LocalizedLabel::native("Change Tile Media", "Kachelbild ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("create-rule", LocalizedLabel::native("Create Rule", "Regel erstellen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("delete-rule", LocalizedLabel::native("Delete Rule", "Regel löschen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("resize-grid", LocalizedLabel::native("Resize Grid", "Raster ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("change-cell-size", LocalizedLabel::native("Change Cell Size", "Zellgröße ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("change-periodicity", LocalizedLabel::native("Change Periodicity", "Periodizität ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("change-seed", LocalizedLabel::native("Change Seed", "Seed ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("pin-cell", LocalizedLabel::native("Pin Cell", "Zelle fixieren"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("unpin-cell", LocalizedLabel::native("Unpin Cell", "Zelle lösen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("mask-cell", LocalizedLabel::native("Mask Cell", "Zelle ausblenden"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("unmask-cell", LocalizedLabel::native("Unmask Cell", "Zelle einblenden"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("pick-cell", LocalizedLabel::native("Pick Cell", "Zelle wählen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("set-active-tile", LocalizedLabel::native("Set Active Tile", "Aktive Kachel setzen"), ActionKind::View),
        ActionDefinition::bounded_catalog("set-camera", LocalizedLabel::native("Set Camera", "Kamera setzen"), ActionKind::View),
        ActionDefinition::bounded_catalog("set-grid-visible", LocalizedLabel::native("Show Grid", "Raster zeigen"), ActionKind::View),
        ActionDefinition::bounded_catalog("set-grid-snap-enabled", LocalizedLabel::native("Snap To Grid", "Am Raster fangen"), ActionKind::View),
        ActionDefinition::bounded_catalog("set-grid-factor", LocalizedLabel::native("Set Grid Factor", "Rasterweite setzen"), ActionKind::View),
    ];
    actions.extend(canvas_actions());
    for action in &mut actions {
        action.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
    }
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Grid", "Raster"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "layout-grid".into(),
        options: WindowOptions::default(),
        actions,
        utilities: vec![UTILITY_SELECT.into(), UTILITY_PIN.into(), UTILITY_MASK.into()],
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Picking
/// 🪪️ Whether a dispatched `surfaceId` names THIS pane. The React host stamps its scene nodes with
/// `window:<window-kind-id>` rather than the id the guest published, so a press is recognised by
/// either spelling instead of silently resolving to nothing in the browser while passing in tests.
pub fn owns_surface(surface_id: &str) -> bool {
    surface_id == SURFACE_ID || surface_id.ends_with(WINDOW_KIND_ID)
}

/// 🎯️ The cell under one `canvasPointerDown` sample. `x`/`y` are CSS pixels inside the surface and
/// `width`/`height` its logical viewport, so the inverse of `Canvas2dHost`'s own
/// `screenToWorldLogical` is the whole conversion; a sample outside the authored grid answers
/// `None` rather than clamping onto an edge cell a person never aimed at.
pub fn cell_at(document: &Grid2dSnapshot, config: &Grid2dWindowConfig, x: f64, y: f64, viewport_width: f64, viewport_height: f64) -> Option<(u32, u32)> {
    let (camera_x, camera_y, zoom) = effective_camera(document, config);
    if !(x.is_finite() && y.is_finite() && viewport_width > 0.0 && viewport_height > 0.0) {
        return None;
    }
    let world_x = (x - viewport_width * 0.5) / zoom + camera_x;
    let world_y = (y - viewport_height * 0.5) / zoom + camera_y;
    let cell_width = if document.cell_width > 0.0 { document.cell_width } else { 1.0 };
    let cell_height = if document.cell_height > 0.0 { document.cell_height } else { 1.0 };
    let column = (world_x / cell_width).floor();
    let row = (world_y / cell_height).floor();
    if column < 0.0 || row < 0.0 || column >= f64::from(document.width) || row >= f64::from(document.height) {
        return None;
    }
    Some((column as u32, row as u32))
}
//#endregion 🔖️Picking

//#region 🔖️Render
/// 🎛️ Which state one cell is in — the layer's colour, and what an author reads at a glance
/// without opening a panel.
pub fn cell_kind(document: &Grid2dSnapshot, x: u32, y: u32) -> &'static str {
    if document.masked.iter().any(|cell| cell.x == x && cell.y == y) {
        "masked"
    } else if document.pinned.iter().any(|cell| cell.x == x && cell.y == y) {
        "pinned"
    } else {
        "open"
    }
}

/// 🎨️ The colour a cell state paints with.
pub fn cell_color(kind: &str) -> &'static str {
    match kind {
        "pinned" => COLOR_PINNED,
        "masked" => COLOR_MASKED,
        _ => COLOR_OPEN,
    }
}

/// 🧱️ The canvas layer list: one bounds layer per cell at the authored cell size, so the pane's
/// geometry and `cell_at`'s integer division are the same grid. A pinned cell carries its tile id
/// as the layer's `name` (the only text `Canvas2dHost` draws over a bounds layer) and is `selected`
/// so the host outlines it; a masked cell is a red hole with no text.
pub fn layers_json(document: &Grid2dSnapshot) -> String {
    let mut layers: Vec<Value> = Vec::with_capacity((document.width as usize) * (document.height as usize));
    for y in 0..document.height {
        for x in 0..document.width {
            let kind = cell_kind(document, x, y);
            let name = document.pinned.iter().find(|cell| cell.x == x && cell.y == y).map_or_else(String::new, |cell| cell.tile_id.clone());
            layers.push(json!({
                "id": format!("cell-{x}-{y}"),
                "x": f64::from(x) * document.cell_width,
                "y": f64::from(y) * document.cell_height,
                "width": document.cell_width,
                "height": document.cell_height,
                "color": cell_color(kind),
                "name": name,
                "selected": kind == "pinned",
            }));
        }
    }
    Value::Array(layers).to_string()
}

/// 🖼️ The canvas-2d scene this pane publishes. Public so a law can assert what reaches the client
/// without decoding a rendered surface node.
pub fn scene(document: &Grid2dSnapshot, config: &Grid2dWindowConfig, _active_utility: &str) -> Canvas2dScene {
    let (camera_x, camera_y, zoom) = effective_camera(document, config);
    Canvas2dScene::base(camera_x, camera_y, zoom, layers_json(document))
}

pub fn render(document: &Grid2dSnapshot, config: &Grid2dWindowConfig, active_utility: &str) -> UiAssemblyResult<BuiltNode> {
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Canvas2d, &scene(document, config, active_utility))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
