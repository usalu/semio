//! 🧙️ tui state, key, pointer and paint functions for the Wizard element: a stepped, filterable option list.
//! The filter is incremental and key-stable (`ListModel`), signals carry option identities, a single press
//! selects and a double press or Enter activates.

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::event::{mods, Key, KeyEvent, MouseEvent};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::rows::{paint_scroll_bar, row_style, ListModel, Pointer};
use crate::tui::text::{display_width, elide, Elision};
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{CursorShape, CursorSpec, WidgetSignal};

/// 🪄 Filterable option list for stepped command building.
pub struct WizardState {
    pub steps: Vec<(String, String)>,
    pub list: ListModel,
    /// 🈳️ The text shown when the filter leaves no option; the application supplies it in the user's language.
    pub empty: String,
}

impl WizardState {
    pub fn new(options: Vec<String>) -> Self {
        Self { steps: Vec::new(), list: ListModel::new(options), empty: String::new() }
    }

    pub fn options(&self) -> &[String] {
        self.list.labels()
    }

    /// 🔁️ Replaces the options; the selected label and the filter survive when they still apply.
    pub fn set_options(&mut self, options: Vec<String>) {
        self.list.set_labels(options);
    }

    pub fn filter(&self) -> &str {
        self.list.query()
    }

    /// 🆔️ The option under the selection, if the filter leaves any.
    pub fn selected_option(&self) -> Option<usize> {
        self.list.selected_option()
    }

    fn header_rows(&self) -> u16 {
        1 + u16::from(!self.steps.is_empty())
    }
}

struct Areas {
    breadcrumb: Option<Rect>,
    filter: Rect,
    list: Rect,
    track: Rect,
}

fn areas(w: &WizardState, rect: Rect) -> Areas {
    let header = w.header_rows().min(rect.height);
    let (top, rest) = rect.split_top(header);
    let (breadcrumb, filter) = if w.steps.is_empty() { (None, top) } else { (Some(Rect::new(top.x, top.y, top.width, top.height.min(1))), Rect::new(top.x, top.y + 1, top.width, top.height.saturating_sub(1))) };
    let overflow = w.list.count() > usize::from(rest.height) && rest.width > 1;
    let list = Rect::new(rest.x, rest.y, rest.width - u16::from(overflow), rest.height);
    Areas { breadcrumb, filter, list, track: Rect::new(rest.x + rest.width.saturating_sub(1), rest.y, 1, rest.height) }
}

fn plain(ev: &KeyEvent) -> bool {
    ev.mods & !mods::SHIFT == 0
}

pub(crate) fn wizard_on_key(w: &mut WizardState, ev: &KeyEvent) -> Option<WidgetSignal> {
    let count = w.list.count();
    if let Some(changed) = w.list.rows.navigate(ev.key, count) {
        return changed.then(|| w.list.selected_option().map(WidgetSignal::SelectionChanged)).flatten();
    }
    match ev.key {
        Key::Enter => w.list.selected_option().map(WidgetSignal::Activated),
        Key::Esc if w.list.query().is_empty() => Some(WidgetSignal::NavigateBack),
        Key::Esc => {
            w.list.set_query("");
            Some(WidgetSignal::ValueChanged(String::new()))
        }
        Key::Backspace if ev.mods & mods::ALT != 0 => Some(WidgetSignal::NavigateBack),
        Key::Left if ev.mods & mods::ALT != 0 => Some(WidgetSignal::NavigateBack),
        Key::Backspace => w.list.pop_query().then(|| WidgetSignal::ValueChanged(w.list.query().to_string())),
        Key::Char('u') if ev.mods == mods::CTRL => w.list.set_query("").then(|| WidgetSignal::ValueChanged(String::new())),
        Key::Char(c) if plain(ev) && !c.is_control() => {
            w.list.push_query(c);
            Some(WidgetSignal::ValueChanged(w.list.query().to_string()))
        }
        _ => None,
    }
}

