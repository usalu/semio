//! 🗂️ tui state, key, pointer and paint functions for the Table element, wired as a crate-root sibling module of
//! `crate::tui::widget` (see that mod's `pub use crate::tui::table::{TableAlign, TableColumn, TableRow, TableState};`).
//! Selection is a row index that never goes stale: every handler re-derives the selected row from the rows
//! that are visible, so a table that shrank or collapsed under the selection selects its first visible row instead of panicking.

use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::event::{Key, KeyEvent, MouseEvent};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::rows::{navigate_to, paint_scroll_bar, row_style, Pointer, Rows};
use crate::tui::text::{elide, truncate_to, Elision};
use crate::tui::theme::{Glyph, Role, Surface, Theme};
use crate::tui::widget::WidgetSignal;

//#region 🗂️State
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableAlign {
    Left,
    Right,
}

/// 📐️ One table column; `width == 0` means "flex" and splits the remaining space evenly.
pub struct TableColumn {
    pub label: String,
    pub width: u16,
    pub align: TableAlign,
}

impl TableColumn {
    pub fn new(label: impl Into<String>, width: u16, align: TableAlign) -> Self {
        Self { label: label.into(), width, align }
    }
}

/// 🌳️ One row, flat in display order; `level` and `has_children` express the tree: a row is
/// hidden whenever a preceding, still-nesting ancestor has `expanded == false`.
pub struct TableRow {
    pub id: String,
    pub cells: Vec<String>,
    pub level: u16,
    pub has_children: bool,
    pub expanded: bool,
}

impl TableRow {
    pub fn parent(id: impl Into<String>, cells: Vec<String>) -> Self {
        Self { id: id.into(), cells, level: 0, has_children: true, expanded: true }
    }

    pub fn child(id: impl Into<String>, cells: Vec<String>, level: u16) -> Self {
        Self { id: id.into(), cells, level, has_children: false, expanded: true }
    }
}

/// 🧮 A semio-styled table: bold muted header with a hairline underline, hairline row
/// separators, no vertical rules, no striping, mirroring the React `Table` and the print `semio-table.sty`.
/// Tree rows are plain indented rows in the same table.
pub struct TableState {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub selected: usize,
    /// 🈳️ The text shown when the table has no visible row; the application supplies it in the user's language.
    pub empty: String,
    view: Rows,
}

impl TableState {
    pub fn new(columns: Vec<TableColumn>, rows: Vec<TableRow>) -> Self {
        Self { columns, rows, selected: 0, empty: String::new(), view: Rows::new() }
    }

    /// 🪜 Row indices in display order, skipping any row nested under a collapsed ancestor.
    pub fn visible_indices(&self) -> Vec<usize> {
        let mut out = Vec::new();
        let mut collapsed_from: Option<u16> = None;
        for (i, row) in self.rows.iter().enumerate() {
            if let Some(level) = collapsed_from {
                if row.level > level {
                    continue;
                }
                collapsed_from = None;
            }
            out.push(i);
            if row.has_children && !row.expanded {
                collapsed_from = Some(row.level);
            }
        }
        out
    }

    fn position(&self, visible: &[usize]) -> usize {
        visible.iter().position(|&i| i == self.selected).unwrap_or(0)
    }
}
//#endregion 🗂️State

const ROW_LINES: u16 = 2;
const HEADER_LINES: u16 = 2;

fn body_area(rect: Rect) -> Rect {
    let (_, rest) = rect.split_top(HEADER_LINES.min(rect.height));
    rest
}

fn rows_fit(rect: Rect) -> usize {
    usize::from(body_area(rect).height.div_ceil(ROW_LINES)).max(1)
}

fn toggle(row: &mut TableRow, expanded: bool) -> bool {
    row.has_children && std::mem::replace(&mut row.expanded, expanded) != expanded
}

pub(crate) fn table_on_key(t: &mut TableState, ev: &KeyEvent) -> Option<WidgetSignal> {
    let visible = t.visible_indices();
    if visible.is_empty() {
        return None;
    }
    let position = t.position(&visible);
    if let Some(next) = navigate_to(ev.key, position, visible.len(), t.view.page()) {
        t.view.select(next, visible.len());
        let moved = visible[next] != t.selected;
        t.selected = visible[next];
        return moved.then_some(WidgetSignal::SelectionChanged(t.selected));
    }
    let current = visible[position];
    t.selected = current;
    match ev.key {
        Key::Right => toggle(&mut t.rows[current], true).then_some(WidgetSignal::SelectionChanged(current)),
        Key::Left => toggle(&mut t.rows[current], false).then_some(WidgetSignal::SelectionChanged(current)),
        Key::Enter => {
            let row = &mut t.rows[current];
            if row.has_children {
                row.expanded = !row.expanded;
                Some(WidgetSignal::SelectionChanged(current))
            } else {
                Some(WidgetSignal::Activated(current))
            }
        }
        _ => None,
    }
}

pub(crate) fn table_on_mouse(t: &mut TableState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    let visible = t.visible_indices();
    let area = body_area(rect);
    t.view.select(t.position(&visible), visible.len());
    match t.view.pointer_with(area, event, visible.len(), ROW_LINES) {
        Pointer::Selected(position) => {
            t.selected = visible[position];
            Some(WidgetSignal::SelectionChanged(t.selected))
        }
        Pointer::Activated(position) => {
            t.selected = visible[position];
            let row = &mut t.rows[t.selected];
            if row.has_children {
                row.expanded = !row.expanded;
                Some(WidgetSignal::SelectionChanged(t.selected))
            } else {
                Some(WidgetSignal::Activated(t.selected))
            }
        }
        Pointer::Context { position, row } => Some(WidgetSignal::ContextMenu { pos: position, item: row.map(|row| visible[row]) }),
        Pointer::Scrolled | Pointer::Ignored => None,
    }
}

