use super::encode::{encode_focus, encode_key, encode_mouse, encode_paste, encode_wheel_as_arrows, MouseReporting};
use super::palette::Palette;
use super::screen::{CellPoint, Match, VtScreen};
use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::event::{mods, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::text::display_width;
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{CursorSpec, WidgetSignal};

const WHEEL_LINES: i64 = 3;

/// 🔤️ How far a selection grows around the cell it started on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionMode {
    Cell,
    Word,
    Line,
}

/// 🖍️ A text selection between two cells in absolute row numbering, so it stays on its text while output scrolls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerminalSelection {
    pub start: CellPoint,
    pub end: CellPoint,
    pub mode: SelectionMode,
    origin: (CellPoint, CellPoint),
}

/// 🔎️ An active scrollback search: the query, every hit, and the hit the viewport shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalSearch {
    pub query: String,
    pub matches: Vec<Match>,
    pub current: usize,
}

/// 📊️ Where the scroll thumb sits on a track of `track` cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scrollbar {
    pub track: u16,
    pub thumb_start: u16,
    pub thumb_len: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Press {
    None,
    Child,
    Select,
    Scrollbar,
}

/// 🖥️ The embedded terminal pane: a VT screen, the viewport over its history, selection, search and the encoders that turn input into child bytes.
pub struct TerminalState {
    pub screen: Box<VtScreen>,
    pub passthrough: bool,
    pub selection: Option<TerminalSelection>,
    search: Option<TerminalSearch>,
    press: Press,
    was_alternate: bool,
}

fn input(bytes: Vec<u8>) -> Option<WidgetSignal> {
    (!bytes.is_empty()).then_some(WidgetSignal::TerminalInput(bytes))
}

impl TerminalState {
    /// 🆕️ A blank pane of `size` keeping `scrollback_cap` history rows (0 means the default); every key goes to the child.
    pub fn new(size: Size, scrollback_cap: usize) -> Self {
        Self { screen: Box::new(VtScreen::new(size, scrollback_cap)), passthrough: true, selection: None, search: None, press: Press::None, was_alternate: false }
    }

    /// 📥️ Feeds child output and returns what the host must act on: answers to the child's queries and clipboard requests.
    pub fn feed(&mut self, bytes: &[u8]) -> Vec<WidgetSignal> {
        self.absorb(bytes);
        let mut signals = Vec::new();
        if let Some(signal) = input(self.screen.take_replies()) {
            signals.push(signal);
        }
        if let Some(text) = self.screen.take_clipboard() {
            signals.push(WidgetSignal::Copy(text));
        }
        signals
    }

    /// ⏪️ Feeds recorded output without answering the queries inside it: the child asked them long ago.
    pub fn feed_replay(&mut self, bytes: &[u8]) {
        self.absorb(bytes);
        self.screen.take_replies();
        self.screen.take_clipboard();
    }

    fn absorb(&mut self, bytes: &[u8]) {
        self.screen.feed(bytes);
        if self.screen.alt_active != self.was_alternate {
            self.was_alternate = self.screen.alt_active;
            self.selection = None;
        }
        self.refresh_search();
    }

    /// 📐️ Resizes the screen, re-wrapping its text; the selection and search hits belong to the old layout and go.
    pub fn resize(&mut self, size: Size) {
        self.screen.resize(size);
        self.selection = None;
        self.refresh_search();
    }

    /// 🪟️ Resizes to `rect` when it differs; true when the child must be told the new size.
    pub fn fit(&mut self, rect: Rect) -> bool {
        let size = Size { width: rect.width.max(1), height: rect.height.max(1) };
        if size == self.screen.size {
            return false;
        }
        self.resize(size);
        true
    }

    /// 🌗️ Tells the child the theme's default colours for its colour queries.
    pub fn apply_theme(&mut self, theme: &Theme) {
        self.screen.palette = Palette::from_theme(theme);
    }