pub(crate) fn wizard_on_paste(w: &mut WizardState, text: &str) -> Option<WidgetSignal> {
    let mut query = w.list.query().to_string();
    query.extend(text.chars().map(|c| if c.is_whitespace() { ' ' } else { c }).filter(|c| !c.is_control()));
    w.list.set_query(&query).then_some(WidgetSignal::ValueChanged(query))
}

pub(crate) fn wizard_on_mouse(w: &mut WizardState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    let area = areas(w, rect).list;
    let count = w.list.count();
    match w.list.rows.pointer(area, event, count) {
        Pointer::Selected(position) => w.list.option_at(position).map(WidgetSignal::SelectionChanged),
        Pointer::Activated(position) => w.list.option_at(position).map(WidgetSignal::Activated),
        Pointer::Context { position, row } => Some(WidgetSignal::ContextMenu { pos: position, item: row.and_then(|row| w.list.option_at(row)) }),
        Pointer::Scrolled | Pointer::Ignored => None,
    }
}

pub(crate) fn wizard_set_hover(w: &mut WizardState, rect: Rect, pos: Option<Pos>) -> bool {
    let area = areas(w, rect).list;
    let count = w.list.count();
    w.list.rows.hover_at(area, pos, count)
}

pub(crate) fn wizard_cursor(w: &WizardState, rect: Rect) -> Option<CursorSpec> {
    let filter = areas(w, rect).filter;
    (filter.height > 0 && filter.width > 0).then(|| {
        let x = (filter.x + 2 + display_width(w.filter())).min(filter.x + filter.width - 1);
        CursorSpec { pos: Pos { x, y: filter.y }, shape: CursorShape::Bar, blink: true }
    })
}

pub(crate) fn paint_wizard(w: &WizardState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let bg = theme.surface(Surface::Window);
    let muted = theme.role(Role::MutedForeground);
    let ellipsis = theme.glyphs.ellipsis();
    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
    let areas = areas(w, rect);
    if let Some(row) = areas.breadcrumb {
        let separator = format!(" {} ", theme.glyphs.glyph(crate::tui::theme::Glyph::Prompt));
        let crumbs = w.steps.iter().map(|(label, _)| label.as_str()).collect::<Vec<_>>().join(&separator);
        buf.put_str(Pos { x: row.x, y: row.y }, &elide(&crumbs, row.width, Elision::End, ellipsis), muted, bg, 0, row);
    }
    if areas.filter.height > 0 {
        let filter = areas.filter;
        let line = Rect::new(filter.x, filter.y, filter.width, 1);
        let text = format!("/ {}", w.filter());
        buf.put_str(Pos { x: filter.x, y: filter.y }, &elide(&text, filter.width, Elision::Start, ellipsis), if w.filter().is_empty() { muted } else { theme.role(Role::Foreground) }, bg, 0, line);
    }
    let list = areas.list;
    let count = w.list.count();
    let window = w.list.rows.window(usize::from(list.height), count);
    let prompt = theme.glyphs.glyph(crate::tui::theme::Glyph::Prompt);
    for (index, position) in window.clone().enumerate() {
        let y = list.y + index as u16;
        let selected = position == w.list.rows.selected();
        let (fg, row_bg, attrs) = row_style(theme, bg, selected, focused, w.list.rows.hover() == Some(position));
        let line = Rect::new(list.x, y, list.width, 1);
        buf.fill_rect(line, Cell::blank(fg, row_bg));
        let Some(label) = w.list.option_at(position).and_then(|option| w.list.label(option)) else { continue };
        let marker = if selected { prompt } else { " " };
        buf.put_str(Pos { x: list.x, y }, marker, fg, row_bg, attrs, line);
        buf.put_str(Pos { x: list.x + 2, y }, &elide(label, list.width.saturating_sub(2), Elision::End, ellipsis), fg, row_bg, attrs, line);
    }
    if count == 0 && list.height > 0 && !w.empty.is_empty() {
        buf.put_str(Pos { x: list.x, y: list.y }, &elide(&w.empty, list.width, Elision::End, ellipsis), muted, bg, 0, Rect::new(list.x, list.y, list.width, 1));
    }
    paint_scroll_bar(buf, theme, areas.track, window.start, usize::from(list.height), count);
}