pub(crate) fn table_set_hover(t: &mut TableState, rect: Rect, pos: Option<Pos>) -> bool {
    let count = t.visible_indices().len();
    t.view.hover_with(body_area(rect), pos, count, ROW_LINES)
}

/// 📏 Resolves each column's width: fixed columns keep their `width`, `width == 0` columns
/// split whatever space remains evenly.
fn table_column_widths(columns: &[TableColumn], total_width: u16) -> Vec<u16> {
    let fixed_total: u16 = columns.iter().filter(|c| c.width > 0).map(|c| c.width).sum();
    let gaps = columns.len().saturating_sub(1) as u16;
    let flex_count = columns.iter().filter(|c| c.width == 0).count() as u16;
    let remaining = total_width.saturating_sub(fixed_total + gaps);
    let flex_width = remaining.checked_div(flex_count).unwrap_or(0);
    columns.iter().map(|c| if c.width > 0 { c.width } else { flex_width }).collect()
}

/// 🖍️ Colours and attributes one table cell is painted with.
#[derive(Clone, Copy)]
struct Ink {
    fg: [u8; 3],
    bg: [u8; 3],
    attrs: u8,
}

fn paint_table_cell(buf: &mut CellBuffer, at: Pos, width: u16, text: &str, ink: Ink, align: TableAlign, clip: Rect) {
    let (t, tw) = truncate_to(text, width);
    let cell_x = match align {
        TableAlign::Left => at.x,
        TableAlign::Right => at.x + width.saturating_sub(tw),
    };
    buf.put_str(Pos { x: cell_x, y: at.y }, t, ink.fg, ink.bg, ink.attrs, clip);
}

/// 🖌️ Header (muted, bold) + hairline underline, then hairline-separated body rows; tree rows
/// indent by level and carry an expand marker. No vertical rules, no striping.
pub(crate) fn paint_table(t: &TableState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    if rect.width == 0 || rect.height == 0 || t.columns.is_empty() {
        return;
    }
    let bg = buf.get(rect.x, rect.y).map_or(theme.surface(Surface::Window), |c| c.bg);
    buf.fill_rect(rect, Cell::blank(theme.role(Role::MutedForeground), bg));

    let visible = t.visible_indices();
    let body = body_area(rect);
    let fit = rows_fit(rect);
    let overflow = visible.len() > fit && rect.width > 1;
    let content = Rect::new(rect.x, rect.y, rect.width - u16::from(overflow), rect.height);
    let widths = table_column_widths(&t.columns, content.width);
    let mut xs = Vec::with_capacity(widths.len());
    let mut x = content.x;
    for &w in &widths {
        xs.push(x);
        x += w + 1;
    }

    for ((col, &cx), &w) in t.columns.iter().zip(&xs).zip(&widths) {
        paint_table_cell(buf, Pos { x: cx, y: content.y }, w, &col.label, Ink { fg: theme.role(Role::MutedForeground), bg, attrs: attr::BOLD }, col.align, content);
    }
    if rect.height == 1 {
        return;
    }
    buf.hline(Pos { x: content.x, y: rect.y + 1 }, content.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
    if rect.height <= HEADER_LINES {
        return;
    }
    if visible.is_empty() {
        if !t.empty.is_empty() {
            let text = elide(&t.empty, content.width, Elision::End, theme.glyphs.ellipsis());
            let width = crate::tui::text::display_width(&text);
            buf.put_str(Pos { x: content.x + content.width.saturating_sub(width) / 2, y: body.y }, &text, theme.role(Role::MutedForeground), bg, 0, Rect::new(content.x, body.y, content.width, 1));
        }
        return;
    }

    let window = t.view.window_for(t.position(&visible), fit, visible.len());
    let bottom = rect.y + rect.height;
    let mut y = body.y;
    for &row_idx in &visible[window.clone()] {
        if y >= bottom {
            break;
        }
        let row = &t.rows[row_idx];
        let position = visible.iter().position(|&i| i == row_idx).unwrap_or(0);
        let (row_fg, row_bg, attrs) = {
            let (fg, row_bg, attrs) = row_style(theme, bg, row_idx == t.selected, focused, t.view.hover() == Some(position));
            if row_idx == t.selected || t.view.hover() == Some(position) { (fg, row_bg, attrs) } else { (theme.role(Role::MutedForeground), row_bg, attrs) }
        };
        buf.fill_rect(Rect::new(content.x, y, content.width, 1), Cell::blank(row_fg, row_bg));
        for (ci, ((col, &cx), &w)) in t.columns.iter().zip(&xs).zip(&widths).enumerate() {
            let text = if ci == 0 {
                let indent = "  ".repeat(usize::from(row.level));
                let marker = if !row.has_children {
                    "  ".to_string()
                } else if row.expanded {
                    format!("{} ", theme.glyphs.glyph(Glyph::Expanded))
                } else {
                    format!("{} ", theme.glyphs.glyph(Glyph::Collapsed))
                };
                format!("{indent}{marker}{}", row.cells.first().map_or("", String::as_str))
            } else {
                row.cells.get(ci).cloned().unwrap_or_default()
            };
            let text = elide(&text, w, Elision::End, theme.glyphs.ellipsis());
            paint_table_cell(buf, Pos { x: cx, y }, w, &text, Ink { fg: row_fg, bg: row_bg, attrs }, col.align, content);
        }
        y += 1;
        if y >= bottom {
            break;
        }
        buf.hline(Pos { x: content.x, y }, content.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
        y += 1;
    }
    paint_scroll_bar(buf, theme, Rect::new(rect.x + rect.width - 1, body.y, 1, body.height), window.start, fit, visible.len());
}
