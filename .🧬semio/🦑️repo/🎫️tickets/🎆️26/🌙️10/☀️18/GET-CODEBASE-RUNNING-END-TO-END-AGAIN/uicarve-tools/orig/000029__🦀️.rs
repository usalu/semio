//! ⌨️ tui state, key, paste, pointer, cursor and paint functions for the single-line Input element, wired as a
//! crate-root sibling module of `crate::tui::widget` (see that mod's `pub use crate::tui::input::InputState;`).
//! The cursor is a byte offset that always sits on a grapheme cluster boundary, so arrows, Backspace and Delete
//! move by whole clusters and never panic on non-ASCII text. The caret is the terminal's own cursor (`input_cursor`);
//! the painter draws no caret glyph, so it can never hide a character.

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::event::{mods, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::{boundary_at_cell, boundary_at_or_before, display_width, elide, next_boundary, previous_boundary, window_cells, Elision};
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{CursorShape, CursorSpec, WidgetSignal};

/// ✏️ A single-line text value, its cursor (a byte offset) and the hint shown while it is empty.
pub struct InputState {
    pub value: String,
    pub cursor: usize,
    pub placeholder: String,
}

impl InputState {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self { value: String::new(), cursor: 0, placeholder: placeholder.into() }
    }

    /// 📝️ Replaces the value and puts the cursor behind it.
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.cursor = self.value.len();
    }

    fn caret(&self) -> usize {
        boundary_at_or_before(&self.value, self.cursor.min(self.value.len()))
    }

    fn changed(&self) -> WidgetSignal {
        WidgetSignal::ValueChanged(self.value.clone())
    }
}

fn word_start(value: &str, from: usize) -> usize {
    let mut at = from;
    while at > 0 {
        let before = previous_boundary(value, at);
        if !value[before..at].chars().all(char::is_whitespace) {
            break;
        }
        at = before;
    }
    while at > 0 {
        let before = previous_boundary(value, at);
        if value[before..at].chars().all(char::is_whitespace) {
            break;
        }
        at = before;
    }
    at
}

fn word_end(value: &str, from: usize) -> usize {
    let mut at = from;
    while at < value.len() {
        let after = next_boundary(value, at);
        if !value[at..after].chars().all(char::is_whitespace) {
            break;
        }
        at = after;
    }
    while at < value.len() {
        let after = next_boundary(value, at);
        if value[at..after].chars().all(char::is_whitespace) {
            break;
        }
        at = after;
    }
    at
}

pub(crate) fn input_on_key(i: &mut InputState, ev: &KeyEvent) -> Option<WidgetSignal> {
    let caret = i.caret();
    i.cursor = caret;
    let ctrl = ev.mods & mods::CTRL != 0;
    let alt = ev.mods & mods::ALT != 0;
    match ev.key {
        Key::Char(c) if !ctrl && !alt && !c.is_control() => {
            i.value.insert(caret, c);
            i.cursor = caret + c.len_utf8();
            Some(i.changed())
        }
        Key::Char('a') if ctrl => {
            i.cursor = 0;
            None
        }
        Key::Char('e') if ctrl => {
            i.cursor = i.value.len();
            None
        }
        Key::Char('u') if ctrl => {
            i.value.drain(..caret);
            i.cursor = 0;
            (caret > 0).then(|| i.changed())
        }
        Key::Char('k') if ctrl => {
            let removed = i.value.len() > caret;
            i.value.truncate(caret);
            removed.then(|| i.changed())
        }
        Key::Char('w') if ctrl => delete_back(i, word_start(&i.value, caret)),
        Key::Backspace if ctrl || alt => delete_back(i, word_start(&i.value, caret)),
        Key::Backspace => delete_back(i, previous_boundary(&i.value, caret)),
        Key::Delete if ctrl || alt => delete_forward(i, word_end(&i.value, caret)),
        Key::Delete => delete_forward(i, next_boundary(&i.value, caret)),
        Key::Left => {
            i.cursor = if ctrl || alt { word_start(&i.value, caret) } else { previous_boundary(&i.value, caret) };
            None
        }
        Key::Right => {
            i.cursor = if ctrl || alt { word_end(&i.value, caret) } else { next_boundary(&i.value, caret) };
            None
        }
        Key::Home => {
            i.cursor = 0;
            None
        }
        Key::End => {
            i.cursor = i.value.len();
            None
        }
        _ => None,
    }
}

fn delete_back(i: &mut InputState, to: usize) -> Option<WidgetSignal> {
    let caret = i.caret();
    if to >= caret {
        return None;
    }
    i.value.drain(to..caret);
    i.cursor = to;
    Some(i.changed())
}

fn delete_forward(i: &mut InputState, to: usize) -> Option<WidgetSignal> {
    let caret = i.caret();
    if to <= caret {
        return None;
    }
    i.value.drain(caret..to);
    i.cursor = caret;
    Some(i.changed())
}

pub(crate) fn input_on_paste(i: &mut InputState, text: &str) -> Option<WidgetSignal> {
    let clean: String = text.chars().map(|c| if c.is_whitespace() { ' ' } else { c }).filter(|c| !c.is_control()).collect();
    if clean.is_empty() {
        return None;
    }
    let caret = i.caret();
    i.value.insert_str(caret, &clean);
    i.cursor = caret + clean.len();
    Some(i.changed())
}

fn view_start(i: &InputState, width: u16) -> u16 {
    let column = display_width(&i.value[..i.caret()]);
    (column + 1).saturating_sub(width)
}

pub(crate) fn input_on_mouse(i: &mut InputState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    if !matches!(event.kind, MouseKind::Down(MouseButton::Left) | MouseKind::Drag(MouseButton::Left)) || !rect.contains(event.pos) {
        return None;
    }
    let column = view_start(i, rect.width) + (event.pos.x - rect.x);
    i.cursor = boundary_at_cell(&i.value, column);
    None
}

pub(crate) fn input_cursor(i: &InputState, rect: Rect) -> Option<CursorSpec> {
    if rect.width == 0 || rect.height == 0 {
        return None;
    }
    let column = display_width(&i.value[..i.caret()]) - view_start(i, rect.width);
    Some(CursorSpec { pos: Pos { x: (rect.x + column).min(rect.x + rect.width - 1), y: rect.y }, shape: CursorShape::Bar, blink: true })
}

pub(crate) fn paint_input(i: &InputState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, _focused: bool) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let bg = theme.surface(Surface::Panel);
    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
    let line = Rect::new(rect.x, rect.y, rect.width, 1);
    if i.value.is_empty() {
        let text = elide(&i.placeholder, rect.width, Elision::End, theme.glyphs.ellipsis());
        buf.put_str(Pos { x: rect.x, y: rect.y }, &text, theme.role(Role::MutedForeground), bg, 0, line);
    } else {
        let start = view_start(i, rect.width);
        let (visible, lead) = window_cells(&i.value, start, rect.width);
        buf.put_str(Pos { x: rect.x + lead, y: rect.y }, visible, theme.role(Role::Foreground), bg, 0, line);
    }
}