    pub fn set_passthrough(&mut self, passthrough: bool) {
        self.passthrough = passthrough;
    }

    /// ⎋️ Whether Escape still has something to undo here: an open search or a selection.
    pub fn wants_escape(&self) -> bool {
        self.search.is_some() || self.selection.is_some() || !self.passthrough
    }

    pub fn search(&self) -> Option<&TerminalSearch> {
        self.search.as_ref()
    }

    pub fn title(&self) -> Option<&str> {
        self.screen.title.as_deref()
    }

    fn send(&mut self, bytes: Vec<u8>) -> Option<WidgetSignal> {
        if bytes.is_empty() {
            return None;
        }
        self.screen.scroll_view_to_bottom();
        self.selection = None;
        input(bytes)
    }

    /// 🔑️ Handles a key: searching first, then browsing, otherwise everything goes to the child.
    pub fn on_key(&mut self, event: &KeyEvent) -> Option<WidgetSignal> {
        if self.search.is_some() {
            return self.search_key(event);
        }
        if !self.passthrough {
            return self.browse_key(event);
        }
        let bytes = encode_key(event, &self.screen.input_modes())?;
        self.send(bytes)
    }

    fn browse_key(&mut self, event: &KeyEvent) -> Option<WidgetSignal> {
        let page = i64::from(self.screen.size.height);
        let ctrl = event.mods & mods::CTRL != 0;
        match event.key {
            Key::Up | Key::Char('k') if !ctrl => self.screen.scroll_view(1),
            Key::Down | Key::Char('j') if !ctrl => self.screen.scroll_view(-1),
            Key::PageUp => self.screen.scroll_view(page),
            Key::PageDown => self.screen.scroll_view(-page),
            Key::Char('b') if ctrl => self.screen.scroll_view(page),
            Key::Char('f') if ctrl => self.screen.scroll_view(-page),
            Key::Char('u') if ctrl => self.screen.scroll_view(page / 2),
            Key::Char('d') if ctrl => self.screen.scroll_view(-page / 2),
            Key::Home | Key::Char('g') => self.screen.scroll_view_to_top(),
            Key::End | Key::Char('G') => self.screen.scroll_view_to_bottom(),
            Key::Char('n') => return self.step_search(true),
            Key::Char('N') => return self.step_search(false),
            Key::Char('y') | Key::Enter => return self.copy_selection(),
            Key::Char('i') => return Some(self.resume_passthrough()),
            Key::Esc => {
                if self.selection.take().is_some() {
                    return None;
                }
                return Some(self.resume_passthrough());
            }
            _ => {}
        }
        None
    }

    fn resume_passthrough(&mut self) -> WidgetSignal {
        self.passthrough = true;
        self.screen.scroll_view_to_bottom();
        WidgetSignal::Toggled(true)
    }

    fn search_key(&mut self, event: &KeyEvent) -> Option<WidgetSignal> {
        let ctrl = event.mods & mods::CTRL != 0;
        let shift = event.mods & mods::SHIFT != 0;
        match event.key {
            Key::Esc => {
                self.end_search();
                Some(WidgetSignal::ValueChanged(String::new()))
            }
            Key::Enter if !shift => self.step_search(true),
            Key::Enter | Key::Down => self.step_search(false),
            Key::Up => self.step_search(true),
            Key::Char('n') if ctrl => self.step_search(false),
            Key::Char('p') if ctrl => self.step_search(true),
            Key::PageUp => {
                self.screen.scroll_view(i64::from(self.screen.size.height));
                None
            }
            Key::PageDown => {
                self.screen.scroll_view(-i64::from(self.screen.size.height));
                None
            }
            Key::Backspace => self.edit_query(|query| {
                query.pop();
            }),
            Key::Char('u') if ctrl => self.edit_query(String::clear),
            Key::Char(c) if event.mods & !mods::SHIFT == 0 => self.edit_query(|query| query.push(c)),
            _ => None,
        }
    }

