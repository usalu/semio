//! 🔎️ tui paint function for the Navbar element — extracted from `chrome` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::tui::chrome` (see that mod's `use crate::tui::navbar::paint_navbar;`). Placement follows the React
//! `navbarFreeBandV1`: the centre sits in the band the flow items leave free and never covers them.

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::chrome::{NavItem, NavbarState};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::{display_width, elide, Elision};
use crate::tui::theme::{Role, Surface, Theme};

const BAND_GAP: u16 = 1;

fn items_width(items: &[NavItem]) -> u16 {
    items.iter().map(|item| display_width(&item.label) + 2).sum()
}

fn paint_items(items: &[NavItem], theme: &Theme, x: u16, y: u16, bg: [u8; 3], clip: Rect, buf: &mut CellBuffer) {
    let end = clip.x + clip.width;
    let mut x = x;
    for item in items {
        if x >= end {
            break;
        }
        let fg = if item.active { theme.role(Role::Accent) } else { theme.role(Role::Foreground) };
        let label = format!(" {} ", item.label);
        let label = elide(&label, end - x, Elision::End, theme.glyphs.ellipsis());
        x += buf.put_str(Pos { x, y }, &label, fg, bg, 0, clip);
    }
}

pub(crate) fn paint_navbar(n: &NavbarState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = theme.surface(Surface::Base);
    let (content, hairline) = rect.split_bottom(1);
    buf.fill_rect(content, Cell::blank(theme.role(Role::Foreground), bg));
    let left_width = items_width(&n.left).min(content.width);
    paint_items(&n.left, theme, content.x, content.y, bg, Rect::new(content.x, content.y, left_width, 1), buf);
    let right_room = content.width.saturating_sub(left_width + BAND_GAP);
    let right_width = items_width(&n.right).min(right_room);
    let right_x = content.x + content.width - right_width;
    paint_items(&n.right, theme, right_x, content.y, bg, Rect::new(right_x, content.y, right_width, 1), buf);
    let band_start = content.x + if left_width > 0 { left_width + BAND_GAP } else { 0 };
    let band_end = right_x.saturating_sub(if right_width > 0 { BAND_GAP } else { 0 }).max(band_start);
    let band = band_end - band_start;
    let center_text: String = n.center.iter().map(|i| i.label.clone()).collect::<Vec<_>>().join(" ");
    if band > 0 && !center_text.is_empty() {
        let text = elide(&center_text, band, Elision::End, theme.glyphs.ellipsis());
        let width = display_width(&text);
        let ideal = content.x + content.width.saturating_sub(width) / 2;
        let x = ideal.clamp(band_start, band_end - width);
        buf.put_str(Pos { x, y: content.y }, &text, theme.role(Role::MutedForeground), bg, 0, Rect::new(band_start, content.y, band, 1));
    }
    buf.hline(Pos { x: hairline.x, y: hairline.y }, hairline.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
}
