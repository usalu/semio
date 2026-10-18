//! 🔎️ tui paint function for the Window element — extracted from `chrome` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::tui::chrome` (see that mod's `use crate::tui::window::paint_window;`). `paint_corner_tab` is a
//! private helper used only by `paint_window` and stays module-private here. `window_chip_layout`,
//! `WindowTab` and `WindowChipLayout`'s fields stay in `chrome` mod (shared with
//! `ChromeState::window_target`) and were promoted to `pub(crate)` there so this sibling module
//! can reach them.

use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::chrome::{window_chip_layout, ChromeMode, TabKind, WindowChipLayout, WindowCornerTab, WindowHit, WindowState, WindowTab};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::layout::WindowStackCorner;
use crate::tui::theme::{Glyph, GlyphSet, Rgb, Role, Surface, Theme};

/// 🖋️ The line-drawing set a window outline uses: heavy for the focused stack so focus shows without colour.
struct Lines {
    h: char,
    v: char,
    tl: char,
    tr: char,
    bl: char,
    br: char,
    up: char,
    down: char,
    right: char,
    left: char,
}

/// 🖼️ What the outline painters share: the line set, the border and surface colours and the window's left and right columns.
#[derive(Clone, Copy)]
struct Frame<'a> {
    lines: &'a Lines,
    border: Rgb,
    bg: Rgb,
    left: u16,
    right: u16,
}

const LIGHT: Lines = Lines { h: '\u{2500}', v: '\u{2502}', tl: '\u{250c}', tr: '\u{2510}', bl: '\u{2514}', br: '\u{2518}', up: '\u{2534}', down: '\u{252c}', right: '\u{251c}', left: '\u{2524}' };
const HEAVY: Lines = Lines { h: '\u{2501}', v: '\u{2503}', tl: '\u{250f}', tr: '\u{2513}', bl: '\u{2517}', br: '\u{251b}', up: '\u{253b}', down: '\u{2533}', right: '\u{2523}', left: '\u{252b}' };

fn lines(focused: bool) -> &'static Lines {
    if focused {
        &HEAVY
    } else {
        &LIGHT
    }
}

/// 🗗️ The restore glyph of a zoomed window; the theme's repertoire has maximize only.
fn restore_glyph(set: GlyphSet) -> &'static str {
    match set {
        GlyphSet::Unicode => "\u{2921}",
        GlyphSet::Ascii => "v",
    }
}

/// 🎨️ Colours one chip uses, resolved from the tab state.
struct ChipPaint {
    fg: Rgb,
    bg: Rgb,
    attrs: u8,
}

fn chip_paint(w: &WindowState, theme: &Theme, tab: &WindowCornerTab, window_bg: Rgb) -> ChipPaint {
    let hovered = match (w.hover, tab.kind) {
        (Some(WindowHit::Tab(i) | WindowHit::Close(i)), TabKind::Tab) => i == tab.index,
        (Some(WindowHit::OverflowPrev(t)), TabKind::OverflowPrev(target)) => t == target,
        (Some(WindowHit::OverflowNext(t)), TabKind::OverflowNext(target)) => t == target,
        _ => false,
    };
    if tab.kind == TabKind::Tab && tab.index == w.active_stack_tab {
        return ChipPaint { fg: theme.role(Role::ActiveForeground), bg: theme.role(Role::ActiveBase), attrs: attr::BOLD };
    }
    if hovered || (tab.kind == TabKind::Tab && w.drop_target == Some(tab.index)) {
        return ChipPaint { fg: theme.role(Role::Foreground), bg: theme.role(Role::HoverInteractive), attrs: 0 };
    }
    ChipPaint { fg: theme.role(Role::MutedForeground), bg: window_bg, attrs: 0 }
}

