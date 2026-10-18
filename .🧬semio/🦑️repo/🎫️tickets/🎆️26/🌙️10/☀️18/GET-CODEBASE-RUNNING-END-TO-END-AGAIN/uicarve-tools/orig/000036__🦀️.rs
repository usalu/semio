//! 💬️ tui overlay Dialog: a modal prompt with a title, a wrapped body and a row of buttons.
//! Every string comes from the app, so the element assumes no language. Wired as a crate-root sibling
//! module of `crate::tui::widget`; the engine owns placement, focus trapping and dismissal
//! (see `crate::tui::engine::Tui::open_dialog`).

use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::text::{display_width, truncate_to};
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::WidgetSignal;

const DIALOG_MAX_WIDTH: u16 = 64;
const DIALOG_MIN_WIDTH: u16 = 20;
const BUTTON_GAP: u16 = 2;

/// 🗨️ A modal prompt: `selected` names the focused button, `dismiss_outside` lets a click beside it cancel.
pub struct DialogState {
    pub title: String,
    pub body: String,
    pub buttons: Vec<String>,
    pub selected: usize,
    pub dismiss_outside: bool,
}

impl DialogState {
    pub fn new(title: impl Into<String>, body: impl Into<String>, buttons: Vec<String>) -> Self {
        Self { title: title.into(), body: body.into(), buttons, selected: 0, dismiss_outside: false }
    }
}

/// 📜️ Greedy word wrap by display cells; over-long words break at the cell limit.
pub(crate) fn wrap_cells(text: &str, width: u16) -> Vec<String> {
    let width = width.max(1);
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        let mut used = 0u16;
        for word in paragraph.split_whitespace() {
            let mut word = word;
            loop {
                let needed = display_width(word) + if used == 0 { 0 } else { 1 };
                if used + needed <= width {
                    if used > 0 {
                        line.push(' ');
                    }
                    line.push_str(word);
                    used += needed;
                    break;
                }
                if used > 0 {
                    lines.push(std::mem::take(&mut line));
                    used = 0;
                    continue;
                }
                let (head, _) = truncate_to(word, width);
                if head.is_empty() {
                    break;
                }
                lines.push(head.to_string());
                word = &word[head.len()..];
                if word.is_empty() {
                    break;
                }
            }
        }
        lines.push(line);
    }
    lines
}

/// 🖼️ Paints a bordered surface with the title set into the top border; shared by every overlay element.
pub(crate) fn paint_frame(buf: &mut CellBuffer, rect: Rect, theme: &Theme, surface: Surface, title: &str) {
    if rect.width < 2 || rect.height < 2 {
        return;
    }
    let bg = theme.surface(surface);
    let border = theme.role(Role::BorderEmphasized);
    let fg = theme.role(Role::Foreground);
    buf.fill_rect(rect, Cell::blank(fg, bg));
    let right = rect.x + rect.width - 1;
    let bottom = rect.y + rect.height - 1;
    buf.hline(Pos { x: rect.x + 1, y: rect.y }, rect.width - 2, '\u{2500}', border, bg);
    buf.hline(Pos { x: rect.x + 1, y: bottom }, rect.width - 2, '\u{2500}', border, bg);
    buf.vline(Pos { x: rect.x, y: rect.y + 1 }, rect.height - 2, '\u{2502}', border, bg);
    buf.vline(Pos { x: right, y: rect.y + 1 }, rect.height - 2, '\u{2502}', border, bg);
    for (x, y, ch) in [(rect.x, rect.y, '\u{250c}'), (right, rect.y, '\u{2510}'), (rect.x, bottom, '\u{2514}'), (right, bottom, '\u{2518}')] {
        buf.put(x, y, Cell { ch, fg: border, bg, attrs: 0, width: 1 });
    }
    if !title.is_empty() && rect.width > 6 {
        let (head, _) = truncate_to(title, rect.width - 6);
        let label = format!(" {head} ");
        buf.put_str(Pos { x: rect.x + 2, y: rect.y }, &label, fg, bg, attr::BOLD, Rect::new(rect.x + 1, rect.y, rect.width - 2, 1));
    }
}

fn inner_width(width: u16) -> u16 {
    width.saturating_sub(4).max(1)
}

fn buttons_width(d: &DialogState) -> u16 {
    let labels: u16 = d.buttons.iter().map(|b| display_width(b) + 2).sum();
    labels + BUTTON_GAP * d.buttons.len().saturating_sub(1) as u16
}

