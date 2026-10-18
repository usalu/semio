//! 🖱️ tui overlay Menu: a context or dropdown menu anchored at a cell. Every label comes from the app.
//! Wired as a crate-root sibling module of `crate::tui::widget`; the engine owns placement and dismissal
//! (see `crate::tui::engine::Tui::open_menu`).

use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::dialog::paint_frame;
use crate::tui::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::text::{display_width, truncate_to};
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::WidgetSignal;

const SHORTCUT_GAP: u16 = 2;

/// 📋️ One menu row; a separator row carries no label and cannot be selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuItem {
    pub label: String,
    pub shortcut: String,
    pub enabled: bool,
    pub separator: bool,
}

impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), shortcut: String::new(), enabled: true, separator: false }
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = shortcut.into();
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn separator() -> Self {
        Self { label: String::new(), shortcut: String::new(), enabled: false, separator: true }
    }

    fn selectable(&self) -> bool {
        self.enabled && !self.separator
    }
}

/// 📑 A menu: `selected` indexes `items`; `offset` is the first visible row, kept by paint when the menu is taller than its viewport.
pub struct MenuState {
    pub items: Vec<MenuItem>,
    pub selected: Option<usize>,
    offset: std::cell::Cell<usize>,
}

impl MenuState {
    pub fn new(items: Vec<MenuItem>) -> Self {
        let selected = items.iter().position(MenuItem::selectable);
        Self { items, selected, offset: std::cell::Cell::new(0) }
    }

    fn step(&self, from: Option<usize>, forward: bool) -> Option<usize> {
        let count = self.items.len();
        if count == 0 {
            return None;
        }
        let start = from.unwrap_or(if forward { count - 1 } else { 0 });
        (1..=count).map(|n| if forward { (start + n) % count } else { (start + count - n % count) % count }).find(|&i| self.items[i].selectable())
    }
}

/// 📐️ The overlay's size for `viewport`: widest label plus shortcut inside a one-cell border and padding.
pub(crate) fn menu_size(m: &MenuState, viewport: Size) -> Size {
    let widest = m.items.iter().map(|i| display_width(&i.label) + if i.shortcut.is_empty() { 0 } else { SHORTCUT_GAP + display_width(&i.shortcut) }).max().unwrap_or(0);
    let width = (widest + 4).max(8).min(viewport.width.max(2));
    let height = (m.items.len() as u16 + 2).min(viewport.height.max(2));
    Size { width, height }
}

fn visible_rows(rect: Rect) -> usize {
    usize::from(rect.height.saturating_sub(2))
}

fn view_offset(m: &MenuState, rows: usize) -> usize {
    let rows = rows.max(1);
    let mut offset = m.offset.get().min(m.items.len().saturating_sub(rows));
    if let Some(selected) = m.selected {
        if selected < offset {
            offset = selected;
        } else if selected >= offset + rows {
            offset = selected + 1 - rows;
        }
    }
    m.offset.set(offset);
    offset
}

fn row_at(m: &MenuState, rect: Rect, pos: Pos) -> Option<usize> {
    if pos.x <= rect.x || pos.x + 1 >= rect.x + rect.width || pos.y <= rect.y || pos.y + 1 >= rect.y + rect.height {
        return None;
    }
    let index = view_offset(m, visible_rows(rect)) + usize::from(pos.y - rect.y - 1);
    (index < m.items.len()).then_some(index)
}

pub(crate) fn menu_on_key(m: &mut MenuState, ev: &KeyEvent) -> Option<WidgetSignal> {
    let moved = match ev.key {
        Key::Down | Key::Tab => m.step(m.selected, true),
        Key::Up | Key::BackTab => m.step(m.selected, false),
        Key::Home => m.items.iter().position(MenuItem::selectable),
        Key::End => m.items.iter().rposition(MenuItem::selectable),
        Key::Enter | Key::Char(' ') => return m.selected.filter(|&i| m.items[i].selectable()).map(WidgetSignal::Activated),
        _ => return None,
    };
    let changed = moved != m.selected;
    m.selected = moved;
    changed.then_some(WidgetSignal::SelectionChanged(moved?))
}

pub(crate) fn menu_on_mouse(m: &mut MenuState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    match event.kind {
        MouseKind::Down(MouseButton::Left) => {
            let index = row_at(m, rect, event.pos)?;
            m.items[index].selectable().then(|| {
                m.selected = Some(index);
                WidgetSignal::Activated(index)
            })
        }
        MouseKind::Scroll { dy, .. } if dy != 0 => {
            let moved = m.step(m.selected, dy > 0);
            let changed = moved != m.selected;
            m.selected = moved;
            changed.then_some(WidgetSignal::SelectionChanged(moved?))
        }
        _ => None,
    }
}

pub(crate) fn menu_set_hover(m: &mut MenuState, rect: Rect, pos: Option<Pos>) -> bool {
    let Some(index) = pos.and_then(|p| row_at(m, rect, p)) else { return false };
    if m.items[index].selectable() && m.selected != Some(index) {
        m.selected = Some(index);
        return true;
    }
    false
}

pub(crate) fn paint_menu(m: &MenuState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    paint_frame(buf, rect, theme, Surface::Menu, "");
    if rect.width < 6 || rect.height < 3 {
        return;
    }
    let bg = theme.surface(Surface::Menu);
    let border = theme.role(Role::BorderEmphasized);
    let offset = view_offset(m, visible_rows(rect));
    for row in 0..visible_rows(rect) {
        let index = offset + row;
        let Some(item) = m.items.get(index) else { break };
        let y = rect.y + 1 + row as u16;
        let line = Rect::new(rect.x + 1, y, rect.width - 2, 1);
        if item.separator {
            buf.hline(Pos { x: line.x, y }, line.width, '\u{2500}', border, bg);
            continue;
        }
        let selected = m.selected == Some(index);
        let (fg, row_bg) = match (selected, item.enabled) {
            (true, _) => (theme.role(Role::ActiveForeground), theme.role(Role::ActiveBase)),
            (false, true) => (theme.role(Role::Foreground), bg),
            (false, false) => (theme.role(Role::MutedForeground), bg),
        };
        buf.fill_rect(line, Cell::blank(fg, row_bg));
        let attrs = if item.enabled { 0 } else { attr::DIM };
        let room = line.width.saturating_sub(2);
        let shortcut_width = display_width(&item.shortcut);
        let label_room = if shortcut_width == 0 { room } else { room.saturating_sub(shortcut_width + SHORTCUT_GAP) };
        let (label, _) = truncate_to(&item.label, label_room);
        buf.put_str(Pos { x: line.x + 1, y }, label, fg, row_bg, attrs, line);
        if shortcut_width > 0 && shortcut_width < room {
            buf.put_str(Pos { x: line.x + line.width - 1 - shortcut_width, y }, &item.shortcut, fg, row_bg, attrs | attr::DIM, line);
        }
    }
}
