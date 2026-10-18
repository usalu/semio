//! 🔎️ tui paint function for the Footer element — extracted from `chrome` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::tui::chrome` (see that mod's `use crate::tui::footer::paint_footer;`). Hints and status never overlap:
//! the status keeps up to a third of the row before hints are cut, and whatever is cut ends in an ellipsis.

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::chrome::FooterState;
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::{display_width, elide, Elision};
use crate::tui::theme::{Role, Surface, Theme};

const STATUS_GAP: u16 = 1;

pub(crate) fn paint_footer(f: &FooterState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = theme.surface(Surface::Base);
    let (hairline, content) = rect.split_top(1);
    buf.hline(Pos { x: hairline.x, y: hairline.y }, hairline.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
    buf.fill_rect(content, Cell::blank(theme.role(Role::Foreground), bg));
    let status_width = display_width(&f.status);
    let reserve = status_width.min(content.width / 3);
    let hint_end = content.x + content.width.saturating_sub(reserve + STATUS_GAP);
    let mut x = content.x;
    let mut cut = false;
    for hint in &f.hints {
        let key = format!(" {} ", hint.key);
        let label = format!("{} ", hint.label);
        if x + display_width(&key) + display_width(&label) > hint_end {
            cut = true;
            break;
        }
        let clip = Rect::new(content.x, content.y, content.width, 1);
        x += buf.put_str(Pos { x, y: content.y }, &key, theme.role(Role::Accent), bg, 0, clip);
        x += buf.put_str(Pos { x, y: content.y }, &label, theme.role(Role::MutedForeground), bg, 0, clip);
    }
    if cut && x < hint_end {
        buf.put_str(Pos { x, y: content.y }, "\u{2026}", theme.role(Role::MutedForeground), bg, 0, Rect::new(x, content.y, hint_end - x, 1));
        x += 1;
    }
    let status_room = (content.x + content.width).saturating_sub(x + STATUS_GAP);
    let status = elide(&f.status, status_room, Elision::End, theme.glyphs.ellipsis());
    let status_x = content.x + content.width - display_width(&status);
    buf.put_str(Pos { x: status_x, y: content.y }, &status, theme.role(Role::MutedForeground), bg, 0, Rect::new(status_x, content.y, display_width(&status), 1));
}