/// 📐️ The overlay's size for `viewport`: wide enough for the title, body and buttons, capped to the terminal.
pub(crate) fn dialog_size(d: &DialogState, viewport: Size) -> Size {
    let longest_body = d.body.split('\n').map(display_width).max().unwrap_or(0);
    let wanted = (display_width(&d.title) + 2).max(longest_body).max(buttons_width(d)) + 4;
    let width = wanted.clamp(DIALOG_MIN_WIDTH, DIALOG_MAX_WIDTH).min(viewport.width.saturating_sub(2).max(2));
    let body_rows = wrap_cells(&d.body, inner_width(width)).len() as u16;
    let height = (body_rows + 4).min(viewport.height.saturating_sub(2).max(2));
    Size { width, height }
}

/// 🔘️ Button rectangles in paint order, centred on the bottom content row.
pub(crate) fn dialog_button_rects(d: &DialogState, rect: Rect) -> Vec<Rect> {
    if rect.height < 3 {
        return Vec::new();
    }
    let total = buttons_width(d).min(rect.width.saturating_sub(2));
    let mut x = rect.x + 1 + rect.width.saturating_sub(2).saturating_sub(total) / 2;
    let y = rect.y + rect.height - 2;
    let mut out = Vec::new();
    for button in &d.buttons {
        let width = display_width(button) + 2;
        out.push(Rect::new(x, y, width, 1));
        x += width + BUTTON_GAP;
    }
    out
}

pub(crate) fn dialog_on_key(d: &mut DialogState, ev: &KeyEvent) -> Option<WidgetSignal> {
    if d.buttons.is_empty() {
        return None;
    }
    let count = d.buttons.len();
    match ev.key {
        Key::Left | Key::Up | Key::BackTab => {
            d.selected = (d.selected + count - 1) % count;
            Some(WidgetSignal::SelectionChanged(d.selected))
        }
        Key::Right | Key::Down | Key::Tab => {
            d.selected = (d.selected + 1) % count;
            Some(WidgetSignal::SelectionChanged(d.selected))
        }
        Key::Home => {
            d.selected = 0;
            Some(WidgetSignal::SelectionChanged(0))
        }
        Key::End => {
            d.selected = count - 1;
            Some(WidgetSignal::SelectionChanged(count - 1))
        }
        Key::Enter | Key::Char(' ') => Some(WidgetSignal::Activated(d.selected)),
        _ => None,
    }
}

pub(crate) fn dialog_on_mouse(d: &mut DialogState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    let MouseKind::Down(MouseButton::Left) = event.kind else { return None };
    let index = dialog_button_rects(d, rect).iter().position(|button| button.contains(event.pos))?;
    d.selected = index;
    Some(WidgetSignal::Activated(index))
}

pub(crate) fn dialog_set_hover(d: &mut DialogState, rect: Rect, pos: Option<Pos>) -> bool {
    let Some(pos) = pos else { return false };
    match dialog_button_rects(d, rect).iter().position(|button| button.contains(pos)) {
        Some(index) if index != d.selected => {
            d.selected = index;
            true
        }
        _ => false,
    }
}

pub(crate) fn paint_dialog(d: &DialogState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    paint_frame(buf, rect, theme, Surface::Dialog, &d.title);
    if rect.width < 6 || rect.height < 4 {
        return;
    }
    let bg = theme.surface(Surface::Dialog);
    let fg = theme.role(Role::Foreground);
    let body_rows = usize::from(rect.height - 4);
    for (row, line) in wrap_cells(&d.body, inner_width(rect.width)).iter().take(body_rows).enumerate() {
        let clip = Rect::new(rect.x + 2, rect.y + 1 + row as u16, rect.width - 4, 1);
        buf.put_str(Pos { x: clip.x, y: clip.y }, line, fg, bg, 0, clip);
    }
    for (index, (button, slot)) in d.buttons.iter().zip(dialog_button_rects(d, rect)).enumerate() {
        let selected = index == d.selected;
        let (button_fg, button_bg) = if selected { (theme.role(Role::ActiveForeground), theme.role(Role::ActiveBase)) } else { (fg, theme.surface(Surface::Panel)) };
        buf.fill_rect(slot, Cell::blank(button_fg, button_bg));
        buf.put_str(Pos { x: slot.x + 1, y: slot.y }, button, button_fg, button_bg, if selected { attr::BOLD } else { 0 }, slot);
    }
}