/// 🪟 Paints the interior text of one chip and lights the hovered control glyph.
fn paint_chip_text(buf: &mut CellBuffer, w: &WindowState, theme: &Theme, tab: &WindowCornerTab, y: u16, paint: &ChipPaint) {
    let clip = Rect::new(tab.x + 1, y, tab.interior_width, 1);
    buf.fill_rect(clip, Cell::blank(paint.fg, paint.bg));
    buf.put_str(Pos { x: clip.x, y }, &tab.interior, paint.fg, paint.bg, paint.attrs, clip);
    let set = theme.glyphs;
    let mut draw = |x: u16, glyph: &str, fg: Rgb| {
        buf.put_str(Pos { x, y }, glyph, fg, paint.bg, paint.attrs, clip);
    };
    if let (TabKind::Tab, Some(status)) = (tab.kind, tab.status) {
        let active = tab.index == w.active_stack_tab;
        draw(tab.x + 2, status.glyph(set, w.spin), if active { paint.fg } else { theme.role(status.role()) });
    }
    if let Some(x) = tab.close_x {
        draw(x, set.glyph(Glyph::Close), paint.fg);
    }
    if let Some(x) = tab.maximize_x {
        draw(x, if w.zoomed { restore_glyph(set) } else { set.glyph(Glyph::Maximize) }, paint.fg);
    }
    let lit = match (w.hover, tab.kind) {
        (Some(WindowHit::Close(i)), TabKind::Tab) if i == tab.index => tab.close_x,
        (Some(WindowHit::Maximize(_)), TabKind::Controls) => tab.maximize_x,
        (Some(WindowHit::NewTab(_)), TabKind::Controls) => tab.new_x,
        _ => None,
    };
    if let Some(x) = lit {
        if let Some(cell) = buf.get(x, y).copied() {
            buf.put(x, y, Cell { fg: theme.role(Role::Foreground), bg: theme.role(Role::HoverInteractive), attrs: attr::BOLD, ..cell });
        }
    }
}

/// 🔖 Paints one 2-row corner tab. The body hairline is drawn afterwards, so a tab stays closed until the active one is opened.
fn paint_corner_tab(buf: &mut CellBuffer, y: u16, tab: &WindowCornerTab, is_bottom: bool, frame: &Frame<'_>, paint_text: &dyn Fn(&mut CellBuffer, &WindowCornerTab, u16)) {
    let Frame { lines: g, border, bg, .. } = *frame;
    let width = tab.interior_width + 2;
    let text_y = y + 1;
    let outer_y = y + 2;
    let wall = |buf: &mut CellBuffer, x: u16, yy: u16, ch: char| buf.put(x, yy, Cell { ch, fg: border, bg, attrs: 0, width: 1 });
    if is_bottom {
        wall(buf, tab.x, text_y, g.v);
        paint_text(buf, tab, text_y);
        wall(buf, tab.x + width - 1, text_y, g.v);
        wall(buf, tab.x, outer_y, g.bl);
        buf.hline(Pos { x: tab.x + 1, y: outer_y }, width.saturating_sub(2), g.h, border, bg);
        wall(buf, tab.x + width - 1, outer_y, g.br);
    } else {
        wall(buf, tab.x, y, g.tl);
        buf.hline(Pos { x: tab.x + 1, y }, width.saturating_sub(2), g.h, border, bg);
        wall(buf, tab.x + width - 1, y, g.tr);
        wall(buf, tab.x, text_y, g.v);
        paint_text(buf, tab, text_y);
        wall(buf, tab.x + width - 1, text_y, g.v);
    }
}

/// 🪡 Joins one tab to the body hairline. Inactive tabs keep that edge; only the active tab leaves it open.
fn paint_tab_seam(buf: &mut CellBuffer, tab: &WindowTab, y: u16, active: bool, is_bottom: bool, frame: &Frame<'_>) {
    let Frame { lines: g, border, bg, left: window_left, right: window_right } = *frame;
    let left = tab.x;
    let right = tab.x + tab.interior_width + 1;
    let at_left = left == window_left;
    let at_right = right == window_right;
    let (left_ch, right_ch) = match (is_bottom, active) {
        (true, true) => (if at_left { g.v } else { g.tr }, if at_right { g.v } else { g.tl }),
        (true, false) => (if at_left { g.right } else { g.down }, if at_right { g.left } else { g.down }),
        (false, true) => (if at_left { g.v } else { g.br }, if at_right { g.v } else { g.bl }),
        (false, false) => (if at_left { g.right } else { g.up }, if at_right { g.left } else { g.up }),
    };
    buf.put(left, y, Cell { ch: left_ch, fg: border, bg, attrs: 0, width: 1 });
    if tab.interior_width > 0 {
        let ch = if active { ' ' } else { g.h };
        buf.hline(Pos { x: left + 1, y }, tab.interior_width, ch, if active { bg } else { border }, bg);
    }
    buf.put(right, y, Cell { ch: right_ch, fg: border, bg, attrs: 0, width: 1 });
}