    fn edit_query(&mut self, edit: impl FnOnce(&mut String)) -> Option<WidgetSignal> {
        let search = self.search.as_mut()?;
        edit(&mut search.query);
        let query = search.query.clone();
        self.search = Some(TerminalSearch { query: query.clone(), matches: Vec::new(), current: 0 });
        self.refresh_search();
        self.reveal_current();
        Some(WidgetSignal::ValueChanged(query))
    }

    /// 🔍️ Opens the search row; only this call opens it, and Escape always closes it.
    pub fn begin_search(&mut self) {
        if self.search.is_none() {
            self.search = Some(TerminalSearch { query: String::new(), matches: Vec::new(), current: 0 });
        }
    }

    pub fn end_search(&mut self) {
        self.search = None;
    }

    fn refresh_search(&mut self) {
        let Some(search) = self.search.as_mut() else { return };
        let previous = search.matches.get(search.current).copied();
        search.matches = self.screen.find(&search.query);
        search.current = match previous {
            Some(old) => search.matches.iter().position(|hit| (hit.row, hit.col) >= (old.row, old.col)).unwrap_or(search.matches.len().saturating_sub(1)),
            None => search.matches.len().saturating_sub(1),
        };
    }

    /// ⏭️ Moves to the next older hit (`older`) or the next newer one, wrapping at the ends.
    pub fn step_search(&mut self, older: bool) -> Option<WidgetSignal> {
        let search = self.search.as_mut()?;
        let count = search.matches.len();
        if count == 0 {
            return None;
        }
        search.current = if older { (search.current + count - 1) % count } else { (search.current + 1) % count };
        let current = search.current;
        self.reveal_current();
        Some(WidgetSignal::SelectionChanged(current))
    }

    fn reveal_current(&mut self) {
        let Some(hit) = self.search.as_ref().and_then(|search| search.matches.get(search.current).copied()) else { return };
        if hit.row >= self.screen.screen_top() {
            self.screen.scroll_view_to_bottom();
            return;
        }
        let top = hit.row.saturating_sub(u64::from(self.screen.size.height / 2));
        let offset = self.screen.screen_top().saturating_sub(top);
        self.screen.set_view_offset(offset as usize);
    }

    fn point_at(&self, local: Pos) -> CellPoint {
        CellPoint { row: self.screen.view_top() + u64::from(local.y), col: local.x.min(self.screen.size.width - 1) }
    }

    fn span_of(&self, mode: SelectionMode, point: CellPoint) -> (CellPoint, CellPoint) {
        match mode {
            SelectionMode::Cell => (point, point),
            SelectionMode::Word => self.screen.word_span(point),
            SelectionMode::Line => self.screen.line_span(point),
        }
    }

    fn begin_selection(&mut self, mode: SelectionMode, point: CellPoint) {
        let span = self.span_of(mode, point);
        self.selection = Some(TerminalSelection { start: span.0, end: span.1, mode, origin: span });
    }

    fn extend_selection(&mut self, point: CellPoint) {
        let Some(selection) = self.selection else { return };
        let span = self.span_of(selection.mode, point);
        self.selection = Some(TerminalSelection { start: selection.origin.0.min(span.0), end: selection.origin.1.max(span.1), ..selection });
    }

    /// 📎️ Copies the selection as a `Copy` signal and clears it.
    pub fn copy_selection(&mut self) -> Option<WidgetSignal> {
        let selection = self.selection.take()?;
        Some(WidgetSignal::Copy(self.screen.text_between(selection.start, selection.end)))
    }

    /// ✍️ The text under the selection, if there is one.
    pub fn selected_text(&self) -> Option<String> {
        self.selection.map(|selection| self.screen.text_between(selection.start, selection.end))
    }

