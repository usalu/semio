//! 📃️ tui state, key, pointer and paint functions for the List element: a scrollable, selectable, optionally
//! multi-marked list whose viewport follows the selection, with hover, select-then-activate and a scroll bar.
//! Wired as a crate-root sibling module of `crate::tui::widget` (see that mod's `pub use crate::tui::list::ListState;`).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::rows::{follow_top, navigate_to, paint_scroll_bar, row_style, row_under};
use crate::tui::text::{elide, Elision};
use crate::tui::theme::{Glyph, Role, Status, Surface, Theme};
use crate::tui::widget::WidgetSignal;
use std::cell::Cell as Shared;

const WHEEL_ROWS: usize = 3;
const SPIN_FRAME_MS: u64 = 125;

/// 📋 Items with a selection, a persistent scroll offset, per-item marks and a hover row.
pub struct ListState {
    pub items: Vec<String>,
    pub selected: usize,
    pub offset: usize,
    pub marks: Vec<bool>,
    /// 🚦️ The state of each row, parallel to `items` (rows past its end have none); a row with a status shows its
    /// glyph in the status role colour after the mark, and every row then reserves that column.
    pub statuses: Vec<Option<Status>>,
    /// 🈳️ The text shown when the list has no items; the application supplies it in the user's language.
    pub empty: String,
    hover: Option<usize>,
    frame: u64,
    page: Shared<usize>,
    shown: Shared<usize>,
    anchored: Shared<bool>,
}

impl ListState {
    pub fn new(items: Vec<String>) -> Self {
        let marks = vec![false; items.len()];
        Self { items, selected: 0, offset: 0, marks, statuses: Vec::new(), empty: String::new(), hover: None, frame: 0, page: Shared::new(0), shown: Shared::new(0), anchored: Shared::new(true) }
    }

    /// 🎯️ Selects `position` (clamped) and scrolls it into view; true when the selection moved.
    pub fn select(&mut self, position: usize) -> bool {
        let next = position.min(self.items.len().saturating_sub(1));
        self.anchored.set(true);
        self.offset = follow_top(next, self.offset, self.page.get(), self.items.len());
        std::mem::replace(&mut self.selected, next) != next
    }

    pub fn hover(&self) -> Option<usize> {
        self.hover
    }

    /// 🏁 Sets the status of row `position`, growing the status column as needed.
    pub fn set_status(&mut self, position: usize, status: Option<Status>) {
        if self.statuses.len() <= position {
            self.statuses.resize(position + 1, None);
        }
        self.statuses[position] = status;
    }

    fn status_at(&self, position: usize) -> Option<Status> {
        self.statuses.get(position).copied().flatten()
    }

    fn has_status(&self) -> bool {
        self.statuses.iter().any(Option::is_some)
    }

    fn top_for(&self, rows: usize) -> usize {
        let max_top = self.items.len().saturating_sub(rows.max(1));
        if self.anchored.get() {
            follow_top(self.selected.min(self.items.len().saturating_sub(1)), self.offset, rows, self.items.len())
        } else {
            self.offset.min(max_top)
        }
    }
}

fn body(l: &ListState, rect: Rect) -> Rect {
    let overflow = l.items.len() > usize::from(rect.height) && rect.width > 1;
    Rect::new(rect.x, rect.y, rect.width - u16::from(overflow), rect.height)
}

pub(crate) fn list_on_key(l: &mut ListState, ev: &KeyEvent) -> Option<WidgetSignal> {
    let count = l.items.len();
    if count == 0 {
        return None;
    }
    if let Some(next) = navigate_to(ev.key, l.selected, count, l.page.get()) {
        return l.select(next).then_some(WidgetSignal::SelectionChanged(l.selected));
    }
    match ev.key {
        Key::Char(' ') => {
            let selected = l.selected.min(count - 1);
            let mark = l.marks.get_mut(selected)?;
            *mark = !*mark;
            Some(WidgetSignal::Toggled(*mark))
        }
        Key::Enter => Some(WidgetSignal::Activated(l.selected.min(count - 1))),
        _ => None,
    }
}

