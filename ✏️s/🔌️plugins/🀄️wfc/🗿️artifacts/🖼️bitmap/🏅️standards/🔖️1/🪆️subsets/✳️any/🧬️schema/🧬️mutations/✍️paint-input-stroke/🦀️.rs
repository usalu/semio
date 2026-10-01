//! ✍️ Bitmap mutation — `PaintInputStroke`: one parametric brush stroke over the input sample. The
//! gesture's own inputs ARE the payload — the sampled cells in drawing order and the palette index the
//! brush paints — so a history edit of the stroke (another colour, another path) re-derives every
//! painted cell from whatever base it replays on. Consecutive points are joined by an inclusive
//! Bresenham line, so a fast drag that skips cells still paints a connected stroke.

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️PaintInputStroke
/// 🧮️ The most points one stroke carries — a host flushes a longer drag as several strokes.
pub const BITMAP_STROKE_MAXIMUM_POINTS: usize = 4_096;

/// 📍️ One sampled cell of a stroke, in input-bitmap cells.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapStrokePoint {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PaintInputStroke {
    pub points: Vec<BitmapStrokePoint>,
    pub color: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn paint_input_stroke(points: Vec<BitmapStrokePoint>, color: u32) -> BitmapMutation {
    BitmapMutation::PaintInputStroke(PaintInputStroke { points, color })
}

/// 〰️ Every cell the stroke covers, each once, in drawing order: the first point, then an inclusive
/// Bresenham line to every following point.
pub fn stroke_cells(points: &[BitmapStrokePoint]) -> Vec<BitmapStrokePoint> {
    let mut cells: Vec<BitmapStrokePoint> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut visit = |cell: BitmapStrokePoint, cells: &mut Vec<BitmapStrokePoint>| {
        if seen.insert((cell.x, cell.y)) {
            cells.push(cell);
        }
    };
    let Some(first) = points.first() else { return cells };
    visit(*first, &mut cells);
    for pair in points.windows(2) {
        let (mut x, mut y) = (i64::from(pair[0].x), i64::from(pair[0].y));
        let (end_x, end_y) = (i64::from(pair[1].x), i64::from(pair[1].y));
        let (dx, dy) = ((end_x - x).abs(), -(end_y - y).abs());
        let (step_x, step_y) = (if x < end_x { 1 } else { -1 }, if y < end_y { 1 } else { -1 });
        let mut error = dx + dy;
        loop {
            visit(BitmapStrokePoint { x: x as u32, y: y as u32 }, &mut cells);
            if x == end_x && y == end_y {
                break;
            }
            let doubled = 2 * error;
            if doubled >= dy {
                error += dy;
                x += step_x;
            }
            if doubled <= dx {
                error += dx;
                y += step_y;
            }
        }
    }
    cells
}

/// 🔲️ The inclusive bounding box `(x, y, width, height)` of the stroke cells inside a `width × height`
/// sample, with those cells; `None` when no cell lies inside.
pub fn stroke_extent(points: &[BitmapStrokePoint], width: u32, height: u32) -> Option<((u32, u32, u32, u32), Vec<BitmapStrokePoint>)> {
    let inside: Vec<BitmapStrokePoint> = stroke_cells(points).into_iter().filter(|cell| cell.x < width && cell.y < height).collect();
    let min_x = inside.iter().map(|cell| cell.x).min()?;
    let min_y = inside.iter().map(|cell| cell.y).min()?;
    let max_x = inside.iter().map(|cell| cell.x).max()?;
    let max_y = inside.iter().map(|cell| cell.y).max()?;
    Some(((min_x, min_y, max_x - min_x + 1, max_y - min_y + 1), inside))
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for PaintInputStroke {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "paint", entity: "input-stroke", kind: "paint-input-stroke", record: "PaintedInputStroke" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let cells = stroke_cells(&self.points).len();
        let (en, de) = if cells == 1 { ("1 cell".to_string(), "1 Zelle".to_string()) } else { (format!("{cells} cells"), format!("{cells} Zellen")) };
        protocol::LocalizedLabel::native(&format!("Paint stroke of {en} in colour {}", self.color), &format!("Strich mit {de} in Farbe {} malen", self.color))
    }
}
//#endregion 🔖️PaintInputStroke