    /// 🧼️ Drops the selection.
    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    /// 🔲️ Selects the whole viewport.
    pub fn select_all(&mut self) {
        let top = self.screen.view_top();
        let start = CellPoint { row: top, col: 0 };
        let end = CellPoint { row: top + u64::from(self.screen.size.height) - 1, col: self.screen.size.width - 1 };
        self.selection = Some(TerminalSelection { start, end, mode: SelectionMode::Cell, origin: (start, end) });
    }

    /// 📈️ The scroll thumb for a track `track` cells tall; `None` when there is no history to scroll.
    pub fn scrollbar(&self, track: u16) -> Option<Scrollbar> {
        let history = self.screen.scrollback_len() as u64;
        let height = u64::from(self.screen.size.height);
        if history == 0 || track == 0 || self.screen.alt_active {
            return None;
        }
        let total = history + height;
        let track_cells = u64::from(track);
        let thumb_len = (track_cells * height / total).clamp(1, track_cells);
        let top = self.screen.view_top() - self.screen.first_row();
        let span = total - height;
        let thumb_start = (top.min(span) * (track_cells - thumb_len)).checked_div(span).unwrap_or(0) as u16;
        Some(Scrollbar { track, thumb_start, thumb_len: thumb_len as u16 })
    }

    fn drag_scrollbar(&mut self, local: Pos, rect: Rect) {
        let history = self.screen.scrollback_len() as u64;
        let track = u64::from(rect.height.saturating_sub(1).max(1));
        let top = history * u64::from(local.y).min(track) / track;
        self.screen.set_view_offset((history - top.min(history)) as usize);
    }

    /// 🖲️ Handles a pointer event over `rect`: reports go to the child when it tracks the pointer, otherwise the pane selects, scrolls and opens links.
    pub fn on_mouse(&mut self, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
        if rect.width == 0 || rect.height == 0 {
            return None;
        }
        let local = Pos { x: event.pos.x.saturating_sub(rect.x).min(rect.width - 1), y: event.pos.y.saturating_sub(rect.y).min(rect.height - 1) };
        let modes = self.screen.input_modes();
        let shift = event.mods & mods::SHIFT != 0;
        let child = self.passthrough && modes.mouse != MouseReporting::Off && !shift;
        match event.kind {
            MouseKind::Down(button) => {
                if child {
                    self.press = Press::Child;
                    self.screen.scroll_view_to_bottom();
                    return input(encode_mouse(event, local, &modes));
                }
                self.press_local(button, event, rect, local)
            }
            MouseKind::Drag(_) => match self.press {
                Press::Child => input(encode_mouse(event, local, &modes)),
                Press::Select => {
                    let point = self.drag_point(event, rect, local);
                    self.extend_selection(point);
                    None
                }
                Press::Scrollbar => {
                    self.drag_scrollbar(local, rect);
                    None
                }
                Press::None => None,
            },
            MouseKind::Up(button) => {
                let press = std::mem::replace(&mut self.press, Press::None);
                match press {
                    Press::Child => input(encode_mouse(event, local, &modes)),
                    Press::Select if button == MouseButton::Left => self.finish_selection(),
                    _ => None,
                }
            }
            MouseKind::Move => {
                if child && modes.mouse == MouseReporting::Motion {
                    input(encode_mouse(event, local, &modes))
                } else {
                    None
                }
            }
            MouseKind::Scroll { dy, .. } => {
                if child {
                    input(encode_mouse(event, local, &modes))
                } else if modes.alt_screen && modes.alt_scroll && self.passthrough && !shift {
                    input(encode_wheel_as_arrows(dy, &modes))
                } else {
                    self.screen.scroll_view(-i64::from(dy) * WHEEL_LINES);
                    None
                }
            }
        }
    }