pub(crate) fn list_on_mouse(l: &mut ListState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    let area = body(l, rect);
    let count = l.items.len();
    let row = row_under(area, event.pos, l.shown.get(), count);
    match event.kind {
        MouseKind::Down(MouseButton::Left) => {
            let position = row?;
            l.select(position);
            Some(if event.clicks >= 2 { WidgetSignal::Activated(position) } else { WidgetSignal::SelectionChanged(position) })
        }
        MouseKind::Down(MouseButton::Right) => {
            if let Some(position) = row {
                l.select(position);
            }
            Some(WidgetSignal::ContextMenu { pos: event.pos, item: row })
        }
        MouseKind::Scroll { dy, .. } if dy != 0 && rect.contains(event.pos) => {
            let max_top = count.saturating_sub(usize::from(rect.height).max(1));
            let base = l.shown.get();
            l.offset = base.saturating_add_signed(isize::from(dy) * WHEEL_ROWS as isize).min(max_top);
            l.anchored.set(false);
            None
        }
        _ => None,
    }
}

/// ⏱️ Advances the spinner of rows that are running; true when a frame changed and the list must repaint.
pub(crate) fn list_tick(l: &mut ListState, now_ms: u64) -> bool {
    if !l.statuses.contains(&Some(Status::Running)) {
        return false;
    }
    let frame = now_ms / SPIN_FRAME_MS;
    std::mem::replace(&mut l.frame, frame) != frame
}

pub(crate) fn list_set_hover(l: &mut ListState, rect: Rect, pos: Option<Pos>) -> bool {
    let row = pos.and_then(|pos| row_under(body(l, rect), pos, l.shown.get(), l.items.len()));
    std::mem::replace(&mut l.hover, row) != row
}

pub(crate) fn paint_list(l: &ListState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let bg = theme.surface(Surface::Window);
    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
    let count = l.items.len();
    let rows = usize::from(rect.height);
    l.page.set(rows);
    let top = l.top_for(rows);
    l.shown.set(top);
    let area = body(l, rect);
    let ellipsis = theme.glyphs.ellipsis();
    for (index, position) in (top..(top + rows).min(count)).enumerate() {
        let y = rect.y + index as u16;
        let (fg, row_bg, attrs) = row_style(theme, bg, position == l.selected, focused, l.hover == Some(position));
        let line = Rect::new(area.x, y, area.width, 1);
        buf.fill_rect(line, Cell::blank(fg, row_bg));
        let mark = if l.marks.get(position).copied().unwrap_or(false) { theme.glyphs.glyph(Glyph::Check) } else { " " };
        let label_x = area.x + 2 + 2 * u16::from(l.has_status());
        let text = elide(&l.items[position], area.width.saturating_sub(label_x - area.x), Elision::End, ellipsis);
        buf.put_str(Pos { x: area.x, y }, mark, fg, row_bg, attrs, line);
        if let Some(status) = l.status_at(position) {
            let status_fg = if position == l.selected && focused { fg } else { theme.role(status.role()) };
            buf.put_str(Pos { x: area.x + 2, y }, status.glyph(theme.glyphs, l.frame), status_fg, row_bg, attrs, line);
        }
        buf.put_str(Pos { x: label_x, y }, &text, fg, row_bg, attrs, line);
    }
    if count == 0 && !l.empty.is_empty() {
        let text = elide(&l.empty, rect.width, Elision::End, ellipsis);
        buf.put_str(Pos { x: rect.x, y: rect.y }, &text, theme.role(Role::MutedForeground), bg, 0, Rect::new(rect.x, rect.y, rect.width, 1));
    }
    paint_scroll_bar(buf, theme, Rect::new(rect.x + rect.width - 1, rect.y, 1, rect.height), top, rows, count);
}
