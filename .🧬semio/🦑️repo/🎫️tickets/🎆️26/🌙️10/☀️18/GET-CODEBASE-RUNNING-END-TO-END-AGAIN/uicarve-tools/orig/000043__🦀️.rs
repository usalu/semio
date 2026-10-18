//! 🔀️ tui state, key, pointer and paint functions for the Toggle element: an on/off switch with a label that
//! flips with Space, Enter or a click. Wired as a crate-root sibling module of `crate::tui::widget`
//! (see that mod's `pub use crate::tui::toggle::ToggleState;`).

use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::{display_width, elide, Elision};
use crate::tui::theme::{Glyph, Role, Surface, Theme};
use crate::tui::widget::WidgetSignal;

/// 🎛️ A labelled switch.
pub struct ToggleState {
    pub label: String,
    pub on: bool,
    hover: bool,
}

impl ToggleState {
    pub fn new(label: impl Into<String>, on: bool) -> Self {
        Self { label: label.into(), on, hover: false }
    }

    /// 📐️ The width of the switch glyph, a gap and the label.
    pub fn preferred_width(&self) -> u16 {
        display_width(&self.label).saturating_add(2)
    }

    fn flip(&mut self) -> WidgetSignal {
        self.on = !self.on;
        WidgetSignal::Toggled(self.on)
    }
}

pub(crate) fn toggle_on_key(t: &mut ToggleState, ev: &KeyEvent) -> Option<WidgetSignal> {
    match ev.key {
        Key::Char(' ') | Key::Enter => Some(t.flip()),
        Key::Left | Key::Home if t.on => Some(t.flip()),
        Key::Right | Key::End if !t.on => Some(t.flip()),
        _ => None,
    }
}

pub(crate) fn toggle_on_mouse(t: &mut ToggleState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    (matches!(event.kind, MouseKind::Down(MouseButton::Left)) && rect.contains(event.pos)).then(|| t.flip())
}

pub(crate) fn toggle_set_hover(t: &mut ToggleState, rect: Rect, pos: Option<Pos>) -> bool {
    let hover = pos.is_some_and(|pos| rect.contains(pos));
    std::mem::replace(&mut t.hover, hover) != hover
}

pub(crate) fn paint_toggle(t: &ToggleState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let bg = if t.hover { theme.role(Role::HoverInteractive) } else { theme.surface(Surface::Window) };
    let line = Rect::new(rect.x, rect.y, rect.width, 1);
    let fg = theme.role(if focused { Role::Accent } else { Role::Foreground });
    buf.fill_rect(line, Cell::blank(fg, bg));
    let (glyph, glyph_fg) = if t.on { (theme.glyphs.glyph(Glyph::ToggleOn), theme.role(Role::Accent)) } else { (theme.glyphs.glyph(Glyph::ToggleOff), theme.role(Role::MutedForeground)) };
    buf.put_str(Pos { x: rect.x, y: rect.y }, glyph, glyph_fg, bg, 0, line);
    let label = elide(&t.label, rect.width.saturating_sub(2), Elision::End, theme.glyphs.ellipsis());
    buf.put_str(Pos { x: rect.x + 2, y: rect.y }, &label, fg, bg, if focused { attr::BOLD } else { 0 }, line);
}