    fn press_local(&mut self, button: MouseButton, event: &MouseEvent, rect: Rect, local: Pos) -> Option<WidgetSignal> {
        match button {
            MouseButton::Left => {
                if self.screen.view_offset() > 0 && local.x == rect.width - 1 && rect.width > 2 {
                    self.press = Press::Scrollbar;
                    self.drag_scrollbar(local, rect);
                    return None;
                }
                let point = self.point_at(local);
                if event.mods & mods::CTRL != 0 {
                    if let Some(url) = self.screen.url_at(point) {
                        return Some(WidgetSignal::OpenUrl(url));
                    }
                }
                let mode = match event.clicks {
                    0 | 1 => SelectionMode::Cell,
                    2 => SelectionMode::Word,
                    _ => SelectionMode::Line,
                };
                self.begin_selection(mode, point);
                self.press = Press::Select;
                None
            }
            MouseButton::Right => Some(WidgetSignal::ContextMenu { pos: event.pos, item: None }),
            MouseButton::Middle => None,
        }
    }

    fn drag_point(&mut self, event: &MouseEvent, rect: Rect, local: Pos) -> CellPoint {
        if event.pos.y < rect.y {
            self.screen.scroll_view(1);
            return CellPoint { row: self.screen.view_top(), col: if event.pos.x < rect.x { 0 } else { local.x } };
        }
        if event.pos.y >= rect.y + rect.height {
            self.screen.scroll_view(-1);
            return CellPoint { row: self.screen.view_top() + u64::from(rect.height) - 1, col: local.x };
        }
        self.point_at(local)
    }

    fn finish_selection(&mut self) -> Option<WidgetSignal> {
        let selection = self.selection?;
        if selection.mode == SelectionMode::Cell && selection.start == selection.end {
            self.selection = None;
            return None;
        }
        Some(WidgetSignal::Copy(self.screen.text_between(selection.start, selection.end)))
    }

    /// 📃️ Pastes `text` into the search row, or into the child as one bracketed paste when it asked for that.
    pub fn on_paste(&mut self, text: &str) -> Option<WidgetSignal> {
        if self.search.is_some() {
            return self.edit_query(|query| query.extend(text.chars().filter(|c| !c.is_control())));
        }
        if !self.passthrough {
            return None;
        }
        let bytes = encode_paste(text, &self.screen.input_modes());
        self.send(bytes)
    }

    /// 🔦️ Reports focus to the child when it asked for focus events.
    pub fn on_focus(&mut self, gained: bool) -> Option<WidgetSignal> {
        input(encode_focus(gained, &self.screen.input_modes())?)
    }

    /// ✏️ Where the hardware cursor belongs in `rect`: the search row's caret while searching, otherwise the child's cursor when visible on screen.
    pub fn cursor(&self, rect: Rect) -> Option<CursorSpec> {
        if rect.width == 0 || rect.height == 0 {
            return None;
        }
        if let Some(search) = &self.search {
            let x = (1 + display_width(&search.query)).min(rect.width - 1);
            return Some(CursorSpec { pos: Pos { x: rect.x + x, y: rect.y + rect.height - 1 }, shape: crate::tui::widget::CursorShape::Bar, blink: true });
        }
        if !self.screen.modes.cursor_visible {
            return None;
        }
        let absolute = self.screen.screen_top() + u64::from(self.screen.cursor.y);
        let relative = absolute.checked_sub(self.screen.view_top())?;
        if relative >= u64::from(rect.height) || self.screen.cursor.x >= rect.width {
            return None;
        }
        Some(CursorSpec { pos: Pos { x: rect.x + self.screen.cursor.x, y: rect.y + relative as u16 }, shape: self.screen.modes.cursor_shape, blink: self.screen.modes.cursor_blink })
    }