/// 🚪 Fills the union of the body and raised chips. Notches stay on the parent surface.
fn fill_silhouette(buf: &mut CellBuffer, rect: Rect, layout: &WindowChipLayout, cell: Cell) {
    if layout.mode != ChromeMode::Full {
        buf.fill_rect(rect, cell);
        return;
    }
    let has_top = layout.groups.iter().any(|group| group.corner.is_top());
    let has_bottom = layout.groups.iter().any(|group| !group.corner.is_top());
    let bottom_y = rect.y + rect.height - 1;
    let top_edge = if has_top { layout.top_body_y } else { rect.y };
    let bottom_edge = if has_bottom { layout.bottom_body_y.unwrap_or(bottom_y) } else { bottom_y };
    if bottom_edge >= top_edge {
        buf.fill_rect(Rect::new(rect.x, top_edge, rect.width, bottom_edge - top_edge + 1), cell);
    }
    for group in &layout.groups {
        for tab in &group.tabs {
            let width = tab.interior_width.saturating_add(2);
            if group.corner.is_top() {
                let height = layout.top_body_y.saturating_sub(rect.y);
                if height > 0 {
                    buf.fill_rect(Rect::new(tab.x, rect.y, width, height), cell);
                }
            } else if let Some(hairline) = layout.bottom_body_y {
                let height = bottom_y.saturating_sub(hairline);
                if height > 0 {
                    buf.fill_rect(Rect::new(tab.x, hairline.saturating_add(1), width, height), cell);
                }
            }
        }
    }
}

/// 📱️ Paints the one-row chrome: tabs sit on the top border between shared walls.
fn paint_compact(buf: &mut CellBuffer, rect: Rect, w: &WindowState, theme: &Theme, layout: &WindowChipLayout, frame: &Frame<'_>) {
    let Frame { lines: g, border, bg, .. } = *frame;
    let right_x = rect.x + rect.width - 1;
    let bottom_y = rect.y + rect.height - 1;
    buf.hline(Pos { x: rect.x + 1, y: rect.y }, rect.width.saturating_sub(2), g.h, border, bg);
    buf.hline(Pos { x: rect.x + 1, y: bottom_y }, rect.width.saturating_sub(2), g.h, border, bg);
    buf.vline(Pos { x: rect.x, y: rect.y + 1 }, rect.height.saturating_sub(2), g.v, border, bg);
    buf.vline(Pos { x: right_x, y: rect.y + 1 }, rect.height.saturating_sub(2), g.v, border, bg);
    for (x, y, ch) in [(rect.x, rect.y, g.tl), (right_x, rect.y, g.tr), (rect.x, bottom_y, g.bl), (right_x, bottom_y, g.br)] {
        buf.put(x, y, Cell { ch, fg: border, bg, attrs: 0, width: 1 });
    }
    for group in &layout.groups {
        for tab in &group.tabs {
            let paint = chip_paint(w, theme, tab, bg);
            let wall = if tab.x == rect.x { g.tl } else { g.down };
            buf.put(tab.x, rect.y, Cell { ch: wall, fg: border, bg, attrs: 0, width: 1 });
            paint_chip_text(buf, w, theme, tab, rect.y, &paint);
            let end = tab.x + tab.interior_width + 1;
            let end_ch = if end == right_x { g.tr } else { g.down };
            buf.put(end, rect.y, Cell { ch: end_ch, fg: border, bg, attrs: 0, width: 1 });
        }
    }
}

