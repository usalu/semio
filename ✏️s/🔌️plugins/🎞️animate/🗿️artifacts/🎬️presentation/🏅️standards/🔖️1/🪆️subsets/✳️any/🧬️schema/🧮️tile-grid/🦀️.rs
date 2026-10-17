//! 🧮️ Presentation tile grid — pure geometry that splits a figure frame into tile crops and seeds tile drafts from a grid; reachable from the schema tier (`📸️snapshot`, `🧬️mutations`) and from app commands alike.

//#region 🎞️TilePlay
/// 🌱️ Relocated verbatim from the former artifact-tree `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): every function here is a pure `(&Snapshot-
/// adjacent structs) -> value` transform with no `&mut self`, no app coupling, and no I/O — reachable
/// from BOTH schema-tier DSL/grammar hooks (`📸️snapshot/📝️text`, `🧬️mutations/📝️text`) and app-tier
/// commands, so it belongs at this shared schema tier rather than in either dependent.
pub const NORMALIZED_RECT_MIN_FRACTION: f64 = 0.02;

#[derive(Clone, Copy)]
pub struct SplitFigureGridSpec<'a> {
    pub rows: u32,
    pub columns: u32,
    pub frame: &'a crate::FigureTileFrame,
    pub gap: f64,
    pub key_prefix: &'a str,
}

pub struct SplitGridCell {
    pub key: String,
    pub crop: crate::FigureTileFrame,
}

#[derive(Clone, Copy)]
pub struct FigureTileGridSeedSpec<'a> {
    pub source: &'a crate::FigureTileSource,
    pub rows: u32,
    pub columns: u32,
    pub gap: f64,
    pub key_prefix: &'a str,
}

pub fn clamp_normalized_fraction(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

pub fn clamp_tile_crop(crop: &crate::FigureTileFrame) -> crate::FigureTileFrame {
    let width = crop.width.max(NORMALIZED_RECT_MIN_FRACTION);
    let height = crop.height.max(NORMALIZED_RECT_MIN_FRACTION);
    let x = clamp_normalized_fraction(crop.x.min(1.0 - width));
    let y = clamp_normalized_fraction(crop.y.min(1.0 - height));
    crate::FigureTileFrame { x, y, width, height }
}

pub fn parse_grid_engagement(text: &str) -> Option<(u32, u32)> {
    let trimmed = text.trim();
    let lower = trimmed.to_lowercase();
    let normalized = lower.replace('×', "x");
    let parts: Vec<&str> = normalized.split('x').map(str::trim).collect();
    if parts.len() != 2 {
        return None;
    }
    let rows: u32 = parts[0].parse().ok()?;
    let columns: u32 = parts[1].parse().ok()?;
    if rows < 1 || columns < 1 {
        return None;
    }
    Some((rows, columns))
}

pub fn split_figure_grid(spec: SplitFigureGridSpec<'_>) -> Vec<SplitGridCell> {
    let rows = spec.rows.max(1);
    let columns = spec.columns.max(1);
    let gap = spec.gap;
    let frame = spec.frame;
    let cell_width = (frame.width - gap * (columns as f64 - 1.0)) / columns as f64;
    let cell_height = (frame.height - gap * (rows as f64 - 1.0)) / rows as f64;
    let crop_width = frame.width / columns as f64;
    let crop_height = frame.height / rows as f64;
    let mut cells = Vec::new();
    for row in 0..rows {
        for column in 0..columns {
            cells.push(SplitGridCell { key: format!("{}-r{row}-c{column}", spec.key_prefix), crop: crate::FigureTileFrame { x: frame.x + column as f64 * crop_width, y: frame.y + row as f64 * crop_height, width: crop_width, height: crop_height } });
        }
    }
    let _ = (cell_width, cell_height);
    cells
}

pub fn populate_tile_drafts_from_grid(spec: FigureTileGridSeedSpec<'_>) -> Vec<crate::FigureTileDraft> {
    split_figure_grid(SplitFigureGridSpec { rows: spec.rows, columns: spec.columns, frame: &spec.source.frame, gap: spec.gap, key_prefix: spec.key_prefix })
        .into_iter()
        .map(|cell| crate::FigureTileDraft { id: cell.key.clone(), name: cell.key, crop: cell.crop })
        .collect()
}
//#endregion 🎞️TilePlay
