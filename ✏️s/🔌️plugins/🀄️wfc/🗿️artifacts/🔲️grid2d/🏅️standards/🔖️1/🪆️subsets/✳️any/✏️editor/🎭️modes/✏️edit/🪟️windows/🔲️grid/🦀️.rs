//! 🔲️ 2D-grid editor — the `grid` window: the interactive `Board2d` pane over the authored grid.
//! One board node per cell, snapped to the authored cell size (`grid_factor` = `cellWidth`), each
//! node stating whether the cell is OPEN, PINNED (to which tile) or MASKED. A click runs the armed
//! utility: `pin` writes the active tile into the cell, `mask` cuts it out, `select` only picks.

use crate::editor::grid2d::window::Grid2dWindowConfig;
use crate::schema::snapshot::Grid2dSnapshot;
use semio_framework_plugin::{ActionDefinition, ActionKind, Board2dScene, BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "wfc-grid2d-grid";
pub const BODY_KEY: &str = "wfc.grid2d.grid";
pub const SURFACE_ID: &str = "wfc.grid2d.surface.grid";

/// 🧰️ The three utilities this pane arms, per window INSTANCE (`active_utility_by_window_id`).
pub const UTILITY_SELECT: &str = "select";
pub const UTILITY_PIN: &str = "pin";
pub const UTILITY_MASK: &str = "mask";
//#endregion 🔖️Constants

//#region 🔖️Definition
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
        ActionDefinition::bounded_catalog("set-active-tile", LocalizedLabel::native("Set Active Tile", "Aktive Kachel setzen"), ActionKind::View),
        ActionDefinition::bounded_catalog("set-camera", LocalizedLabel::native("Set Camera", "Kamera setzen"), ActionKind::View),
        ActionDefinition::bounded_catalog("set-grid-visible", LocalizedLabel::native("Show Grid", "Raster zeigen"), ActionKind::View),
        ActionDefinition::bounded_catalog("set-grid-snap-enabled", LocalizedLabel::native("Snap To Grid", "Am Raster fangen"), ActionKind::View),
        ActionDefinition::bounded_catalog("set-grid-factor", LocalizedLabel::native("Set Grid Factor", "Rasterweite setzen"), ActionKind::View),
    ];
    for action in &mut actions {
        action.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
    }
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Grid", "Raster"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Board2d,
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

//#region 🔖️Render
/// 🎛️ Which state one cell is in — the board node's `nodeKind`, and what an author reads at a
/// glance without opening a panel.
pub fn cell_kind(document: &Grid2dSnapshot, x: u32, y: u32) -> &'static str {
    if document.masked.iter().any(|cell| cell.x == x && cell.y == y) {
        "masked"
    } else if document.pinned.iter().any(|cell| cell.x == x && cell.y == y) {
        "pinned"
    } else {
        "open"
    }
}

/// 🎲️ The board fixture: one rectangle node per cell, placed at the authored cell size so the
/// board's own snap grid and the document's grid are the same grid.
pub fn fixture_json(document: &Grid2dSnapshot) -> String {
    let mut nodes = Vec::with_capacity((document.width as usize) * (document.height as usize));
    for y in 0..document.height {
        for x in 0..document.width {
            let kind = cell_kind(document, x, y);
            let text = document.pinned.iter().find(|cell| cell.x == x && cell.y == y).map_or_else(String::new, |cell| cell.tile_id.clone());
            nodes.push(serde_json::json!({
                "id": format!("cell-{x}-{y}"),
                "nodeKind": kind,
                "shape": "rectangle",
                "x": f64::from(x) * document.cell_width + document.cell_width * 0.5,
                "y": f64::from(y) * document.cell_height + document.cell_height * 0.5,
                "width": document.cell_width,
                "height": document.cell_height,
                "text": text,
            }));
        }
    }
    serde_json::json!({ "schema": crate::WFC_GRID2D_DOCUMENT_SCHEMA, "nodes": nodes, "edges": [] }).to_string()
}

/// 🛍️ The per-node glyph catalogue the board resolves `nodeKind` through — one row per cell state,
/// so a masked hole and a pinned cell never read as the same square.
pub fn glyph_catalogs_json() -> String {
    serde_json::json!({
        "nodeKinds": [
            { "id": "open", "icon": "square-dashed", "color": "#334155" },
            { "id": "pinned", "icon": "lock", "color": "#2563eb" },
            { "id": "masked", "icon": "eraser", "color": "#64748b" }
        ]
    })
    .to_string()
}

/// 🖼️ The board-2d scene this pane publishes. Public so a law can assert what reaches the client
/// without decoding a rendered surface node.
pub fn scene(document: &Grid2dSnapshot, config: &Grid2dWindowConfig, active_utility: &str) -> Board2dScene {
    Board2dScene {
        fixture_json: fixture_json(document),
        camera_json: serde_json::json!({ "x": config.camera_x, "y": config.camera_y, "zoom": config.camera_zoom }).to_string(),
        glyph_catalogs_json: glyph_catalogs_json(),
        selection_json: "[]".into(),
        interactive: true,
        hovered_id: None,
        active_utility: Some(active_utility.to_string()),
        selection_method: "rectangle".into(),
        grid_visible: config.grid_visible,
        grid_snap_enabled: config.grid_snap_enabled,
        // 📐️ The snap step IS the authored cell size — a board click therefore lands on exactly one
        // cell, which is what makes `pick-cell` a pure integer division.
        grid_factor: if config.grid_factor > 0.0 { config.grid_factor } else { document.cell_width.max(1.0) },
        selectable_nodes: true,
        selectable_edges: false,
        selectable_handles: false,
        suggestion_offset: 0.0,
        brush_weights_json: "{}".into(),
        placement_compatibility_json: "[]".into(),
        lod_mode: "automatic".into(),
        transform_flags: None,
        area_brush_size: None,
        domain_id: None,
        suggestion_menu_json: None,
        tool_run_trace: None,
        lanes: Vec::new(),
    }
}

pub fn render(document: &Grid2dSnapshot, config: &Grid2dWindowConfig, active_utility: &str) -> UiAssemblyResult<BuiltNode> {
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Board2d, &scene(document, config, active_utility))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