/// 🖌️ Paints one closed window outline. Raised chips step out of the body and the notches beside them stay outside it.
pub(crate) fn paint_window(w: &WindowState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    if rect.width < 2 || rect.height < 2 {
        return;
    }
    let bg = theme.surface(Surface::Window);
    let border = if w.focused { theme.role(Role::ActiveBase) } else { theme.role(Role::BorderNormal) };
    let fg = theme.role(Role::Foreground);
    let g = lines(w.focused);
    let bottom_y = rect.y + rect.height - 1;
    let right_x = rect.x + rect.width - 1;
    let layout = window_chip_layout(w, rect);
    let frame = Frame { lines: g, border, bg, left: rect.x, right: right_x };
    fill_silhouette(buf, rect, &layout, Cell::blank(fg, bg));

    if layout.mode == ChromeMode::Flat {
        buf.hline(Pos { x: rect.x + 1, y: rect.y }, rect.width.saturating_sub(2), g.h, border, bg);
        buf.hline(Pos { x: rect.x + 1, y: bottom_y }, rect.width.saturating_sub(2), g.h, border, bg);
        buf.vline(Pos { x: rect.x, y: rect.y + 1 }, bottom_y.saturating_sub(rect.y + 1), g.v, border, bg);
        buf.vline(Pos { x: right_x, y: rect.y + 1 }, bottom_y.saturating_sub(rect.y + 1), g.v, border, bg);
        for (x, y, ch) in [(rect.x, rect.y, g.tl), (right_x, rect.y, g.tr), (rect.x, bottom_y, g.bl), (right_x, bottom_y, g.br)] {
            buf.put(x, y, Cell { ch, fg: border, bg, attrs: 0, width: 1 });
        }
        return;
    }
    if layout.mode == ChromeMode::Compact {
        paint_compact(buf, rect, w, theme, &layout, &frame);
        return;
    }

    let has_top = layout.groups.iter().any(|g| g.corner.is_top());
    let has_bottom = layout.groups.iter().any(|g| !g.corner.is_top());
    let top_body_y = layout.top_body_y;
    let bottom_body_y = layout.bottom_body_y.unwrap_or(bottom_y);

    let wall_top = if has_top { top_body_y + 1 } else { rect.y + 1 };
    let wall_last = if has_bottom { bottom_body_y.saturating_sub(1) } else { bottom_y.saturating_sub(1) };
    if wall_last >= wall_top {
        let len = wall_last - wall_top + 1;
        buf.vline(Pos { x: rect.x, y: wall_top }, len, g.v, border, bg);
        buf.vline(Pos { x: right_x, y: wall_top }, len, g.v, border, bg);
    }

    if !has_top {
        buf.hline(Pos { x: rect.x + 1, y: rect.y }, rect.width.saturating_sub(2), g.h, border, bg);
        buf.put(rect.x, rect.y, Cell { ch: g.tl, fg: border, bg, attrs: 0, width: 1 });
        buf.put(right_x, rect.y, Cell { ch: g.tr, fg: border, bg, attrs: 0, width: 1 });
    }
    if !has_bottom {
        buf.hline(Pos { x: rect.x + 1, y: bottom_y }, rect.width.saturating_sub(2), g.h, border, bg);
        buf.put(rect.x, bottom_y, Cell { ch: g.bl, fg: border, bg, attrs: 0, width: 1 });
        buf.put(right_x, bottom_y, Cell { ch: g.br, fg: border, bg, attrs: 0, width: 1 });
    }

    for group in &layout.groups {
        let is_bottom = !group.corner.is_top();
        let y = if is_bottom { layout.bottom_body_y.unwrap_or(rect.y + rect.height.saturating_sub(3)) } else { rect.y };
        for tab in &group.tabs {
            let paint = chip_paint(w, theme, tab, bg);
            paint_corner_tab(buf, y, tab, is_bottom, &frame, &|buf, tab, text_y| paint_chip_text(buf, w, theme, tab, text_y, &paint));
        }
    }

    let span = rect.width.saturating_sub(2);
    if has_top {
        if span > 0 {
            buf.hline(Pos { x: rect.x + 1, y: top_body_y }, span, g.h, border, bg);
        }
        let has_tr = layout.groups.iter().any(|g| g.corner == WindowStackCorner::TopRight);
        let has_tl = layout.groups.iter().any(|g| g.corner == WindowStackCorner::TopLeft);
        if !has_tl {
            buf.put(rect.x, top_body_y, Cell { ch: g.tl, fg: border, bg, attrs: 0, width: 1 });
        }
        if !has_tr {
            buf.put(right_x, top_body_y, Cell { ch: g.tr, fg: border, bg, attrs: 0, width: 1 });
        }
        for group in layout.groups.iter().filter(|group| group.corner.is_top()) {
            for tab in &group.tabs {
                let open = tab.kind == TabKind::Tab && tab.index == w.active_stack_tab;
                paint_tab_seam(buf, &tab.as_window_tab(), top_body_y, open, false, &frame);
            }
        }
    }

    if has_bottom {
        if span > 0 {
            buf.hline(Pos { x: rect.x + 1, y: bottom_body_y }, span, g.h, border, bg);
        }
        let has_br = layout.groups.iter().any(|g| g.corner == WindowStackCorner::BottomRight);
        let has_bl = layout.groups.iter().any(|g| g.corner == WindowStackCorner::BottomLeft);
        if !has_bl {
            buf.put(rect.x, bottom_body_y, Cell { ch: g.bl, fg: border, bg, attrs: 0, width: 1 });
        }
        if !has_br {
            buf.put(right_x, bottom_body_y, Cell { ch: g.br, fg: border, bg, attrs: 0, width: 1 });
        }
        for group in layout.groups.iter().filter(|group| !group.corner.is_top()) {
            for tab in &group.tabs {
                let open = tab.kind == TabKind::Tab && tab.index == w.active_stack_tab;
                paint_tab_seam(buf, &tab.as_window_tab(), bottom_body_y, open, true, &frame);
            }
        }
    }
}