    /// 🖌️ Paints the viewport with the theme's colours, then selection, search hits, scroll bar and search row.
    pub fn paint(&self, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        let mut palette = Palette::from_theme(theme);
        if self.screen.modes.reverse_video {
            std::mem::swap(&mut palette.fg, &mut palette.bg);
        }
        buf.fill_rect(rect, Cell::blank(palette.fg, palette.bg));
        if rect.width == 0 || rect.height == 0 {
            return;
        }
        self.screen.blit(buf, rect, &palette);
        let top = self.screen.view_top();
        if let Some(search) = &self.search {
            for (index, hit) in search.matches.iter().enumerate() {
                let current = index == search.current;
                let (fg, bg, bold) = if current { (theme.role(Role::ActiveForeground), theme.role(Role::ActiveBase), attr::BOLD) } else { (theme.role(Role::AccentForeground), theme.role(Role::Accent), 0) };
                self.paint_span(buf, rect, top, hit.row, hit.col..=hit.col + hit.cells - 1, |cell| Cell { fg, bg, attrs: cell.attrs | bold, ..cell });
            }
        }
        if let Some(selection) = self.selection {
            let last = self.screen.size.width - 1;
            let visible = top..top + u64::from(rect.height);
            for row in selection.start.row.max(visible.start)..=selection.end.row.min(visible.end - 1) {
                let from = if row == selection.start.row { selection.start.col } else { 0 };
                let to = if row == selection.end.row { selection.end.col } else { last };
                self.paint_span(buf, rect, top, row, from..=to, |cell| Cell { fg: cell.bg, bg: cell.fg, ..cell });
            }
        }
        self.paint_scroll_cues(theme, rect, buf);
        if let Some(search) = &self.search {
            self.paint_search_row(theme, rect, buf, search);
        }
    }

    fn paint_span(&self, buf: &mut CellBuffer, rect: Rect, top: u64, row: u64, columns: std::ops::RangeInclusive<u16>, restyle: impl Fn(Cell) -> Cell) {
        let Some(y) = row.checked_sub(top).filter(|y| *y < u64::from(rect.height)) else { return };
        for col in *columns.start()..=(*columns.end()).min(rect.width - 1) {
            let (x, y) = (rect.x + col, rect.y + y as u16);
            if let Some(cell) = buf.get(x, y).copied() {
                buf.put(x, y, restyle(cell));
            }
        }
    }

    fn paint_scroll_cues(&self, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        let offset = self.screen.view_offset();
        if offset == 0 {
            return;
        }
        let bg = theme.surface(Surface::Window);
        if let Some(bar) = self.scrollbar(rect.height) {
            let x = rect.x + rect.width - 1;
            for y in 0..rect.height {
                let thumb = y >= bar.thumb_start && y < bar.thumb_start + bar.thumb_len;
                let (glyph, fg) = if thumb { ('┃', theme.role(Role::Accent)) } else { ('│', theme.role(Role::BorderNormal)) };
                buf.put(x, rect.y + y, Cell { ch: glyph, fg, bg, attrs: 0, width: 1 });
            }
        }
        let label = format!("↓{offset}");
        let width = display_width(&label);
        if width + 1 < rect.width {
            let at = Pos { x: rect.x + rect.width - 1 - width, y: rect.y + rect.height - 1 };
            buf.put_str(at, &label, theme.role(Role::Accent), bg, attr::BOLD, Rect::new(rect.x, at.y, rect.width, 1));
        }
    }

    fn paint_search_row(&self, theme: &Theme, rect: Rect, buf: &mut CellBuffer, search: &TerminalSearch) {
        let y = rect.y + rect.height - 1;
        let row = Rect::new(rect.x, y, rect.width, 1);
        let bg = theme.surface(Surface::Panel);
        let fg = theme.role(Role::Foreground);
        buf.fill_rect(row, Cell::blank(fg, bg));
        buf.put_str(Pos { x: rect.x, y }, &format!("/{}", search.query), fg, bg, 0, row);
        let count = if search.matches.is_empty() { "0".to_string() } else { format!("{}/{}", search.current + 1, search.matches.len()) };
        let width = display_width(&count);
        if width + 2 < rect.width {
            buf.put_str(Pos { x: rect.x + rect.width - width, y }, &count, theme.role(Role::MutedForeground), bg, 0, row);
        }
    }
}
