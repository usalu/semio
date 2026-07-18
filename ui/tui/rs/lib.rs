//! 🖥️ Handcrafted retained-mode terminal UI: semio-styled scene, cell renderer, and ANSI backend.

// #region 🔖Geometry
pub mod geometry {
    /// 📍 A cell coordinate on the terminal grid.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct Pos {
        pub x: u16,
        pub y: u16,
    }

    /// 📏 A cell-grid size.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct Size {
        pub width: u16,
        pub height: u16,
    }

    /// 🔲 An axis-aligned cell-grid rectangle.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct Rect {
        pub x: u16,
        pub y: u16,
        pub width: u16,
        pub height: u16,
    }

    impl Rect {
        pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
            Self { x, y, width, height }
        }

        /// 🧭 Whether `pos` lies within this rect.
        pub fn contains(&self, pos: Pos) -> bool {
            pos.x >= self.x && pos.x < self.x + self.width && pos.y >= self.y && pos.y < self.y + self.height
        }

        /// ✂️ The overlap between two rects (an empty rect on miss).
        pub fn intersect(&self, other: Rect) -> Rect {
            let x0 = self.x.max(other.x);
            let y0 = self.y.max(other.y);
            let x1 = (self.x + self.width).min(other.x + other.width);
            let y1 = (self.y + self.height).min(other.y + other.height);
            if x1 <= x0 || y1 <= y0 {
                Rect::default()
            } else {
                Rect::new(x0, y0, x1 - x0, y1 - y0)
            }
        }

        /// 🧊 Shrinks the rect by `margin` cells on every side.
        pub fn inset(&self, margin: u16) -> Rect {
            self.inset_sides(margin, margin, margin, margin)
        }

        /// 🧊 Shrinks the rect by `top`/`right`/`bottom`/`left` cells.
        pub fn inset_sides(&self, top: u16, right: u16, bottom: u16, left: u16) -> Rect {
            let width = self.width.saturating_sub(left + right);
            let height = self.height.saturating_sub(top + bottom);
            Rect::new(self.x + left.min(self.width), self.y + top.min(self.height), width, height)
        }

        /// ✂️ Splits off `rows` rows from the top, returning `(top, rest)`.
        pub fn split_top(&self, rows: u16) -> (Rect, Rect) {
            let rows = rows.min(self.height);
            let top = Rect::new(self.x, self.y, self.width, rows);
            let rest = Rect::new(self.x, self.y + rows, self.width, self.height - rows);
            (top, rest)
        }

        /// ✂️ Splits off `rows` rows from the bottom, returning `(rest, bottom)`.
        pub fn split_bottom(&self, rows: u16) -> (Rect, Rect) {
            let rows = rows.min(self.height);
            let bottom = Rect::new(self.x, self.y + self.height - rows, self.width, rows);
            let rest = Rect::new(self.x, self.y, self.width, self.height - rows);
            (rest, bottom)
        }
    }
}
// #endregion 🔖Geometry

// #region 🔖Theme
pub mod theme {
    use ui_styling::appearance::AppearanceName;
    use ui_styling::color::linear_to_rgba8;
    use ui_styling::ChromePalette;

    /// 🎨 An 8-bit truecolor triple.
    pub type Rgb = [u8; 3];

    /// 🪟 The four nested semio chrome surfaces (base → canvas → window → panel).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Surface {
        Base,
        Canvas,
        Window,
        Panel,
    }

    /// 🏷️ A semantic foreground/border/state role, resolved against the active palette.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Role {
        Foreground,
        MutedForeground,
        Accent,
        AccentForeground,
        ActiveBase,
        ActiveForeground,
        BorderNormal,
        BorderEmphasized,
        BorderElement,
        HoverWindow,
        HoverPanel,
        Temporary,
    }

    fn rgb(channel: [f32; 4]) -> Rgb {
        let [r, g, b, _a] = linear_to_rgba8(channel[0], channel[1], channel[2], channel[3]);
        [r, g, b]
    }

    /// 🖌️ A resolved semio theme: every chrome color precomputed once as 8-bit truecolor.
    pub struct Theme {
        pub appearance: AppearanceName,
        base: Rgb,
        canvas: Rgb,
        window: Rgb,
        panel: Rgb,
        foreground: Rgb,
        muted_foreground: Rgb,
        accent: Rgb,
        accent_foreground: Rgb,
        active_base: Rgb,
        active_foreground: Rgb,
        border_normal: Rgb,
        border_emphasized: Rgb,
        border_element: Rgb,
        hover_window: Rgb,
        hover_panel: Rgb,
        temporary: Rgb,
    }

    impl Theme {
        pub fn new(appearance: AppearanceName) -> Self {
            let p: &ChromePalette = appearance.chrome();
            Self {
                appearance,
                base: rgb(p.base),
                canvas: rgb(p.canvas),
                window: rgb(p.window),
                panel: rgb(p.panel),
                foreground: rgb(p.foreground),
                muted_foreground: rgb(p.muted_foreground),
                accent: rgb(p.accent),
                accent_foreground: rgb(p.accent_foreground),
                active_base: rgb(p.active_base),
                active_foreground: rgb(p.active_foreground),
                border_normal: rgb(p.border_normal),
                border_emphasized: rgb(p.border_emphasized),
                border_element: rgb(p.border_element),
                hover_window: rgb(p.hover_window),
                hover_panel: rgb(p.hover_panel),
                temporary: rgb(p.temporary),
            }
        }

        pub fn surface(&self, surface: Surface) -> Rgb {
            match surface {
                Surface::Base => self.base,
                Surface::Canvas => self.canvas,
                Surface::Window => self.window,
                Surface::Panel => self.panel,
            }
        }

        pub fn role(&self, role: Role) -> Rgb {
            match role {
                Role::Foreground => self.foreground,
                Role::MutedForeground => self.muted_foreground,
                Role::Accent => self.accent,
                Role::AccentForeground => self.accent_foreground,
                Role::ActiveBase => self.active_base,
                Role::ActiveForeground => self.active_foreground,
                Role::BorderNormal => self.border_normal,
                Role::BorderEmphasized => self.border_emphasized,
                Role::BorderElement => self.border_element,
                Role::HoverWindow => self.hover_window,
                Role::HoverPanel => self.hover_panel,
                Role::Temporary => self.temporary,
            }
        }

        pub fn set_appearance(&mut self, appearance: AppearanceName) {
            *self = Theme::new(appearance);
        }
    }
}
// #endregion 🔖Theme

// #region 🔖Text
pub mod text {
    /// 📐 Terminal cell width of one `char` (0 for zero-width, 1 normal, 2 wide).
    pub(crate) fn char_cells(c: char) -> u8 {
        match unicode_width::UnicodeWidthChar::width(c) {
            Some(w) => w.min(2) as u8,
            None => 0,
        }
    }

    /// 📏 Total display width in cells of a string.
    pub fn display_width(s: &str) -> u16 {
        s.chars().map(|c| u16::from(char_cells(c))).sum()
    }

    /// ✂️ Truncates `s` to at most `max_cells` display cells, returning the slice and its width.
    pub fn truncate_to(s: &str, max_cells: u16) -> (&str, u16) {
        let mut used = 0u16;
        let mut end = 0usize;
        for (idx, c) in s.char_indices() {
            let w = u16::from(char_cells(c));
            if used + w > max_cells {
                break;
            }
            used += w;
            end = idx + c.len_utf8();
        }
        (&s[..end], used)
    }
}
// #endregion 🔖Text

// #region 🔖Cell
pub mod cell {
    use crate::geometry::{Pos, Rect, Size};
    use crate::text::char_cells;
    use crate::theme::Rgb;

    /// 🎛️ Bitflags for cell text attributes.
    pub mod attr {
        pub const BOLD: u8 = 1;
        pub const DIM: u8 = 2;
        pub const ITALIC: u8 = 4;
        pub const UNDERLINE: u8 = 8;
        pub const REVERSE: u8 = 16;
    }

    /// 🧱 One terminal cell: a glyph, its colors, attributes, and cell width (0 = wide-char continuation).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Cell {
        pub ch: char,
        pub fg: Rgb,
        pub bg: Rgb,
        pub attrs: u8,
        pub width: u8,
    }

    impl Cell {
        pub fn blank(fg: Rgb, bg: Rgb) -> Self {
            Self { ch: ' ', fg, bg, attrs: 0, width: 1 }
        }
    }

    /// 🗺️ A retained grid of `Cell`s.
    #[derive(Clone)]
    pub struct CellBuffer {
        pub size: Size,
        cells: Vec<Cell>,
    }

    impl CellBuffer {
        pub fn new(size: Size, fill: Cell) -> Self {
            let count = usize::from(size.width) * usize::from(size.height);
            Self { size, cells: vec![fill; count] }
        }

        pub fn resize(&mut self, size: Size, fill: Cell) {
            *self = Self::new(size, fill);
        }

        fn index(&self, x: u16, y: u16) -> Option<usize> {
            if x < self.size.width && y < self.size.height {
                Some(usize::from(y) * usize::from(self.size.width) + usize::from(x))
            } else {
                None
            }
        }

        pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
            self.index(x, y).map(|i| &self.cells[i])
        }

        /// ✍️ Writes one cell, blanking an orphaned wide-char continuation on either side.
        pub fn put(&mut self, x: u16, y: u16, mut cell: Cell) {
            let Some(i) = self.index(x, y) else { return };
            if cell.width == 0 && x > 0 {
                if let Some(prev) = self.index(x - 1, y) {
                    if self.cells[prev].width == 2 {
                        // keep continuation paired with its lead cell
                    } else {
                        cell.width = 1;
                    }
                }
            }
            if cell.width == 2 && x + 1 >= self.size.width {
                cell.width = 1;
            }
            self.cells[i] = cell;
            if cell.width == 2 {
                if let Some(next) = self.index(x + 1, y) {
                    self.cells[next] = Cell { ch: '\0', width: 0, ..cell };
                }
            }
        }

        /// ✍️ Writes a string starting at `pos`, clipped to `clip`; returns cells consumed.
        pub fn put_str(&mut self, pos: Pos, s: &str, fg: Rgb, bg: Rgb, attrs: u8, clip: Rect) -> u16 {
            let mut x = pos.x;
            let mut written = 0u16;
            for c in s.chars() {
                let w = char_cells(c);
                if w == 0 {
                    continue;
                }
                if x + u16::from(w) > clip.x + clip.width || pos.y < clip.y || pos.y >= clip.y + clip.height {
                    break;
                }
                if x >= clip.x {
                    self.put(x, pos.y, Cell { ch: c, fg, bg, attrs, width: w });
                    if w == 2 {
                        self.put(x + 1, pos.y, Cell { ch: '\0', fg, bg, attrs, width: 0 });
                    }
                }
                x += u16::from(w);
                written += u16::from(w);
            }
            written
        }

        pub fn fill_rect(&mut self, rect: Rect, cell: Cell) {
            let clipped = Rect::new(0, 0, self.size.width, self.size.height).intersect(rect);
            for y in clipped.y..clipped.y + clipped.height {
                for x in clipped.x..clipped.x + clipped.width {
                    self.put(x, y, cell);
                }
            }
        }

        pub fn hline(&mut self, pos: Pos, len: u16, ch: char, fg: Rgb, bg: Rgb) {
            for i in 0..len {
                self.put(pos.x + i, pos.y, Cell { ch, fg, bg, attrs: 0, width: 1 });
            }
        }

        pub fn vline(&mut self, pos: Pos, len: u16, ch: char, fg: Rgb, bg: Rgb) {
            for i in 0..len {
                self.put(pos.x, pos.y + i, Cell { ch, fg, bg, attrs: 0, width: 1 });
            }
        }
    }

    /// 🩹 A contiguous run of changed cells on one row.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct DiffRun {
        pub y: u16,
        pub x: u16,
        pub len: u16,
    }

    /// 🔍 Computes the minimal set of changed-cell runs between two same-sized buffers.
    pub fn diff(prev: &CellBuffer, next: &CellBuffer) -> Vec<DiffRun> {
        const MERGE_GAP: u16 = 4;
        let mut runs = Vec::new();
        if prev.size != next.size {
            return vec![DiffRun { y: 0, x: 0, len: next.size.width * next.size.height }];
        }
        for y in 0..next.size.height {
            let mut run_start: Option<u16> = None;
            let mut last_diff: Option<u16> = None;
            for x in 0..next.size.width {
                let changed = prev.get(x, y) != next.get(x, y);
                if changed {
                    match (run_start, last_diff) {
                        (None, _) => run_start = Some(x),
                        (Some(_), Some(last)) if x - last > MERGE_GAP => {
                            runs.push(DiffRun { y, x: run_start.unwrap(), len: last - run_start.unwrap() + 1 });
                            run_start = Some(x);
                        }
                        _ => {}
                    }
                    last_diff = Some(x);
                }
            }
            if let (Some(start), Some(last)) = (run_start, last_diff) {
                runs.push(DiffRun { y, x: start, len: last - start + 1 });
            }
        }
        runs
    }
}
// #endregion 🔖Cell

// #region 🔖Ansi
pub mod ansi {
    use crate::cell::{Cell, CellBuffer, DiffRun};
    use crate::theme::Rgb;

    //#region 🔖Emit
    /// 📦 A batch of raw ANSI bytes ready to write to a terminal (or feed to xterm.js).
    #[derive(Default, Clone)]
    pub struct AnsiPatch(pub String);

    #[derive(Clone, Copy, PartialEq)]
    struct SgrState {
        fg: Option<Rgb>,
        bg: Option<Rgb>,
        attrs: u8,
    }

    fn push_sgr(out: &mut String, state: &mut SgrState, cell: &Cell) {
        if state.fg == Some(cell.fg) && state.bg == Some(cell.bg) && state.attrs == cell.attrs {
            return;
        }
        out.push_str("\x1b[0");
        if cell.attrs & crate::cell::attr::BOLD != 0 {
            out.push_str(";1");
        }
        if cell.attrs & crate::cell::attr::DIM != 0 {
            out.push_str(";2");
        }
        if cell.attrs & crate::cell::attr::ITALIC != 0 {
            out.push_str(";3");
        }
        if cell.attrs & crate::cell::attr::UNDERLINE != 0 {
            out.push_str(";4");
        }
        if cell.attrs & crate::cell::attr::REVERSE != 0 {
            out.push_str(";7");
        }
        out.push_str(&format!(";38;2;{};{};{}", cell.fg[0], cell.fg[1], cell.fg[2]));
        out.push_str(&format!(";48;2;{};{};{}", cell.bg[0], cell.bg[1], cell.bg[2]));
        out.push('m');
        *state = SgrState { fg: Some(cell.fg), bg: Some(cell.bg), attrs: cell.attrs };
    }

    /// 🖨️ Emits the minimal ANSI needed to repaint `runs` of `next` onto a terminal.
    pub fn emit_runs(next: &CellBuffer, runs: &[DiffRun], out: &mut AnsiPatch) {
        let mut state = SgrState { fg: None, bg: None, attrs: u8::MAX };
        for run in runs {
            out.0.push_str(&format!("\x1b[{};{}H", run.y + 1, run.x + 1));
            let mut x = run.x;
            while x < run.x + run.len {
                let Some(c) = next.get(x, run.y) else { break };
                if c.width == 0 {
                    x += 1;
                    continue;
                }
                push_sgr(&mut out.0, &mut state, c);
                out.0.push(if c.ch == '\0' { ' ' } else { c.ch });
                x += u16::from(c.width.max(1));
            }
        }
    }

    /// 🚪 Enters the alternate screen, hides the cursor, and enables mouse/paste reporting.
    pub fn setup_sequence() -> &'static str {
        "\x1b[?1049h\x1b[?25l\x1b[?1002h\x1b[?1006h\x1b[?2004h\x1b[2J"
    }

    /// 🚪 Restores the primary screen and default modes.
    pub fn teardown_sequence() -> &'static str {
        "\x1b[?2004l\x1b[?1006l\x1b[?1002l\x1b[?25h\x1b[?1049l\x1b[0m"
    }
    //#endregion 🔖Emit

    //#region 🔖Parse
    use crate::event::{mods, Event, Key, KeyEvent, MouseEvent, MouseKind};
    use crate::geometry::Pos;

    #[derive(Clone, Copy, PartialEq)]
    enum ParserState {
        Ground,
        Escape,
        Csi,
        Ss3,
        Osc,
        Paste,
    }

    /// ⌨️ Handcrafted incremental ANSI input decoder (keys, mouse, paste, focus, UTF-8).
    pub struct AnsiParser {
        state: ParserState,
        params: Vec<u16>,
        current: u16,
        has_current: bool,
        private: Option<u8>,
        utf8_buf: [u8; 4],
        utf8_len: u8,
        utf8_need: u8,
        paste_buf: String,
        paste_close: Vec<u8>,
        pending_esc: bool,
    }

    impl Default for AnsiParser {
        fn default() -> Self {
            Self::new()
        }
    }

    fn modifier_bits(code: u16) -> u8 {
        if code == 0 {
            return 0;
        }
        let m = code.saturating_sub(1);
        let mut bits = 0u8;
        if m & 1 != 0 {
            bits |= mods::SHIFT;
        }
        if m & 2 != 0 {
            bits |= mods::ALT;
        }
        if m & 4 != 0 {
            bits |= mods::CTRL;
        }
        bits
    }

    impl AnsiParser {
        pub fn new() -> Self {
            Self {
                state: ParserState::Ground,
                params: Vec::new(),
                current: 0,
                has_current: false,
                private: None,
                utf8_buf: [0; 4],
                utf8_len: 0,
                utf8_need: 0,
                paste_buf: String::new(),
                paste_close: Vec::new(),
                pending_esc: false,
            }
        }

        fn reset_seq(&mut self) {
            self.params.clear();
            self.current = 0;
            self.has_current = false;
            self.private = None;
        }

        fn push_param(&mut self) {
            self.params.push(if self.has_current { self.current } else { 0 });
            self.current = 0;
            self.has_current = false;
        }

        fn param(&self, i: usize, default: u16) -> u16 {
            self.params.get(i).copied().unwrap_or(default)
        }

        /// 📥 Feeds raw input bytes, appending any decoded events to `out`.
        pub fn feed(&mut self, bytes: &[u8], out: &mut Vec<Event>) {
            for &b in bytes {
                self.feed_byte(b, out);
            }
        }

        /// ⏱️ Resolves a lone pending ESC (no follow-up byte arrived before a poll timeout).
        pub fn flush_escape(&mut self, out: &mut Vec<Event>) {
            if self.pending_esc && self.state == ParserState::Escape {
                out.push(Event::Key(KeyEvent { key: Key::Esc, mods: 0 }));
                self.state = ParserState::Ground;
                self.pending_esc = false;
            }
        }

        fn feed_byte(&mut self, b: u8, out: &mut Vec<Event>) {
            if self.utf8_need > 0 {
                self.utf8_buf[self.utf8_len as usize] = b;
                self.utf8_len += 1;
                self.utf8_need -= 1;
                if self.utf8_need == 0 {
                    if let Ok(s) = std::str::from_utf8(&self.utf8_buf[..self.utf8_len as usize]) {
                        if let Some(c) = s.chars().next() {
                            out.push(Event::Key(KeyEvent { key: Key::Char(c), mods: 0 }));
                        }
                    }
                    self.utf8_len = 0;
                }
                return;
            }
            match self.state {
                ParserState::Ground => self.feed_ground(b, out),
                ParserState::Escape => self.feed_escape(b, out),
                ParserState::Csi => self.feed_csi(b, out),
                ParserState::Ss3 => self.feed_ss3(b, out),
                ParserState::Osc => {
                    if b == 0x07 || b == 0x1b {
                        self.state = ParserState::Ground;
                    }
                }
                ParserState::Paste => self.feed_paste(b, out),
            }
        }

        fn feed_ground(&mut self, b: u8, out: &mut Vec<Event>) {
            match b {
                0x1b => {
                    self.state = ParserState::Escape;
                    self.pending_esc = true;
                }
                0x0d | 0x0a => out.push(Event::Key(KeyEvent { key: Key::Enter, mods: 0 })),
                0x09 => out.push(Event::Key(KeyEvent { key: Key::Tab, mods: 0 })),
                0x7f | 0x08 => out.push(Event::Key(KeyEvent { key: Key::Backspace, mods: 0 })),
                0x00..=0x1a if b != 0x1b => {
                    let c = (b'a' + b - 1) as char;
                    out.push(Event::Key(KeyEvent { key: Key::Char(c), mods: mods::CTRL }));
                }
                0x00..=0x7f => out.push(Event::Key(KeyEvent { key: Key::Char(b as char), mods: 0 })),
                0xc0..=0xdf => {
                    self.utf8_buf[0] = b;
                    self.utf8_len = 1;
                    self.utf8_need = 1;
                }
                0xe0..=0xef => {
                    self.utf8_buf[0] = b;
                    self.utf8_len = 1;
                    self.utf8_need = 2;
                }
                0xf0..=0xf7 => {
                    self.utf8_buf[0] = b;
                    self.utf8_len = 1;
                    self.utf8_need = 3;
                }
                _ => {}
            }
        }

        fn feed_escape(&mut self, b: u8, out: &mut Vec<Event>) {
            self.pending_esc = false;
            match b {
                b'[' => {
                    self.reset_seq();
                    self.state = ParserState::Csi;
                }
                b'O' => self.state = ParserState::Ss3,
                b']' => self.state = ParserState::Osc,
                0x00..=0x7f => {
                    let c = b as char;
                    out.push(Event::Key(KeyEvent { key: Key::Char(c), mods: mods::ALT }));
                    self.state = ParserState::Ground;
                }
                _ => self.state = ParserState::Ground,
            }
        }

        fn feed_ss3(&mut self, b: u8, out: &mut Vec<Event>) {
            self.state = ParserState::Ground;
            let key = match b {
                b'P' => Some(Key::F(1)),
                b'Q' => Some(Key::F(2)),
                b'R' => Some(Key::F(3)),
                b'S' => Some(Key::F(4)),
                _ => None,
            };
            if let Some(key) = key {
                out.push(Event::Key(KeyEvent { key, mods: 0 }));
            }
        }

        fn feed_csi(&mut self, b: u8, out: &mut Vec<Event>) {
            match b {
                b'0'..=b'9' => {
                    self.current = self.current.saturating_mul(10).saturating_add(u16::from(b - b'0'));
                    self.has_current = true;
                }
                b';' => self.push_param(),
                b'<' if self.params.is_empty() && !self.has_current => self.private = Some(b'<'),
                _ => {
                    self.push_param();
                    self.state = ParserState::Ground;
                    self.finish_csi(b, out);
                }
            }
        }

        fn finish_csi(&mut self, final_byte: u8, out: &mut Vec<Event>) {
            if self.private == Some(b'<') {
                self.finish_sgr_mouse(final_byte, out);
                return;
            }
            match final_byte {
                b'A' => self.emit_arrow(Key::Up, out),
                b'B' => self.emit_arrow(Key::Down, out),
                b'C' => self.emit_arrow(Key::Right, out),
                b'D' => self.emit_arrow(Key::Left, out),
                b'H' => self.emit_arrow(Key::Home, out),
                b'F' => self.emit_arrow(Key::End, out),
                b'Z' => out.push(Event::Key(KeyEvent { key: Key::BackTab, mods: 0 })),
                b'I' => out.push(Event::FocusGained),
                b'O' => out.push(Event::FocusLost),
                b'~' => self.finish_tilde(out),
                _ => {}
            }
        }

        fn emit_arrow(&self, key: Key, out: &mut Vec<Event>) {
            let m = modifier_bits(self.param(1, 0));
            out.push(Event::Key(KeyEvent { key, mods: m }));
        }

        fn finish_tilde(&mut self, out: &mut Vec<Event>) {
            let code = self.param(0, 0);
            if code == 200 {
                self.paste_buf.clear();
                self.paste_close = b"\x1b[201~".to_vec();
                self.state = ParserState::Paste;
                return;
            }
            let m = modifier_bits(self.param(1, 0));
            let key = match code {
                1 | 7 => Some(Key::Home),
                2 => Some(Key::Insert),
                3 => Some(Key::Delete),
                4 | 8 => Some(Key::End),
                5 => Some(Key::PageUp),
                6 => Some(Key::PageDown),
                11 => Some(Key::F(1)),
                12 => Some(Key::F(2)),
                13 => Some(Key::F(3)),
                14 => Some(Key::F(4)),
                15 => Some(Key::F(5)),
                17 => Some(Key::F(6)),
                18 => Some(Key::F(7)),
                19 => Some(Key::F(8)),
                20 => Some(Key::F(9)),
                21 => Some(Key::F(10)),
                23 => Some(Key::F(11)),
                24 => Some(Key::F(12)),
                _ => None,
            };
            if let Some(key) = key {
                out.push(Event::Key(KeyEvent { key, mods: m }));
            }
        }

        fn finish_sgr_mouse(&mut self, final_byte: u8, out: &mut Vec<Event>) {
            let b = self.param(0, 0);
            let x = self.param(1, 1).saturating_sub(1);
            let y = self.param(2, 1).saturating_sub(1);
            let pos = Pos { x, y };
            let m = modifier_bits(((b >> 2) & 0x7).saturating_add(1));
            let btn = (b & 0x3) as u8;
            let kind = if b & 0x40 != 0 {
                if btn == 0 { MouseKind::ScrollUp } else { MouseKind::ScrollDown }
            } else if b & 0x20 != 0 {
                MouseKind::Drag(btn)
            } else if final_byte == b'm' {
                MouseKind::Up(btn)
            } else {
                MouseKind::Down(btn)
            };
            out.push(Event::Mouse(MouseEvent { kind, pos, mods: m }));
        }

        fn feed_paste(&mut self, b: u8, out: &mut Vec<Event>) {
            self.paste_buf.push(b as char);
            if self.paste_buf.as_bytes().ends_with(self.paste_close.as_slice()) {
                let end = self.paste_buf.len() - self.paste_close.len();
                let content = self.paste_buf[..end].to_string();
                out.push(Event::Paste(content));
                self.state = ParserState::Ground;
            }
        }
    }
    //#endregion 🔖Parse
}
// #endregion 🔖Ansi

// #region 🔖Event
pub mod event {
    use crate::geometry::{Pos, Size};

    /// ⌨️ A decoded terminal key.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Key {
        Char(char),
        Enter,
        Esc,
        Tab,
        BackTab,
        Backspace,
        Delete,
        Insert,
        Up,
        Down,
        Left,
        Right,
        Home,
        End,
        PageUp,
        PageDown,
        F(u8),
    }

    /// 🎛️ Modifier bitflags.
    pub mod mods {
        pub const SHIFT: u8 = 1;
        pub const ALT: u8 = 2;
        pub const CTRL: u8 = 4;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct KeyEvent {
        pub key: Key,
        pub mods: u8,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MouseKind {
        Down(u8),
        Up(u8),
        Drag(u8),
        Move,
        ScrollUp,
        ScrollDown,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct MouseEvent {
        pub kind: MouseKind,
        pub pos: Pos,
        pub mods: u8,
    }

    /// 📡 Any input the terminal can report to the retained-mode engine.
    #[derive(Clone, Debug, PartialEq)]
    pub enum Event {
        Key(KeyEvent),
        Mouse(MouseEvent),
        Paste(String),
        Resize(Size),
        FocusGained,
        FocusLost,
    }
}
// #endregion 🔖Event

// #region 🔖Scene
pub mod scene {
    use crate::chrome::ChromeState;
    use crate::geometry::{Pos, Rect};
    use crate::layout::Constraint;
    use crate::theme::{Role, Surface};
    use crate::widget::WidgetState;

    const LAYOUT_DIRTY: u8 = 1;
    const PAINT_DIRTY: u8 = 2;

    /// 🪪 A stable, generation-checked handle to a scene node.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NodeId {
        index: u32,
        generation: u32,
    }

    /// 🧱 The payload a node carries.
    pub enum NodeContent {
        Box,
        Text(String),
        Widget(WidgetState),
        Chrome(ChromeState),
    }

    /// 🎨 A node's visual role (independent of its content).
    #[derive(Default, Clone, Copy)]
    pub struct Style {
        pub surface: Option<Surface>,
        pub fg: Option<Role>,
        pub attrs: u8,
    }

    /// 🌳 One retained scene node.
    pub struct Node {
        pub content: NodeContent,
        pub style: Style,
        pub constraint: Constraint,
        pub visible: bool,
        pub(crate) children: Vec<NodeId>,
        pub(crate) parent: Option<NodeId>,
        pub(crate) rect: Rect,
        pub(crate) dirty: u8,
    }

    impl Node {
        pub fn new(content: NodeContent) -> Self {
            Self {
                content,
                style: Style::default(),
                constraint: Constraint::default(),
                visible: true,
                children: Vec::new(),
                parent: None,
                rect: Rect::default(),
                dirty: LAYOUT_DIRTY | PAINT_DIRTY,
            }
        }

        pub fn children(&self) -> &[NodeId] {
            &self.children
        }
    }

    struct Slot {
        node: Option<Node>,
        generation: u32,
    }

    /// 🗂️ A generational-arena retained scene tree, renderer-agnostic.
    pub struct Scene {
        slots: Vec<Slot>,
        free: Vec<u32>,
        root: NodeId,
    }

    impl Scene {
        pub fn new() -> Self {
            let mut slots = Vec::new();
            slots.push(Slot { node: Some(Node::new(NodeContent::Box)), generation: 0 });
            Self { slots, free: Vec::new(), root: NodeId { index: 0, generation: 0 } }
        }

        pub fn root(&self) -> NodeId {
            self.root
        }

        pub fn add(&mut self, parent: NodeId, mut node: Node) -> NodeId {
            node.parent = Some(parent);
            let id = if let Some(index) = self.free.pop() {
                let slot = &mut self.slots[index as usize];
                slot.generation += 1;
                let id = NodeId { index, generation: slot.generation };
                slot.node = Some(node);
                id
            } else {
                let index = self.slots.len() as u32;
                self.slots.push(Slot { node: Some(node), generation: 0 });
                NodeId { index, generation: 0 }
            };
            if let Some(p) = self.slots[parent.index as usize].node.as_mut() {
                p.children.push(id);
            }
            self.mark_dirty(parent, LAYOUT_DIRTY | PAINT_DIRTY);
            id
        }

        pub fn remove(&mut self, id: NodeId) {
            let children = self.node(id).children.clone();
            for child in children {
                self.remove(child);
            }
            if let Some(parent) = self.node(id).parent {
                if let Some(p) = self.slots[parent.index as usize].node.as_mut() {
                    p.children.retain(|c| *c != id);
                }
                self.mark_dirty(parent, LAYOUT_DIRTY | PAINT_DIRTY);
            }
            self.slots[id.index as usize].node = None;
            self.free.push(id.index);
        }

        fn valid(&self, id: NodeId) -> bool {
            self.slots.get(id.index as usize).is_some_and(|s| s.generation == id.generation && s.node.is_some())
        }

        pub fn node(&self, id: NodeId) -> &Node {
            self.slots[id.index as usize].node.as_ref().expect("stale NodeId")
        }

        pub(crate) fn node_raw_mut(&mut self, id: NodeId) -> &mut Node {
            self.slots[id.index as usize].node.as_mut().expect("stale NodeId")
        }

        pub fn rect(&self, id: NodeId) -> Rect {
            self.node(id).rect
        }

        pub(crate) fn mark_dirty(&mut self, id: NodeId, flags: u8) {
            let mut cursor = Some(id);
            while let Some(cur) = cursor {
                if !self.valid(cur) {
                    break;
                }
                let node = self.node_raw_mut(cur);
                if node.dirty & flags == flags {
                    break;
                }
                node.dirty |= flags;
                cursor = node.parent;
            }
        }

        pub(crate) fn take_dirty(&mut self, id: NodeId) -> u8 {
            let d = self.node(id).dirty;
            self.node_raw_mut(id).dirty = 0;
            d
        }

        pub fn node_mut(&mut self, id: NodeId) -> NodeMut<'_> {
            NodeMut { scene: self, id }
        }

        /// 🎯 The deepest visible node whose rect contains `pos`.
        pub fn hit(&self, pos: Pos) -> Option<NodeId> {
            fn walk(scene: &Scene, id: NodeId, pos: Pos) -> Option<NodeId> {
                let node = scene.node(id);
                if !node.visible || !node.rect.contains(pos) {
                    return None;
                }
                for &child in node.children.iter().rev() {
                    if let Some(hit) = walk(scene, child, pos) {
                        return Some(hit);
                    }
                }
                Some(id)
            }
            walk(self, self.root, pos)
        }
    }

    impl Default for Scene {
        fn default() -> Self {
            Self::new()
        }
    }

    /// ✍️ A scoped mutation handle: every setter marks layout/paint dirty up the parent chain.
    pub struct NodeMut<'a> {
        scene: &'a mut Scene,
        id: NodeId,
    }

    impl<'a> NodeMut<'a> {
        pub fn set_text(&mut self, text: impl Into<String>) {
            self.scene.node_raw_mut(self.id).content = NodeContent::Text(text.into());
            self.scene.mark_dirty(self.id, LAYOUT_DIRTY | PAINT_DIRTY);
        }

        pub fn set_constraint(&mut self, constraint: Constraint) {
            self.scene.node_raw_mut(self.id).constraint = constraint;
            self.scene.mark_dirty(self.id, LAYOUT_DIRTY);
        }

        pub fn set_style(&mut self, style: Style) {
            self.scene.node_raw_mut(self.id).style = style;
            self.scene.mark_dirty(self.id, PAINT_DIRTY);
        }

        pub fn set_visible(&mut self, visible: bool) {
            self.scene.node_raw_mut(self.id).visible = visible;
            self.scene.mark_dirty(self.id, LAYOUT_DIRTY | PAINT_DIRTY);
        }

        pub fn widget(&mut self) -> Option<&mut WidgetState> {
            self.scene.mark_dirty(self.id, PAINT_DIRTY);
            match &mut self.scene.node_raw_mut(self.id).content {
                NodeContent::Widget(w) => Some(w),
                _ => None,
            }
        }

        pub fn chrome(&mut self) -> Option<&mut ChromeState> {
            self.scene.mark_dirty(self.id, PAINT_DIRTY);
            match &mut self.scene.node_raw_mut(self.id).content {
                NodeContent::Chrome(c) => Some(c),
                _ => None,
            }
        }

        pub fn id(&self) -> NodeId {
            self.id
        }
    }
}
// #endregion 🔖Scene

// #region 🔖Layout
pub mod layout {
    use crate::geometry::Rect;
    use crate::scene::{NodeContent, NodeId, Scene};

    /// 📐 How one axis of a node's size is determined.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Dimension {
        Auto,
        Cells(u16),
        Weight(u16),
    }

    impl Default for Dimension {
        fn default() -> Self {
            Dimension::Auto
        }
    }

    /// ↔️ How a node arranges its children.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub enum Direction {
        #[default]
        Row,
        Column,
        Stack,
    }

    /// 📦 A node's layout intent.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Constraint {
        pub direction: Direction,
        pub width: Dimension,
        pub height: Dimension,
        pub gap: u16,
        pub padding: [u16; 4],
    }

    fn measure(scene: &Scene, id: NodeId) -> (u16, u16) {
        match &scene.node(id).content {
            NodeContent::Text(s) => (crate::text::display_width(s), 1),
            NodeContent::Widget(w) => {
                let size = w.preferred_size();
                (size.width, size.height)
            }
            _ => (0, 0),
        }
    }

    fn distribute(dims: &[Dimension], measured: &[u16], total: u16, gap: u16) -> Vec<u16> {
        let n = dims.len();
        if n == 0 {
            return Vec::new();
        }
        let gaps = gap.saturating_mul(n.saturating_sub(1) as u16);
        let mut sizes = vec![0u16; n];
        let mut weight_total = 0u32;
        let mut fixed_total = gaps;
        for (i, d) in dims.iter().enumerate() {
            match d {
                Dimension::Cells(c) => {
                    sizes[i] = *c;
                    fixed_total += c;
                }
                Dimension::Auto => {
                    sizes[i] = measured[i];
                    fixed_total += measured[i];
                }
                Dimension::Weight(w) => weight_total += u32::from(*w),
            }
        }
        let remaining = u32::from(total).saturating_sub(u32::from(fixed_total));
        if weight_total > 0 {
            let mut remainders: Vec<(usize, u32)> = Vec::new();
            let mut used = 0u32;
            for (i, d) in dims.iter().enumerate() {
                if let Dimension::Weight(w) = d {
                    let share = remaining * u32::from(*w) / weight_total;
                    sizes[i] = share as u16;
                    used += share;
                    remainders.push((i, remaining * u32::from(*w) % weight_total));
                }
            }
            let mut leftover = remaining.saturating_sub(used);
            remainders.sort_by(|a, b| b.1.cmp(&a.1));
            for (i, _) in remainders {
                if leftover == 0 {
                    break;
                }
                sizes[i] += 1;
                leftover -= 1;
            }
        }
        sizes
    }

    /// 🧮 Recomputes rects for the whole tree from `viewport` down (no-op-safe to call every frame).
    pub fn solve(scene: &mut Scene, viewport: Rect) {
        layout_node(scene, scene.root(), viewport);
    }

    fn layout_node(scene: &mut Scene, id: NodeId, rect: Rect) {
        scene.node_raw_mut(id).rect = rect;
        let constraint = scene.node(id).constraint;
        let [top, right, bottom, left] = constraint.padding;
        let inner = rect.inset_sides(top, right, bottom, left);
        let children: Vec<NodeId> = scene.node(id).children().to_vec();
        if children.is_empty() {
            return;
        }
        match constraint.direction {
            Direction::Stack => {
                for &child in &children {
                    layout_node(scene, child, inner);
                }
            }
            Direction::Row => {
                let dims: Vec<Dimension> = children.iter().map(|c| scene.node(*c).constraint.width).collect();
                let measured: Vec<u16> = children.iter().map(|c| measure(scene, *c).0).collect();
                let sizes = distribute(&dims, &measured, inner.width, constraint.gap);
                let mut x = inner.x;
                for (i, &child) in children.iter().enumerate() {
                    let w = sizes[i];
                    layout_node(scene, child, Rect::new(x, inner.y, w, inner.height));
                    x += w + constraint.gap;
                }
            }
            Direction::Column => {
                let dims: Vec<Dimension> = children.iter().map(|c| scene.node(*c).constraint.height).collect();
                let measured: Vec<u16> = children.iter().map(|c| measure(scene, *c).1).collect();
                let sizes = distribute(&dims, &measured, inner.height, constraint.gap);
                let mut y = inner.y;
                for (i, &child) in children.iter().enumerate() {
                    let h = sizes[i];
                    layout_node(scene, child, Rect::new(inner.x, y, inner.width, h));
                    y += h + constraint.gap;
                }
            }
        }
    }

    //#region 🔖WindowLayout
    /// 🪟 One tiled window leaf: which content it hosts and its display title.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WindowLayoutWindowNode {
        pub window_kind_id: String,
        pub title: Option<String>,
    }

    /// 🗂️ A stack of windows sharing one area; only `active_window_kind_id` is visible.
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct WindowLayoutStackNode {
        pub size: Option<f64>,
        pub active_window_kind_id: Option<String>,
        pub children: Vec<WindowLayoutWindowNode>,
    }

    /// ↔️ A row or column of tiled children.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WindowLayoutAxisNode {
        pub kind: String,
        pub size: Option<f64>,
        pub children: Vec<WindowLayoutChild>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum WindowLayoutChild {
        Axis(WindowLayoutAxisNode),
        Stack(WindowLayoutStackNode),
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum WindowLayoutRoot {
        Axis(WindowLayoutAxisNode),
        Stack(WindowLayoutStackNode),
    }

    /// 🌳 A full tiling window arrangement (rows/columns/stacks with weights).
    #[derive(Clone, Debug, PartialEq)]
    pub struct WindowLayout {
        pub root: WindowLayoutRoot,
    }

    /// 📐 The resolved on-screen placement of one visible window.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WindowMeasure {
        pub window_kind_id: String,
        pub rect: Rect,
        pub active: bool,
        pub stack_tabs: Vec<String>,
    }

    fn axis_child_size(child: &WindowLayoutChild) -> f64 {
        match child {
            WindowLayoutChild::Axis(a) => a.size.unwrap_or(1.0),
            WindowLayoutChild::Stack(s) => s.size.unwrap_or(1.0),
        }
    }

    fn solve_axis(node: &WindowLayoutAxisNode, area: Rect, out: &mut Vec<WindowMeasure>) {
        let is_row = node.kind == "row";
        let total_weight: f64 = node.children.iter().map(axis_child_size).sum::<f64>().max(1e-6);
        let extent = if is_row { area.width } else { area.height };
        let mut offset = 0u16;
        for child in &node.children {
            let weight = axis_child_size(child);
            let size = ((f64::from(extent) * weight / total_weight).round() as u16).min(extent - offset);
            let child_rect = if is_row {
                Rect::new(area.x + offset, area.y, size, area.height)
            } else {
                Rect::new(area.x, area.y + offset, area.width, size)
            };
            match child {
                WindowLayoutChild::Axis(a) => solve_axis(a, child_rect, out),
                WindowLayoutChild::Stack(s) => solve_stack(s, child_rect, out),
            }
            offset += size;
        }
    }

    fn solve_stack(node: &WindowLayoutStackNode, area: Rect, out: &mut Vec<WindowMeasure>) {
        if node.children.is_empty() {
            return;
        }
        let tabs: Vec<String> = node.children.iter().map(|c| c.window_kind_id.clone()).collect();
        let active = node
            .active_window_kind_id
            .clone()
            .unwrap_or_else(|| node.children[0].window_kind_id.clone());
        out.push(WindowMeasure { window_kind_id: active, rect: area, active: true, stack_tabs: tabs });
    }

    /// 🧮 Resolves a `WindowLayout` into concrete on-screen `WindowMeasure`s.
    pub fn solve_window_layout(layout: &WindowLayout, area: Rect) -> Vec<WindowMeasure> {
        let mut out = Vec::new();
        match &layout.root {
            WindowLayoutRoot::Axis(a) => solve_axis(a, area, &mut out),
            WindowLayoutRoot::Stack(s) => solve_stack(s, area, &mut out),
        }
        out
    }

    /// 🏗️ Builds a row/column layout of individually-sized windows.
    pub fn create_default_layout(
        window_ids: &[String],
        direction: &str,
        sizes: Option<&[f64]>,
        titles: Option<&[String]>,
    ) -> WindowLayout {
        let children = window_ids
            .iter()
            .enumerate()
            .map(|(i, id)| {
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    size: sizes.and_then(|s| s.get(i)).copied(),
                    active_window_kind_id: Some(id.clone()),
                    children: vec![WindowLayoutWindowNode {
                        window_kind_id: id.clone(),
                        title: titles.and_then(|t| t.get(i)).cloned(),
                    }],
                })
            })
            .collect();
        WindowLayout {
            root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
                kind: direction.to_string(),
                size: None,
                children,
            }),
        }
    }

    /// 🏗️ Builds an evenly-weighted row layout.
    pub fn even_window_layout(window_ids: &[String]) -> WindowLayout {
        create_default_layout(window_ids, "row", None, None)
    }
    //#endregion 🔖WindowLayout
}
// #endregion 🔖Layout

// #region 🔖Widget
pub mod widget {
    use crate::cell::{attr, Cell, CellBuffer};
    use crate::event::{Key, KeyEvent};
    use crate::geometry::{Pos, Rect, Size};
    use crate::text::{display_width, truncate_to};
    use crate::theme::{Role, Surface, Theme};
    use std::collections::VecDeque;

    /// 📣 A widget- or window-chrome-level result of handling input, surfaced to the app.
    #[derive(Clone, Debug, PartialEq)]
    pub enum WidgetSignal {
        Activated(usize),
        SelectionChanged(usize),
        ValueChanged(String),
        Toggled(bool),
        TabChanged(usize),
        WindowClose,
        WindowMaximize,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Align {
        Left,
        Center,
        Right,
    }

    /// 🏷️ Static or dynamic single-line text.
    pub struct LabelState {
        pub text: String,
        pub align: Align,
        pub role: Role,
    }

    /// 📃 A scrollable, selectable, optionally multi-marked list.
    pub struct ListState {
        pub items: Vec<String>,
        pub selected: usize,
        pub offset: usize,
        pub marks: Vec<bool>,
    }

    impl ListState {
        pub fn new(items: Vec<String>) -> Self {
            let marks = vec![false; items.len()];
            Self { items, selected: 0, offset: 0, marks }
        }
    }

    /// 🔁 A cycler for an `All | Individual(value)` style option pick.
    pub struct SelectState {
        pub label: String,
        pub options: Vec<String>,
        pub index: usize,
    }

    pub struct TabsState {
        pub tabs: Vec<String>,
        pub active: usize,
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum LogScroll {
        Follow,
        At(usize),
    }

    /// 📜 A bounded scrollback log view.
    pub struct LogState {
        lines: VecDeque<String>,
        pub capacity: usize,
        pub scroll: LogScroll,
    }

    impl LogState {
        pub fn new(capacity: usize) -> Self {
            Self { lines: VecDeque::with_capacity(capacity), capacity, scroll: LogScroll::Follow }
        }

        pub fn push(&mut self, line: &str) {
            if self.lines.len() >= self.capacity {
                self.lines.pop_front();
            }
            self.lines.push_back(line.to_string());
        }

        pub fn clear(&mut self) {
            self.lines.clear();
            self.scroll = LogScroll::Follow;
        }

        pub fn lines(&self) -> &VecDeque<String> {
            &self.lines
        }
    }

    pub struct InputState {
        pub value: String,
        pub cursor: usize,
        pub placeholder: String,
    }

    #[derive(Default)]
    pub struct DividerState {
        pub label: Option<String>,
    }

    pub struct ChipState {
        pub label: String,
        pub on: bool,
    }

    //#region 🔖Table
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TableAlign {
        Left,
        Right,
    }

    /// 📐 One table column; `width == 0` means "flex" — split the remaining space evenly.
    pub struct TableColumn {
        pub label: String,
        pub width: u16,
        pub align: TableAlign,
    }

    impl TableColumn {
        pub fn new(label: impl Into<String>, width: u16, align: TableAlign) -> Self {
            Self { label: label.into(), width, align }
        }
    }

    /// 🌳 One row, flat in display order; `level` and `has_children` express the tree — a row is
    /// hidden whenever a preceding, still-nesting ancestor has `expanded == false`.
    pub struct TableRow {
        pub id: String,
        pub cells: Vec<String>,
        pub level: u16,
        pub has_children: bool,
        pub expanded: bool,
    }

    impl TableRow {
        pub fn parent(id: impl Into<String>, cells: Vec<String>) -> Self {
            Self { id: id.into(), cells, level: 0, has_children: true, expanded: true }
        }

        pub fn child(id: impl Into<String>, cells: Vec<String>, level: u16) -> Self {
            Self { id: id.into(), cells, level, has_children: false, expanded: true }
        }
    }

    /// 🗂️ A semio-styled table: bold muted header with a hairline underline, hairline row
    /// separators, no vertical rules, no striping — mirrors `ui/js/react`'s `Table` and
    /// `print/tex/semio-table.sty`. Tree rows are plain indented rows in the same table.
    pub struct TableState {
        pub columns: Vec<TableColumn>,
        pub rows: Vec<TableRow>,
        pub selected: usize,
    }

    impl TableState {
        pub fn new(columns: Vec<TableColumn>, rows: Vec<TableRow>) -> Self {
            Self { columns, rows, selected: 0 }
        }

        /// 👁️ Row indices in display order, skipping any row nested under a collapsed ancestor.
        pub fn visible_indices(&self) -> Vec<usize> {
            let mut out = Vec::new();
            let mut collapsed_from: Option<u16> = None;
            for (i, row) in self.rows.iter().enumerate() {
                if let Some(level) = collapsed_from {
                    if row.level > level {
                        continue;
                    }
                    collapsed_from = None;
                }
                out.push(i);
                if row.has_children && !row.expanded {
                    collapsed_from = Some(row.level);
                }
            }
            out
        }
    }
    //#endregion 🔖Table

    /// 🧩 The concrete state of any core widget.
    pub enum WidgetState {
        Label(LabelState),
        List(ListState),
        Select(SelectState),
        Tabs(TabsState),
        Log(LogState),
        Input(InputState),
        Divider(DividerState),
        Chip(ChipState),
        Table(TableState),
    }

    impl WidgetState {
        pub fn preferred_size(&self) -> Size {
            match self {
                WidgetState::Label(l) => Size { width: display_width(&l.text), height: 1 },
                WidgetState::Select(s) => {
                    let text = format!("{} \u{2039} {} \u{203a}", s.label, s.options.get(s.index).map(String::as_str).unwrap_or(""));
                    Size { width: display_width(&text), height: 1 }
                }
                WidgetState::Chip(c) => Size { width: display_width(&c.label) + 2, height: 1 },
                WidgetState::Divider(_) => Size { width: 1, height: 1 },
                _ => Size { width: 0, height: 0 },
            }
        }

        /// ⌨️ Handles one key press, returning a signal when it changes visible state.
        pub fn on_key(&mut self, ev: &KeyEvent) -> Option<WidgetSignal> {
            match self {
                WidgetState::List(l) => list_on_key(l, ev),
                WidgetState::Select(s) => select_on_key(s, ev),
                WidgetState::Tabs(t) => tabs_on_key(t, ev),
                WidgetState::Input(i) => input_on_key(i, ev),
                WidgetState::Table(t) => table_on_key(t, ev),
                WidgetState::Log(log) => {
                    log_on_key(log, ev);
                    None
                }
                _ => None,
            }
        }

        /// 🖌️ Paints this widget's content into `rect` of `buf`.
        pub fn paint(&self, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
            match self {
                WidgetState::Label(l) => paint_label(l, theme, rect, buf),
                WidgetState::List(l) => paint_list(l, theme, rect, buf, focused),
                WidgetState::Select(s) => paint_select(s, theme, rect, buf, focused),
                WidgetState::Tabs(t) => paint_tabs(t, theme, rect, buf),
                WidgetState::Log(log) => paint_log(log, theme, rect, buf),
                WidgetState::Input(i) => paint_input(i, theme, rect, buf, focused),
                WidgetState::Divider(d) => paint_divider(d, theme, rect, buf),
                WidgetState::Chip(c) => paint_chip(c, theme, rect, buf),
                WidgetState::Table(t) => paint_table(t, theme, rect, buf, focused),
            }
        }
    }

    fn list_on_key(l: &mut ListState, ev: &KeyEvent) -> Option<WidgetSignal> {
        match ev.key {
            Key::Up if l.selected > 0 => {
                l.selected -= 1;
                Some(WidgetSignal::SelectionChanged(l.selected))
            }
            Key::Down if l.selected + 1 < l.items.len() => {
                l.selected += 1;
                Some(WidgetSignal::SelectionChanged(l.selected))
            }
            Key::Char(' ') => {
                if let Some(m) = l.marks.get_mut(l.selected) {
                    *m = !*m;
                    return Some(WidgetSignal::Toggled(*m));
                }
                None
            }
            Key::Enter => Some(WidgetSignal::Activated(l.selected)),
            _ => None,
        }
    }

    fn select_on_key(s: &mut SelectState, ev: &KeyEvent) -> Option<WidgetSignal> {
        if s.options.is_empty() {
            return None;
        }
        match ev.key {
            Key::Left => {
                s.index = (s.index + s.options.len() - 1) % s.options.len();
                Some(WidgetSignal::SelectionChanged(s.index))
            }
            Key::Right | Key::Enter => {
                s.index = (s.index + 1) % s.options.len();
                Some(WidgetSignal::SelectionChanged(s.index))
            }
            _ => None,
        }
    }

    fn tabs_on_key(t: &mut TabsState, ev: &KeyEvent) -> Option<WidgetSignal> {
        if t.tabs.is_empty() {
            return None;
        }
        match ev.key {
            Key::Left => {
                t.active = (t.active + t.tabs.len() - 1) % t.tabs.len();
                Some(WidgetSignal::TabChanged(t.active))
            }
            Key::Right => {
                t.active = (t.active + 1) % t.tabs.len();
                Some(WidgetSignal::TabChanged(t.active))
            }
            _ => None,
        }
    }

    fn input_on_key(i: &mut InputState, ev: &KeyEvent) -> Option<WidgetSignal> {
        match ev.key {
            Key::Char(c) => {
                i.value.insert(i.cursor, c);
                i.cursor += c.len_utf8();
                Some(WidgetSignal::ValueChanged(i.value.clone()))
            }
            Key::Backspace if i.cursor > 0 => {
                let prev = i.value[..i.cursor].chars().next_back().map(char::len_utf8).unwrap_or(1);
                i.cursor -= prev;
                i.value.remove(i.cursor);
                Some(WidgetSignal::ValueChanged(i.value.clone()))
            }
            Key::Left if i.cursor > 0 => {
                i.cursor -= 1;
                None
            }
            Key::Right if i.cursor < i.value.len() => {
                i.cursor += 1;
                None
            }
            _ => None,
        }
    }

    fn log_on_key(log: &mut LogState, ev: &KeyEvent) {
        let len = log.lines.len();
        match ev.key {
            Key::PageUp => log.scroll = LogScroll::At(match log.scroll {
                LogScroll::Follow => len.saturating_sub(1),
                LogScroll::At(n) => n.saturating_sub(10),
            }),
            Key::PageDown => {
                let next = match log.scroll {
                    LogScroll::Follow => return,
                    LogScroll::At(n) => n + 10,
                };
                log.scroll = if next + 1 >= len { LogScroll::Follow } else { LogScroll::At(next) };
            }
            Key::Home => log.scroll = LogScroll::At(0),
            Key::End => log.scroll = LogScroll::Follow,
            _ => {}
        }
    }

    fn table_on_key(t: &mut TableState, ev: &KeyEvent) -> Option<WidgetSignal> {
        let visible = t.visible_indices();
        if visible.is_empty() {
            return None;
        }
        let pos = visible.iter().position(|&i| i == t.selected).unwrap_or(0);
        match ev.key {
            Key::Up if pos > 0 => {
                t.selected = visible[pos - 1];
                Some(WidgetSignal::SelectionChanged(t.selected))
            }
            Key::Down if pos + 1 < visible.len() => {
                t.selected = visible[pos + 1];
                Some(WidgetSignal::SelectionChanged(t.selected))
            }
            Key::Right => {
                let row = &mut t.rows[t.selected];
                if row.has_children && !row.expanded {
                    row.expanded = true;
                    Some(WidgetSignal::SelectionChanged(t.selected))
                } else {
                    None
                }
            }
            Key::Left => {
                let row = &mut t.rows[t.selected];
                if row.has_children && row.expanded {
                    row.expanded = false;
                    Some(WidgetSignal::SelectionChanged(t.selected))
                } else {
                    None
                }
            }
            Key::Enter => {
                let row = &mut t.rows[t.selected];
                if row.has_children {
                    row.expanded = !row.expanded;
                    Some(WidgetSignal::SelectionChanged(t.selected))
                } else {
                    Some(WidgetSignal::Activated(t.selected))
                }
            }
            _ => None,
        }
    }

    fn paint_label(l: &LabelState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        let bg = buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Base));
        let fg = theme.role(l.role);
        let (text, width) = truncate_to(&l.text, rect.width);
        let x = match l.align {
            Align::Left => rect.x,
            Align::Center => rect.x + rect.width.saturating_sub(width) / 2,
            Align::Right => rect.x + rect.width.saturating_sub(width),
        };
        buf.put_str(Pos { x, y: rect.y }, text, fg, bg, 0, rect);
    }

    fn paint_list(l: &ListState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
        let bg = theme.surface(Surface::Window);
        for row in 0..rect.height {
            let idx = l.offset + usize::from(row);
            let Some(item) = l.items.get(idx) else { break };
            let selected = idx == l.selected;
            let row_bg = if selected && focused { theme.role(Role::ActiveBase) } else { bg };
            let row_fg = if selected && focused { theme.role(Role::ActiveForeground) } else { theme.role(Role::Foreground) };
            buf.fill_rect(Rect::new(rect.x, rect.y + row, rect.width, 1), Cell::blank(row_fg, row_bg));
            let mark = l.marks.get(idx).copied().unwrap_or(false);
            let prefix = if mark { "\u{2713} " } else { "  " };
            let text = format!("{prefix}{item}");
            let (text, _) = truncate_to(&text, rect.width);
            buf.put_str(Pos { x: rect.x, y: rect.y + row }, text, row_fg, row_bg, 0, rect);
        }
    }

    fn paint_select(s: &SelectState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
        let fg = if focused { theme.role(Role::Accent) } else { theme.role(Role::Foreground) };
        let bg = buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Panel));
        let value = s.options.get(s.index).map(String::as_str).unwrap_or("");
        let text = format!("{}: \u{2039} {} \u{203a}", s.label, value);
        let (text, _) = truncate_to(&text, rect.width);
        buf.put_str(Pos { x: rect.x, y: rect.y }, text, fg, bg, 0, rect);
    }

    fn paint_tabs(t: &TabsState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        let bg = theme.surface(Surface::Panel);
        buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
        let mut x = rect.x;
        for (i, tab) in t.tabs.iter().enumerate() {
            let active = i == t.active;
            let fg = if active { theme.role(Role::Accent) } else { theme.role(Role::MutedForeground) };
            let label = format!(" {tab} ");
            let w = buf.put_str(Pos { x, y: rect.y }, &label, fg, bg, if active { attr::BOLD } else { 0 }, rect);
            x += w;
        }
    }

    fn paint_log(log: &LogState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        let bg = theme.surface(Surface::Window);
        let fg = theme.role(Role::Foreground);
        buf.fill_rect(rect, Cell::blank(fg, bg));
        let len = log.lines.len();
        let last = match log.scroll {
            LogScroll::Follow => len,
            LogScroll::At(n) => (n + 1).min(len),
        };
        let first = last.saturating_sub(usize::from(rect.height));
        for (row, line) in log.lines.iter().skip(first).take(last - first).enumerate() {
            let (text, _) = truncate_to(line, rect.width);
            buf.put_str(Pos { x: rect.x, y: rect.y + row as u16 }, text, fg, bg, 0, rect);
        }
    }

    fn paint_input(i: &InputState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
        let bg = theme.surface(Surface::Panel);
        buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
        let (text, fg) = if i.value.is_empty() {
            (i.placeholder.as_str(), theme.role(Role::MutedForeground))
        } else {
            (i.value.as_str(), theme.role(Role::Foreground))
        };
        let (text, _) = truncate_to(text, rect.width);
        buf.put_str(Pos { x: rect.x, y: rect.y }, text, fg, bg, 0, rect);
        if focused && rect.width > 0 {
            let cx = (rect.x + display_width(&i.value[..i.cursor.min(i.value.len())])).min(rect.x + rect.width - 1);
            buf.put(cx, rect.y, Cell { ch: '\u{2588}', fg: theme.role(Role::Accent), bg, attrs: 0, width: 1 });
        }
    }

    fn paint_divider(d: &DividerState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        let bg = buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Base));
        let fg = theme.role(Role::BorderNormal);
        buf.hline(Pos { x: rect.x, y: rect.y }, rect.width, '\u{2500}', fg, bg);
        if let Some(label) = &d.label {
            let text = format!(" {label} ");
            let x = rect.x + rect.width.saturating_sub(display_width(&text)) / 2;
            buf.put_str(Pos { x, y: rect.y }, &text, theme.role(Role::Foreground), bg, 0, rect);
        }
    }

    fn paint_chip(c: &ChipState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        let bg = if c.on { theme.role(Role::Accent) } else { theme.surface(Surface::Panel) };
        let fg = if c.on { theme.role(Role::AccentForeground) } else { theme.role(Role::MutedForeground) };
        buf.fill_rect(rect, Cell::blank(fg, bg));
        let text = format!(" {} ", c.label);
        buf.put_str(Pos { x: rect.x, y: rect.y }, &text, fg, bg, 0, rect);
    }

    /// 📐 Resolves each column's width: fixed columns keep their `width`, `width == 0` columns
    /// split whatever space remains evenly.
    fn table_column_widths(columns: &[TableColumn], total_width: u16) -> Vec<u16> {
        let fixed_total: u16 = columns.iter().filter(|c| c.width > 0).map(|c| c.width).sum();
        let gaps = columns.len().saturating_sub(1) as u16;
        let flex_count = columns.iter().filter(|c| c.width == 0).count() as u16;
        let remaining = total_width.saturating_sub(fixed_total + gaps);
        let flex_width = if flex_count > 0 { remaining / flex_count } else { 0 };
        columns.iter().map(|c| if c.width > 0 { c.width } else { flex_width }).collect()
    }

    fn paint_table_cell(buf: &mut CellBuffer, x: u16, y: u16, width: u16, text: &str, fg: [u8; 3], bg: [u8; 3], attrs: u8, align: TableAlign, clip: Rect) {
        let (t, tw) = truncate_to(text, width);
        let cell_x = match align {
            TableAlign::Left => x,
            TableAlign::Right => x + width.saturating_sub(tw),
        };
        buf.put_str(Pos { x: cell_x, y }, t, fg, bg, attrs, clip);
    }

    /// 🖌️ Header (muted, bold) + hairline underline, then hairline-separated body rows; tree rows
    /// indent by level and carry a `▾`/`▸` expand marker — no vertical rules, no striping.
    fn paint_table(t: &TableState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
        if rect.width == 0 || rect.height == 0 || t.columns.is_empty() {
            return;
        }
        let bg = buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Window));
        buf.fill_rect(rect, Cell::blank(theme.role(Role::MutedForeground), bg));

        let widths = table_column_widths(&t.columns, rect.width);
        let mut xs = Vec::with_capacity(widths.len());
        let mut x = rect.x;
        for &w in &widths {
            xs.push(x);
            x += w + 1;
        }

        for ((col, &cx), &w) in t.columns.iter().zip(&xs).zip(&widths) {
            paint_table_cell(buf, cx, rect.y, w, &col.label, theme.role(Role::MutedForeground), bg, attr::BOLD, col.align, rect);
        }
        if rect.height == 1 {
            return;
        }
        buf.hline(Pos { x: rect.x, y: rect.y + 1 }, rect.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
        if rect.height <= 2 {
            return;
        }

        let visible = t.visible_indices();
        if visible.is_empty() {
            let (text, w) = truncate_to("(empty)", rect.width);
            let cx = rect.x + rect.width.saturating_sub(w) / 2;
            buf.put_str(Pos { x: cx, y: rect.y + 2 }, text, theme.role(Role::MutedForeground), bg, 0, rect);
            return;
        }

        let body_height = rect.height - 2;
        let items_fit = usize::from((body_height / 2).max(1));
        let sel_pos = visible.iter().position(|&i| i == t.selected).unwrap_or(0);
        let first = if visible.len() <= items_fit { 0 } else { sel_pos.saturating_sub(items_fit / 2).min(visible.len() - items_fit) };

        let mut y = rect.y + 2;
        let bottom = rect.y + rect.height;
        for &row_idx in visible.iter().skip(first) {
            if y >= bottom {
                break;
            }
            let row = &t.rows[row_idx];
            let selected = row_idx == t.selected;
            let row_bg = if selected && focused { theme.role(Role::ActiveBase) } else { bg };
            let row_fg = if selected && focused { theme.role(Role::ActiveForeground) } else { theme.role(Role::MutedForeground) };
            buf.fill_rect(Rect::new(rect.x, y, rect.width, 1), Cell::blank(row_fg, row_bg));
            for (ci, ((col, &cx), &w)) in t.columns.iter().zip(&xs).zip(&widths).enumerate() {
                let text = if ci == 0 {
                    let indent = "  ".repeat(usize::from(row.level));
                    let marker = if !row.has_children { "  " } else if row.expanded { "\u{25be} " } else { "\u{25b8} " };
                    format!("{indent}{marker}{}", row.cells.first().map(String::as_str).unwrap_or(""))
                } else {
                    row.cells.get(ci).cloned().unwrap_or_default()
                };
                paint_table_cell(buf, cx, y, w, &text, row_fg, row_bg, 0, col.align, rect);
            }
            y += 1;
            if y >= bottom {
                break;
            }
            buf.hline(Pos { x: rect.x, y }, rect.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
            y += 1;
        }
    }
}
// #endregion 🔖Widget

// #region 🔖Chrome
pub mod chrome {
    use crate::cell::{Cell, CellBuffer};
    use crate::geometry::{Pos, Rect};
    use crate::layout::{solve_window_layout, WindowLayout};
    use crate::scene::{Node, NodeContent, NodeId, Scene};
    use crate::text::{display_width, truncate_to};
    use crate::theme::{Role, Surface, Theme};

    #[derive(Clone)]
    pub struct NavItem {
        pub id: String,
        pub label: String,
        pub active: bool,
    }

    pub struct NavbarState {
        pub left: Vec<NavItem>,
        pub center: Vec<NavItem>,
        pub right: Vec<NavItem>,
    }

    #[derive(Clone)]
    pub struct KeyHint {
        pub key: String,
        pub label: String,
    }

    pub struct FooterState {
        pub hints: Vec<KeyHint>,
        pub status: String,
    }

    pub struct WindowState {
        pub title: String,
        pub number: Option<String>,
        pub focused: bool,
        pub closable: bool,
        pub maximizable: bool,
    }

    impl WindowState {
        pub fn new(title: impl Into<String>) -> Self {
            Self { title: title.into(), number: None, focused: false, closable: true, maximizable: true }
        }
    }

    /// 🧩 The concrete state of any semio chrome node.
    pub enum ChromeState {
        Navbar(NavbarState),
        Footer(FooterState),
        Canvas,
        Window(WindowState),
    }

    impl ChromeState {
        /// 🖌️ Paints the chrome background/frame; window/content children paint over it.
        pub fn paint(&self, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
            match self {
                ChromeState::Navbar(n) => paint_navbar(n, theme, rect, buf),
                ChromeState::Footer(f) => paint_footer(f, theme, rect, buf),
                ChromeState::Canvas => {
                    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), theme.surface(Surface::Canvas)));
                }
                ChromeState::Window(w) => paint_window(w, theme, rect, buf),
            }
        }

        /// 🖱️ Resolves a click on a window's close/maximize tab, if any (tab text-row hit only).
        pub fn window_control_at(&self, rect: Rect, pos: Pos) -> Option<crate::widget::WidgetSignal> {
            let ChromeState::Window(w) = self else { return None };
            let layout = window_chip_layout(w, rect);
            if !layout.has_tabs || pos.y != rect.y + 1 {
                return None;
            }
            let controls = layout.controls?;
            let maximize_x = controls.x + 1 + WINDOW_CONTROLS_MAXIMIZE_OFFSET;
            let close_x = controls.x + 1 + WINDOW_CONTROLS_CLOSE_OFFSET;
            if pos.x == close_x && w.closable {
                Some(crate::widget::WidgetSignal::WindowClose)
            } else if pos.x == maximize_x && w.maximizable {
                Some(crate::widget::WidgetSignal::WindowMaximize)
            } else {
                None
            }
        }
    }

    fn paint_items(items: &[NavItem], theme: &Theme, mut x: u16, y: u16, bg: [u8; 3], rect: Rect, buf: &mut CellBuffer) -> u16 {
        for item in items {
            let fg = if item.active { theme.role(Role::Accent) } else { theme.role(Role::Foreground) };
            let label = format!(" {} ", item.label);
            let w = buf.put_str(Pos { x, y }, &label, fg, bg, 0, rect);
            x += w;
        }
        x
    }

    fn paint_navbar(n: &NavbarState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        let bg = theme.surface(Surface::Base);
        let (content, hairline) = rect.split_bottom(1);
        buf.fill_rect(content, Cell::blank(theme.role(Role::Foreground), bg));
        paint_items(&n.left, theme, content.x, content.y, bg, content, buf);
        let center_text: String = n.center.iter().map(|i| i.label.clone()).collect::<Vec<_>>().join(" ");
        let center_x = content.x + content.width.saturating_sub(display_width(&center_text)) / 2;
        buf.put_str(Pos { x: center_x, y: content.y }, &center_text, theme.role(Role::MutedForeground), bg, 0, content);
        let right_width: u16 = n.right.iter().map(|i| display_width(&i.label) + 2).sum();
        let right_x = content.x + content.width.saturating_sub(right_width);
        paint_items(&n.right, theme, right_x, content.y, bg, content, buf);
        buf.hline(Pos { x: hairline.x, y: hairline.y }, hairline.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
    }

    fn paint_footer(f: &FooterState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        let bg = theme.surface(Surface::Base);
        let (hairline, content) = rect.split_top(1);
        buf.hline(Pos { x: hairline.x, y: hairline.y }, hairline.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
        buf.fill_rect(content, Cell::blank(theme.role(Role::Foreground), bg));
        let mut x = content.x;
        for hint in &f.hints {
            let key = format!(" {} ", hint.key);
            x += buf.put_str(Pos { x, y: content.y }, &key, theme.role(Role::Accent), bg, 0, content);
            let label = format!("{} ", hint.label);
            x += buf.put_str(Pos { x, y: content.y }, &label, theme.role(Role::MutedForeground), bg, 0, content);
        }
        let (status, status_w) = truncate_to(&f.status, content.width.saturating_sub(x - content.x));
        let status_x = content.x + content.width.saturating_sub(status_w);
        buf.put_str(Pos { x: status_x, y: content.y }, status, theme.role(Role::MutedForeground), bg, 0, content);
    }

    /// 🔘 The controls tab's interior content: enlarge glyph then close glyph, padded to one cell each.
    const WINDOW_CONTROLS_INTERIOR: &str = " \u{2922} \u{2715} ";
    const WINDOW_CONTROLS_MAXIMIZE_OFFSET: u16 = 1;
    const WINDOW_CONTROLS_CLOSE_OFFSET: u16 = 3;

    /// 🪟 One 2-row tab recessed into a top corner of the window: `x` is its own left-wall column,
    /// `interior` is the padded text between its walls (the tab is `interior_width + 2` cells wide).
    struct WindowTab {
        x: u16,
        interior: String,
        interior_width: u16,
    }

    struct WindowChipLayout {
        has_tabs: bool,
        title: WindowTab,
        controls: Option<WindowTab>,
    }

    /// 📐 Shared by paint and click hit-testing so the two can never drift apart. The title tab's own
    /// left wall is the window's left wall; the controls tab's own right wall is the window's right
    /// wall — both 2 rows tall, each bending down into the main body's top edge one row below.
    fn window_chip_layout(w: &WindowState, rect: Rect) -> WindowChipLayout {
        let controls_interior_width = display_width(WINDOW_CONTROLS_INTERIOR);
        let controls_width = controls_interior_width + 2;
        let number_prefix = w.number.as_ref().map(|n| format!("{n} ")).unwrap_or_default();
        let title_interior_full = format!(" {number_prefix}{} ", w.title);
        let show_controls = w.closable || w.maximizable;
        let title_room = rect.width.saturating_sub(2 + if show_controls { controls_width + 1 } else { 0 });
        let (title_interior, title_interior_width) = truncate_to(&title_interior_full, title_room);
        let title_width = title_interior_width + 2;
        let has_tabs = rect.height >= 4 && title_width >= 3 && rect.width >= title_width + 2;
        let controls_fits = show_controls && rect.width >= title_width + controls_width + 3;
        let controls = controls_fits.then(|| WindowTab {
            x: rect.x + rect.width - controls_width,
            interior: WINDOW_CONTROLS_INTERIOR.to_string(),
            interior_width: controls_interior_width,
        });
        WindowChipLayout { has_tabs, title: WindowTab { x: rect.x, interior: title_interior.to_string(), interior_width: title_interior_width }, controls }
    }

    /// 🪟 Paints one 2-row corner tab: a normal `┌─┐ / │text│` box, then bends its *short* wall (the
    /// one that is not also the window's own permanent side wall) down one row into the main body's
    /// top hairline — `└` when the short wall is on the right (title tab), `┘` when on the left
    /// (controls tab).
    fn paint_corner_tab(buf: &mut CellBuffer, y: u16, tab: &WindowTab, short_wall_is_left: bool, text_fg: [u8; 3], bg: [u8; 3], border: [u8; 3]) {
        let width = tab.interior_width + 2;
        buf.put(tab.x, y, Cell { ch: '\u{250c}', fg: border, bg, attrs: 0, width: 1 });
        buf.hline(Pos { x: tab.x + 1, y }, width.saturating_sub(2), '\u{2500}', border, bg);
        buf.put(tab.x + width - 1, y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(tab.x, y + 1, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
        buf.put_str(Pos { x: tab.x + 1, y: y + 1 }, &tab.interior, text_fg, bg, 0, Rect::new(tab.x + 1, y + 1, tab.interior_width, 1));
        buf.put(tab.x + width - 1, y + 1, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
        if short_wall_is_left {
            buf.put(tab.x, y + 2, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
        } else {
            buf.put(tab.x + width - 1, y + 2, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
        }
    }

    /// 🖌️ Paints a window whose title/control tabs are recessed into its top corners: each is a real
    /// 2-row box sharing the window's own left/right wall, with its short inner wall bending down
    /// into the main body's top edge — the semio-window.sty "flowing" tab look, not text cut into a
    /// flat border line. A side with no tab (controls not wanted, or too narrow to fit) simply stays
    /// flat, its corner sitting at the main body's top row instead of rising two rows like a tab.
    fn paint_window(w: &WindowState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        if rect.width < 2 || rect.height < 2 {
            return;
        }
        let bg = theme.surface(Surface::Window);
        let border = if w.focused { theme.role(Role::BorderEmphasized) } else { theme.role(Role::BorderNormal) };
        let fg = theme.role(Role::Foreground);
        buf.fill_rect(rect, Cell::blank(fg, bg));

        let bottom_y = rect.y + rect.height - 1;
        let right_x = rect.x + rect.width - 1;
        let layout = window_chip_layout(w, rect);

        if !layout.has_tabs {
            buf.hline(Pos { x: rect.x + 1, y: rect.y }, rect.width.saturating_sub(2), '\u{2500}', border, bg);
            buf.hline(Pos { x: rect.x + 1, y: bottom_y }, rect.width.saturating_sub(2), '\u{2500}', border, bg);
            buf.vline(Pos { x: rect.x, y: rect.y + 1 }, bottom_y.saturating_sub(rect.y + 1), '\u{2502}', border, bg);
            buf.vline(Pos { x: right_x, y: rect.y + 1 }, bottom_y.saturating_sub(rect.y + 1), '\u{2502}', border, bg);
            buf.put(rect.x, rect.y, Cell { ch: '\u{250c}', fg: border, bg, attrs: 0, width: 1 });
            buf.put(right_x, rect.y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
            buf.put(rect.x, bottom_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
            buf.put(right_x, bottom_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
            return;
        }

        let top_y = rect.y + 2;

        // bottom edge: always flat, full width
        buf.hline(Pos { x: rect.x + 1, y: bottom_y }, rect.width.saturating_sub(2), '\u{2500}', border, bg);
        buf.put(rect.x, bottom_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(right_x, bottom_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });

        // left side: always a raised title tab
        buf.vline(Pos { x: rect.x, y: rect.y + 1 }, bottom_y.saturating_sub(rect.y + 1), '\u{2502}', border, bg);
        paint_corner_tab(buf, rect.y, &layout.title, false, theme.role(Role::Accent), bg, border);

        // right side: a raised controls tab, or — when absent — a plain flat corner at the body's top
        let body_right_x = match &layout.controls {
            Some(controls) => {
                buf.vline(Pos { x: right_x, y: rect.y + 1 }, bottom_y.saturating_sub(rect.y + 1), '\u{2502}', border, bg);
                paint_corner_tab(buf, rect.y, controls, true, theme.role(Role::MutedForeground), bg, border);
                controls.x
            }
            None => {
                buf.vline(Pos { x: right_x, y: top_y + 1 }, bottom_y.saturating_sub(top_y + 1), '\u{2502}', border, bg);
                buf.put(right_x, top_y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
                right_x
            }
        };

        // main body's top edge: from the title tab's bend to the right side's bend/corner
        let body_left_x = layout.title.x + layout.title.interior_width + 2;
        if body_right_x > body_left_x {
            buf.hline(Pos { x: body_left_x, y: top_y }, body_right_x - body_left_x, '\u{2500}', border, bg);
        }
    }

    /// 🏛️ The three fixed shell regions plus one Window node per resolved `WindowMeasure`.
    pub struct Shell {
        pub navbar: NodeId,
        pub canvas: NodeId,
        pub footer: NodeId,
        pub windows: Vec<(String, NodeId)>,
    }

    /// 🏗️ Builds navbar(top) + canvas(fill) + footer(bottom), then one Window per tiled slot.
    pub fn shell(scene: &mut Scene, navbar: NavbarState, footer: FooterState, layout: &WindowLayout) -> Shell {
        let root = scene.root();
        let navbar_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Navbar(navbar))));
        let canvas_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Canvas)));
        let footer_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Footer(footer))));
        {
            let mut root_mut = scene.node_mut(root);
            root_mut.set_constraint(crate::layout::Constraint {
                direction: crate::layout::Direction::Column,
                ..Default::default()
            });
        }
        scene.node_mut(navbar_id).set_constraint(crate::layout::Constraint {
            height: crate::layout::Dimension::Cells(2),
            ..Default::default()
        });
        scene.node_mut(canvas_id).set_constraint(crate::layout::Constraint {
            height: crate::layout::Dimension::Weight(1),
            direction: crate::layout::Direction::Stack,
            ..Default::default()
        });
        scene.node_mut(footer_id).set_constraint(crate::layout::Constraint {
            height: crate::layout::Dimension::Cells(2),
            ..Default::default()
        });
        let mut windows = Vec::new();
        for measure in solve_window_layout(layout, Rect::default()) {
            let id = scene.add(
                canvas_id,
                Node::new(NodeContent::Chrome(ChromeState::Window(WindowState::new(measure.window_kind_id.clone())))),
            );
            windows.push((measure.window_kind_id, id));
        }
        Shell { navbar: navbar_id, canvas: canvas_id, footer: footer_id, windows }
    }

    impl Shell {
        /// 🔁 Re-measures `layout` against the canvas rect and repositions each window node.
        pub fn apply_layout(&self, scene: &mut Scene, layout: &WindowLayout) {
            let area = scene.rect(self.canvas);
            let measures = solve_window_layout(layout, area);
            for (kind_id, id) in &self.windows {
                if let Some(m) = measures.iter().find(|m| &m.window_kind_id == kind_id) {
                    scene.node_mut(*id).set_constraint(crate::layout::Constraint {
                        width: crate::layout::Dimension::Cells(m.rect.width),
                        height: crate::layout::Dimension::Cells(m.rect.height),
                        ..Default::default()
                    });
                }
            }
        }
    }
}
// #endregion 🔖Chrome

// #region 🔖Engine
pub mod engine {
    use crate::ansi::{emit_runs, AnsiPatch};
    use crate::cell::{diff, Cell, CellBuffer};
    use crate::event::{Event, Key, KeyEvent};
    use crate::geometry::Size;
    use crate::scene::{NodeContent, NodeId, Scene};
    use crate::theme::Theme;
    use crate::widget::WidgetSignal;
    use ui_styling::appearance::AppearanceName;

    fn focusable(scene: &Scene, id: NodeId) -> bool {
        matches!(scene.node(id).content, NodeContent::Widget(_))
    }

    fn dfs_focusables(scene: &Scene, id: NodeId, out: &mut Vec<NodeId>) {
        if focusable(scene, id) {
            out.push(id);
        }
        for &child in scene.node(id).children() {
            dfs_focusables(scene, child, out);
        }
    }

    /// 🖥️ The retained-mode pipeline: layout-if-dirty → paint-if-dirty → damage-diff → ANSI.
    pub struct Tui {
        pub scene: Scene,
        pub theme: Theme,
        size: Size,
        front: CellBuffer,
        back: CellBuffer,
        focus: Option<NodeId>,
        full_redraw: bool,
    }

    impl Tui {
        pub fn new(size: Size, theme: Theme) -> Self {
            let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
            Self {
                scene: Scene::new(),
                theme,
                size,
                front: CellBuffer::new(size, blank),
                back: CellBuffer::new(size, blank),
                focus: None,
                full_redraw: true,
            }
        }

        /// 🔍 The last fully-composed frame, for hosts/tests that need to inspect the actual render.
        pub fn frame(&self) -> &CellBuffer {
            &self.front
        }

        pub fn resize(&mut self, size: Size) {
            self.size = size;
            let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
            self.front.resize(size, blank);
            self.back.resize(size, blank);
            self.full_redraw = true;
        }

        pub fn set_appearance(&mut self, appearance: AppearanceName) {
            self.theme.set_appearance(appearance);
            self.full_redraw = true;
        }

        pub fn focus(&self) -> Option<NodeId> {
            self.focus
        }

        pub fn set_focus(&mut self, id: Option<NodeId>) {
            self.focus = id;
        }

        pub fn focus_next(&mut self) {
            let mut list = Vec::new();
            dfs_focusables(&self.scene, self.scene.root(), &mut list);
            if list.is_empty() {
                return;
            }
            let next = match self.focus.and_then(|f| list.iter().position(|x| *x == f)) {
                Some(i) => (i + 1) % list.len(),
                None => 0,
            };
            self.focus = Some(list[next]);
        }

        pub fn focus_prev(&mut self) {
            let mut list = Vec::new();
            dfs_focusables(&self.scene, self.scene.root(), &mut list);
            if list.is_empty() {
                return;
            }
            let prev = match self.focus.and_then(|f| list.iter().position(|x| *x == f)) {
                Some(i) => (i + list.len() - 1) % list.len(),
                None => list.len() - 1,
            };
            self.focus = Some(list[prev]);
        }

        /// 📡 Routes one input event to the focused widget (keys) or the hit node (mouse).
        pub fn dispatch(&mut self, event: &Event) -> Vec<(NodeId, WidgetSignal)> {
            let mut signals = Vec::new();
            match event {
                Event::Resize(size) => self.resize(*size),
                Event::Key(KeyEvent { key: Key::Tab, .. }) => self.focus_next(),
                Event::Key(KeyEvent { key: Key::BackTab, .. }) => self.focus_prev(),
                Event::Key(key_ev) => {
                    if let Some(id) = self.focus {
                        if let Some(widget) = self.scene.node_mut(id).widget() {
                            if let Some(signal) = widget.on_key(key_ev) {
                                signals.push((id, signal));
                            }
                        }
                    }
                }
                Event::Mouse(m) => {
                    if let Some(id) = self.scene.hit(m.pos) {
                        self.focus = Some(id);
                        if matches!(m.kind, crate::event::MouseKind::Down(_)) {
                            let rect = self.scene.node(id).rect;
                            if let NodeContent::Chrome(chrome) = &self.scene.node(id).content {
                                if let Some(signal) = chrome.window_control_at(rect, m.pos) {
                                    signals.push((id, signal));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            signals
        }

        fn paint(&mut self) {
            fn walk(scene: &Scene, theme: &Theme, focus: Option<NodeId>, id: NodeId, buf: &mut CellBuffer) {
                let node = scene.node(id);
                if !node.visible {
                    return;
                }
                let rect = node.rect;
                match &node.content {
                    NodeContent::Chrome(c) => c.paint(theme, rect, buf),
                    NodeContent::Widget(w) => w.paint(theme, rect, buf, Some(id) == focus),
                    NodeContent::Text(s) => {
                        let bg = buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(crate::theme::Surface::Base));
                        buf.put_str(crate::geometry::Pos { x: rect.x, y: rect.y }, s, theme.role(crate::theme::Role::Foreground), bg, 0, rect);
                    }
                    NodeContent::Box => {}
                }
                for &child in node.children() {
                    walk(scene, theme, focus, child, buf);
                }
            }
            walk(&self.scene, &self.theme, self.focus, self.scene.root(), &mut self.back);
        }

        /// 🎬 Solves layout if dirty, repaints, diffs against the last frame, and emits a patch.
        pub fn render(&mut self) -> AnsiPatch {
            let root_dirty = self.scene.take_dirty(self.scene.root()) != 0;
            if !root_dirty && !self.full_redraw {
                return AnsiPatch::default();
            }
            crate::layout::solve(&mut self.scene, crate::geometry::Rect::new(0, 0, self.size.width, self.size.height));
            self.paint();
            let mut patch = AnsiPatch::default();
            if self.full_redraw {
                let full = vec![crate::cell::DiffRun { y: 0, x: 0, len: self.size.width }; usize::from(self.size.height)]
                    .into_iter()
                    .enumerate()
                    .map(|(y, mut r)| {
                        r.y = y as u16;
                        r
                    })
                    .collect::<Vec<_>>();
                emit_runs(&self.back, &full, &mut patch);
                self.full_redraw = false;
            } else {
                let runs = diff(&self.front, &self.back);
                emit_runs(&self.back, &runs, &mut patch);
            }
            self.front = self.back.clone();
            patch
        }

        /// 🎬 Forces a full-frame repaint regardless of dirty state.
        pub fn render_full(&mut self) -> AnsiPatch {
            self.full_redraw = true;
            self.render()
        }
    }
}
// #endregion 🔖Engine

// #region 🔖Backend
pub mod backend {
    use crate::ansi::AnsiPatch;
    use crate::event::Event;
    use crate::geometry::Size;
    use std::time::Duration;

    #[derive(Debug)]
    pub struct BackendError {
        pub message: String,
    }

    impl std::fmt::Display for BackendError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    /// 🔌 A platform terminal I/O implementation, kept out of the retained-mode core.
    pub trait TerminalBackend {
        fn size(&mut self) -> Result<Size, BackendError>;
        fn enter(&mut self) -> Result<(), BackendError>;
        fn leave(&mut self) -> Result<(), BackendError>;
        fn present(&mut self, patch: &AnsiPatch) -> Result<(), BackendError>;
        fn poll(&mut self, timeout: Duration) -> Result<Vec<Event>, BackendError>;
    }

    #[cfg(all(feature = "terminal", unix, not(target_arch = "wasm32")))]
    mod native_unix {
        use super::*;
        use crate::ansi::{setup_sequence, teardown_sequence, AnsiParser};
        use std::io::Write;
        use std::os::unix::io::RawFd;

        fn err(message: impl Into<String>) -> BackendError {
            BackendError { message: message.into() }
        }

        /// 🖥️ Raw-mode terminal backend for unix (macOS/Linux), driven by `libc` alone.
        pub struct NativeTerminal {
            fd: RawFd,
            original: libc::termios,
            parser: AnsiParser,
            entered: bool,
        }

        impl NativeTerminal {
            pub fn new() -> Result<Self, BackendError> {
                let fd = libc::STDIN_FILENO;
                let original = unsafe {
                    let mut t: libc::termios = std::mem::zeroed();
                    if libc::tcgetattr(fd, &mut t) != 0 {
                        return Err(err("tcgetattr failed"));
                    }
                    t
                };
                Ok(Self { fd, original, parser: AnsiParser::new(), entered: false })
            }
        }

        impl TerminalBackend for NativeTerminal {
            fn size(&mut self) -> Result<Size, BackendError> {
                unsafe {
                    let mut ws: libc::winsize = std::mem::zeroed();
                    if libc::ioctl(self.fd, libc::TIOCGWINSZ, &mut ws) != 0 {
                        return Err(err("TIOCGWINSZ failed"));
                    }
                    Ok(Size { width: ws.ws_col, height: ws.ws_row })
                }
            }

            fn enter(&mut self) -> Result<(), BackendError> {
                let mut raw = self.original;
                raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG | libc::IEXTEN);
                raw.c_iflag &= !(libc::IXON | libc::ICRNL | libc::BRKINT | libc::INPCK | libc::ISTRIP);
                raw.c_oflag &= !libc::OPOST;
                raw.c_cc[libc::VMIN] = 0;
                raw.c_cc[libc::VTIME] = 0;
                unsafe {
                    if libc::tcsetattr(self.fd, libc::TCSANOW, &raw) != 0 {
                        return Err(err("tcsetattr failed"));
                    }
                }
                self.entered = true;
                std::io::stdout().write_all(setup_sequence().as_bytes()).map_err(|e| err(e.to_string()))?;
                std::io::stdout().flush().map_err(|e| err(e.to_string()))
            }

            fn leave(&mut self) -> Result<(), BackendError> {
                if !self.entered {
                    return Ok(());
                }
                std::io::stdout().write_all(teardown_sequence().as_bytes()).map_err(|e| err(e.to_string()))?;
                std::io::stdout().flush().map_err(|e| err(e.to_string()))?;
                unsafe {
                    libc::tcsetattr(self.fd, libc::TCSANOW, &self.original);
                }
                self.entered = false;
                Ok(())
            }

            fn present(&mut self, patch: &AnsiPatch) -> Result<(), BackendError> {
                std::io::stdout().write_all(patch.0.as_bytes()).map_err(|e| err(e.to_string()))?;
                std::io::stdout().flush().map_err(|e| err(e.to_string()))
            }

            fn poll(&mut self, timeout: std::time::Duration) -> Result<Vec<Event>, BackendError> {
                let mut pfd = libc::pollfd { fd: self.fd, events: libc::POLLIN, revents: 0 };
                let ready = unsafe { libc::poll(&mut pfd, 1, timeout.as_millis() as i32) };
                let mut events = Vec::new();
                if ready > 0 && pfd.revents & libc::POLLIN != 0 {
                    let mut buf = [0u8; 4096];
                    let n = unsafe { libc::read(self.fd, buf.as_mut_ptr() as *mut _, buf.len()) };
                    if n > 0 {
                        self.parser.feed(&buf[..n as usize], &mut events);
                    }
                } else {
                    self.parser.flush_escape(&mut events);
                }
                Ok(events)
            }
        }

        impl Drop for NativeTerminal {
            fn drop(&mut self) {
                let _ = self.leave();
            }
        }
    }
    #[cfg(all(feature = "terminal", unix, not(target_arch = "wasm32")))]
    pub use native_unix::NativeTerminal;

    #[cfg(all(feature = "terminal", windows))]
    mod native_windows {
        use super::*;
        use crate::ansi::{setup_sequence, teardown_sequence, AnsiParser};
        use windows_sys::Win32::Foundation::HANDLE;
        use windows_sys::Win32::Storage::FileSystem::{ReadFile, WriteFile};
        use windows_sys::Win32::System::Console::{
            GetConsoleMode, GetConsoleScreenBufferInfo, GetStdHandle, SetConsoleMode, CONSOLE_SCREEN_BUFFER_INFO,
            DISABLE_NEWLINE_AUTO_RETURN, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT,
            ENABLE_VIRTUAL_TERMINAL_INPUT, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
        };
        use windows_sys::Win32::System::Threading::WaitForSingleObject;

        fn err(message: impl Into<String>) -> BackendError {
            BackendError { message: message.into() }
        }

        /// 🖥️ VT-mode terminal backend for Windows consoles, driven by `windows-sys` alone.
        pub struct NativeTerminal {
            stdin: HANDLE,
            stdout: HANDLE,
            original_in: u32,
            original_out: u32,
            parser: AnsiParser,
            entered: bool,
        }

        impl NativeTerminal {
            pub fn new() -> Result<Self, BackendError> {
                unsafe {
                    let stdin = GetStdHandle(STD_INPUT_HANDLE);
                    let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
                    let mut original_in = 0u32;
                    let mut original_out = 0u32;
                    if GetConsoleMode(stdin, &mut original_in) == 0 || GetConsoleMode(stdout, &mut original_out) == 0 {
                        return Err(err("GetConsoleMode failed"));
                    }
                    Ok(Self { stdin, stdout, original_in, original_out, parser: AnsiParser::new(), entered: false })
                }
            }
        }

        impl TerminalBackend for NativeTerminal {
            fn size(&mut self) -> Result<Size, BackendError> {
                unsafe {
                    let mut info: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
                    if GetConsoleScreenBufferInfo(self.stdout, &mut info) == 0 {
                        return Err(err("GetConsoleScreenBufferInfo failed"));
                    }
                    let width = (info.srWindow.Right - info.srWindow.Left + 1).max(0) as u16;
                    let height = (info.srWindow.Bottom - info.srWindow.Top + 1).max(0) as u16;
                    Ok(Size { width, height })
                }
            }

            fn enter(&mut self) -> Result<(), BackendError> {
                unsafe {
                    let out_mode = self.original_out | ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN;
                    let in_mode = (self.original_in | ENABLE_VIRTUAL_TERMINAL_INPUT)
                        & !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT);
                    if SetConsoleMode(self.stdout, out_mode) == 0 || SetConsoleMode(self.stdin, in_mode) == 0 {
                        return Err(err("SetConsoleMode failed"));
                    }
                }
                self.entered = true;
                self.write_raw(setup_sequence().as_bytes())
            }

            fn leave(&mut self) -> Result<(), BackendError> {
                if !self.entered {
                    return Ok(());
                }
                self.write_raw(teardown_sequence().as_bytes())?;
                unsafe {
                    SetConsoleMode(self.stdin, self.original_in);
                    SetConsoleMode(self.stdout, self.original_out);
                }
                self.entered = false;
                Ok(())
            }

            fn present(&mut self, patch: &AnsiPatch) -> Result<(), BackendError> {
                self.write_raw(patch.0.as_bytes())
            }

            fn poll(&mut self, timeout: std::time::Duration) -> Result<Vec<Event>, BackendError> {
                let mut events = Vec::new();
                let wait = unsafe { WaitForSingleObject(self.stdin, timeout.as_millis() as u32) };
                if wait == 0 {
                    let mut buf = [0u8; 4096];
                    let mut read = 0u32;
                    unsafe {
                        if ReadFile(self.stdin, buf.as_mut_ptr(), buf.len() as u32, &mut read, std::ptr::null_mut()) != 0 && read > 0 {
                            self.parser.feed(&buf[..read as usize], &mut events);
                        }
                    }
                } else {
                    self.parser.flush_escape(&mut events);
                }
                Ok(events)
            }
        }

        impl NativeTerminal {
            fn write_raw(&self, bytes: &[u8]) -> Result<(), BackendError> {
                let mut written = 0u32;
                unsafe {
                    if WriteFile(self.stdout, bytes.as_ptr(), bytes.len() as u32, &mut written, std::ptr::null_mut()) == 0 {
                        return Err(err("WriteFile failed"));
                    }
                }
                Ok(())
            }
        }

        impl Drop for NativeTerminal {
            fn drop(&mut self) {
                let _ = self.leave();
            }
        }
    }
    #[cfg(all(feature = "terminal", windows))]
    pub use native_windows::NativeTerminal;
}
// #endregion 🔖Backend

// #region 🔖WasmHost
pub mod host {
    use crate::ansi::{setup_sequence, teardown_sequence, AnsiParser};
    use crate::engine::Tui;
    use crate::event::Event;
    use crate::geometry::Size;
    use crate::theme::Theme;
    use ui_styling::appearance::AppearanceName;

    /// 🌐 A pure bytes-in/string-out host: feed terminal input, get an ANSI patch back.
    pub struct WasmHost {
        pub tui: Tui,
        parser: AnsiParser,
    }

    impl WasmHost {
        pub fn new(width: u16, height: u16, dark: bool) -> Self {
            let appearance = if dark { AppearanceName::Dark } else { AppearanceName::Light };
            Self { tui: Tui::new(Size { width, height }, Theme::new(appearance)), parser: AnsiParser::new() }
        }

        pub fn feed(&mut self, bytes: &[u8]) -> Vec<Event> {
            let mut events = Vec::new();
            self.parser.feed(bytes, &mut events);
            for event in &events {
                self.tui.dispatch(event);
            }
            events
        }

        pub fn resize(&mut self, width: u16, height: u16) {
            self.tui.dispatch(&Event::Resize(Size { width, height }));
        }

        pub fn render(&mut self) -> String {
            self.tui.render().0
        }

        pub fn setup(&self) -> String {
            setup_sequence().to_string()
        }

        pub fn teardown(&self) -> String {
            teardown_sequence().to_string()
        }
    }

    #[cfg(all(target_arch = "wasm32", feature = "bindgen"))]
    mod bindgen_host {
        use super::WasmHost;
        use wasm_bindgen::prelude::*;

        /// 🌐 The `wasm-bindgen` surface for browser hosts (e.g. an xterm.js terminal).
        #[wasm_bindgen]
        pub struct TuiHost(WasmHost);

        #[wasm_bindgen]
        impl TuiHost {
            #[wasm_bindgen(constructor)]
            pub fn new(width: u16, height: u16, dark: bool) -> TuiHost {
                TuiHost(WasmHost::new(width, height, dark))
            }

            pub fn feed(&mut self, bytes: &[u8]) {
                self.0.feed(bytes);
            }

            pub fn resize(&mut self, width: u16, height: u16) {
                self.0.resize(width, height);
            }

            pub fn render(&mut self) -> String {
                self.0.render()
            }

            pub fn setup(&self) -> String {
                self.0.setup()
            }

            pub fn teardown(&self) -> String {
                self.0.teardown()
            }
        }
    }
    #[cfg(all(target_arch = "wasm32", feature = "bindgen"))]
    pub use bindgen_host::TuiHost;
}
// #endregion 🔖WasmHost

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use crate::ansi::AnsiParser;
    use crate::cell::{attr, diff, Cell, CellBuffer};
    use crate::chrome::{shell, ChromeState, FooterState, NavbarState, WindowState};
    use crate::event::{Event, Key, KeyEvent, MouseEvent, MouseKind};
    use crate::geometry::{Pos, Rect, Size};
    use crate::layout::{
        create_default_layout, even_window_layout, solve, solve_window_layout, Constraint, Dimension, Direction,
        WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode,
    };
    use crate::scene::{Node, NodeContent, Scene};
    use crate::text::{display_width, truncate_to};
    use crate::theme::{Role, Surface, Theme};
    use crate::widget::{TableAlign, TableColumn, TableRow, TableState, WidgetSignal, WidgetState};
    use ui_styling::appearance::AppearanceName;

    fn row_text(buf: &CellBuffer, y: u16) -> String {
        (0..buf.size.width).filter_map(|x| buf.get(x, y)).map(|c| c.ch).filter(|&c| c != '\0').collect()
    }

    #[test]
    fn layout_row_weights_fill_exactly() {
        let mut scene = Scene::new();
        let root = scene.root();
        scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
        let a = scene.add(root, Node::new(NodeContent::Box));
        let b = scene.add(root, Node::new(NodeContent::Box));
        scene.node_mut(a).set_constraint(Constraint { width: Dimension::Weight(1), ..Default::default() });
        scene.node_mut(b).set_constraint(Constraint { width: Dimension::Weight(2), ..Default::default() });
        solve(&mut scene, Rect::new(0, 0, 30, 10));
        assert_eq!(scene.rect(a).width + scene.rect(b).width, 30);
        assert!(scene.rect(b).width > scene.rect(a).width);
    }

    #[test]
    fn layout_auto_measures_text_width() {
        let mut scene = Scene::new();
        let root = scene.root();
        scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
        let text = scene.add(root, Node::new(NodeContent::Text("hi".into())));
        solve(&mut scene, Rect::new(0, 0, 30, 10));
        assert_eq!(scene.rect(text).width, 2);
    }

    #[test]
    fn layout_padding_and_gap() {
        let mut scene = Scene::new();
        let root = scene.root();
        scene.node_mut(root).set_constraint(Constraint {
            direction: Direction::Row,
            gap: 1,
            padding: [1, 1, 1, 1],
            ..Default::default()
        });
        let a = scene.add(root, Node::new(NodeContent::Box));
        let b = scene.add(root, Node::new(NodeContent::Box));
        scene.node_mut(a).set_constraint(Constraint { width: Dimension::Cells(2), ..Default::default() });
        scene.node_mut(b).set_constraint(Constraint { width: Dimension::Cells(2), ..Default::default() });
        solve(&mut scene, Rect::new(0, 0, 20, 10));
        assert_eq!(scene.rect(a).x, 1);
        assert_eq!(scene.rect(b).x, 4);
        assert_eq!(scene.rect(root).height, 10);
        assert_eq!(scene.rect(a).height, 8);
    }

    #[test]
    fn window_layout_row_of_stacks_tiles_without_gaps() {
        let layout = create_default_layout(
            &["a".to_string(), "b".to_string()],
            "row",
            Some(&[1.0, 1.0]),
            None,
        );
        let measures = solve_window_layout(&layout, Rect::new(0, 0, 100, 10));
        assert_eq!(measures.len(), 2);
        assert_eq!(measures[0].rect.width + measures[1].rect.width, 100);
        assert_eq!(measures[0].rect.x, 0);
        assert_eq!(measures[1].rect.x, measures[0].rect.width);
    }

    #[test]
    fn window_layout_stack_exposes_tabs() {
        let layout = WindowLayout {
            root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
                size: None,
                active_window_kind_id: Some("b".into()),
                children: vec![
                    WindowLayoutWindowNode { window_kind_id: "a".into(), title: None },
                    WindowLayoutWindowNode { window_kind_id: "b".into(), title: None },
                ],
            }),
        };
        let measures = solve_window_layout(&layout, Rect::new(0, 0, 40, 10));
        assert_eq!(measures.len(), 1);
        assert_eq!(measures[0].window_kind_id, "b");
        assert_eq!(measures[0].stack_tabs, vec!["a", "b"]);
    }

    #[test]
    fn window_chrome_recesses_tabs_into_the_top_corners_of_a_closed_shape() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 40, 5);
        let mut buf = CellBuffer::new(Size { width: 40, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        let mut w = WindowState::new("Puzzle 3D");
        w.focused = true;
        ChromeState::Window(w).paint(&theme, rect, &mut buf);

        // exact shape: both tabs are real 2-row boxes sharing the window's own left/right wall, each
        // bending its short inner wall down into the main body's top edge one row below
        assert_eq!(row_text(&buf, 0), "┌───────────┐                    ┌─────┐");
        assert_eq!(row_text(&buf, 1), "│ Puzzle 3D │                    │ ⤢ ✕ │");
        assert_eq!(row_text(&buf, 2), "│           └────────────────────┘     │");
        assert_eq!(row_text(&buf, 3), "│                                      │");
        assert_eq!(row_text(&buf, 4), "└──────────────────────────────────────┘");

        let title_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == 'P').expect("title text rendered");
        assert_eq!(buf.get(title_x, 1).unwrap().fg, theme.role(Role::Accent));
    }

    #[test]
    fn window_chrome_flattens_the_right_side_when_no_controls_tab_is_wanted() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 40, 6);
        let mut buf = CellBuffer::new(Size { width: 40, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        let mut w = WindowState::new("Log");
        w.closable = false;
        w.maximizable = false;
        ChromeState::Window(w).paint(&theme, rect, &mut buf);

        // the title tab still rises, but the right side has no tab: its corner sits flat at the
        // main body's top row instead of rising — the window stays one closed shape either way
        assert_eq!(row_text(&buf, 0), "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}                                 ");
        assert_eq!(row_text(&buf, 1), "\u{2502} Log \u{2502}                                 ");
        assert_eq!(row_text(&buf, 2), "\u{2502}     \u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}");
        assert_eq!(row_text(&buf, 3), "\u{2502}                                      \u{2502}");
        assert_eq!(row_text(&buf, 4), "\u{2502}                                      \u{2502}");
        assert_eq!(row_text(&buf, 5), "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}");
        assert!(!row_text(&buf, 0).contains('\u{2922}') && !row_text(&buf, 1).contains('\u{2922}'), "no controls tab was requested");
    }

    #[test]
    fn window_chrome_hides_both_tabs_when_too_narrow_for_even_the_title() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 10, 5);
        let mut buf = CellBuffer::new(Size { width: 10, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        let window = ChromeState::Window(WindowState::new("X"));
        window.paint(&theme, rect, &mut buf);
        // plain flat box: a single top row, no raised tabs at all
        assert_eq!(row_text(&buf, 0), "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}");
        assert_eq!(window.window_control_at(rect, Pos { x: 5, y: 0 }), None);
        assert_eq!(window.window_control_at(rect, Pos { x: 5, y: 1 }), None);
    }

    #[test]
    fn window_control_clicks_resolve_to_close_and_maximize_signals() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 40, 5);
        let mut buf = CellBuffer::new(Size { width: 40, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        let window = ChromeState::Window(WindowState::new("Plugins"));
        window.paint(&theme, rect, &mut buf);
        let maximize_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2922}').expect("maximize glyph rendered");
        let close_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2715}').expect("close glyph rendered");
        assert_eq!(window.window_control_at(rect, Pos { x: maximize_x, y: 1 }), Some(WidgetSignal::WindowMaximize));
        assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 1 }), Some(WidgetSignal::WindowClose));
        assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 0 }), None, "clicks on the tab's own top edge must not trigger a control");
        assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 2 }), None, "clicks below the tab row must not trigger a control");
    }

    #[test]
    fn tui_dispatch_emits_window_close_signal_on_click() {
        let mut tui = crate::engine::Tui::new(Size { width: 40, height: 12 }, Theme::new(AppearanceName::Dark));
        let navbar = NavbarState { left: vec![], center: vec![], right: vec![] };
        let footer = FooterState { hints: vec![], status: String::new() };
        let layout = even_window_layout(&["plugins".to_string()]);
        let built = shell(&mut tui.scene, navbar, footer, &layout);
        tui.render_full();
        let (_, window_id) = built.windows[0].clone();
        let rect = tui.scene.rect(window_id);
        let mut buf = CellBuffer::new(Size { width: rect.width, height: rect.height }, Cell::blank([0, 0, 0], [0, 0, 0]));
        ChromeState::Window(WindowState::new("plugins")).paint(&Theme::new(AppearanceName::Dark), Rect::new(0, 0, rect.width, rect.height), &mut buf);
        let close_x = (0..rect.width).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2715}').expect("close glyph rendered");
        let signals = tui.dispatch(&Event::Mouse(MouseEvent { kind: MouseKind::Down(0), pos: Pos { x: rect.x + close_x, y: rect.y + 1 }, mods: 0 }));
        assert_eq!(signals, vec![(window_id, WidgetSignal::WindowClose)]);
    }

    fn sample_table() -> TableState {
        let columns = vec![
            TableColumn::new("Plugin / App", 0, TableAlign::Left),
            TableColumn::new("React", 6, TableAlign::Right),
        ];
        let rows = vec![
            TableRow::parent("puzzle", vec!["puzzle".into(), "".into()]),
            TableRow::child("puzzle2d", vec!["puzzle2d".into(), "6012".into()], 1),
            TableRow::child("puzzle3d", vec!["puzzle3d".into(), "6013".into()], 1),
            TableRow::parent("draw", vec!["draw".into(), "".into()]),
            TableRow::child("draw", vec!["draw".into(), "6064".into()], 1),
        ];
        TableState::new(columns, rows)
    }

    #[test]
    fn table_header_is_bold_muted_with_a_hairline_underline_and_row_separators() {
        let theme = Theme::new(AppearanceName::Dark);
        let table = sample_table();
        let rect = Rect::new(0, 0, 40, 12);
        let mut buf = CellBuffer::new(Size { width: 40, height: 12 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Table(table).paint(&theme, rect, &mut buf, false);

        assert_eq!(row_text(&buf, 0).trim_end(), "Plugin / App                       React");
        assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::MutedForeground));
        assert_eq!(buf.get(0, 0).unwrap().attrs & attr::BOLD, attr::BOLD, "header must be bold");
        assert_eq!(row_text(&buf, 1), "\u{2500}".repeat(40), "hairline underline missing below the header");
        // no vertical rules anywhere in the header/underline rows
        assert!(!row_text(&buf, 0).contains('\u{2502}'));

        assert_eq!(row_text(&buf, 2).trim_end(), "\u{25be} puzzle");
        assert_eq!(row_text(&buf, 3), "\u{2500}".repeat(40), "row separator missing after the first row");
        assert_eq!(row_text(&buf, 4).trim_end(), "    puzzle2d                        6012");
    }

    #[test]
    fn table_visible_indices_skips_children_of_a_collapsed_parent() {
        let mut table = sample_table();
        assert_eq!(table.visible_indices(), vec![0, 1, 2, 3, 4]);
        table.rows[0].expanded = false;
        assert_eq!(table.visible_indices(), vec![0, 3, 4], "puzzle's children must be hidden while collapsed");
    }

    #[test]
    fn table_on_key_navigates_visible_rows_toggles_and_activates_leaves() {
        let mut table = sample_table();
        table.rows[0].expanded = false;
        table.selected = 0;
        let mut widget = WidgetState::Table(table);
        // Down should skip the hidden puzzle2d/puzzle3d children and land on "draw"
        let down = KeyEvent { key: Key::Down, mods: 0 };
        assert_eq!(widget.on_key(&down), Some(WidgetSignal::SelectionChanged(3)));
        let WidgetState::Table(table) = &mut widget else { unreachable!() };
        assert_eq!(table.selected, 3);
        table.selected = 0;

        let enter = KeyEvent { key: Key::Enter, mods: 0 };
        assert_eq!(widget.on_key(&enter), Some(WidgetSignal::SelectionChanged(0)));
        let WidgetState::Table(table) = &mut widget else { unreachable!() };
        assert!(table.rows[0].expanded, "Enter on a collapsed parent should expand it");
        table.selected = 1;

        assert_eq!(widget.on_key(&enter), Some(WidgetSignal::Activated(1)), "Enter on a leaf should activate it");
    }

    #[test]
    fn table_selected_row_uses_active_base_only_when_focused() {
        let theme = Theme::new(AppearanceName::Dark);
        let mut table = sample_table();
        table.selected = 1;
        let rect = Rect::new(0, 0, 40, 12);

        let mut unfocused = CellBuffer::new(Size { width: 40, height: 12 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Table(TableState::new(std::mem::take(&mut table.columns), std::mem::take(&mut table.rows))).paint(&theme, rect, &mut unfocused, false);
        assert_ne!(unfocused.get(4, 4).unwrap().bg, theme.role(Role::ActiveBase), "unfocused selection must not fill with the accent");

        let mut table2 = sample_table();
        table2.selected = 1;
        let mut focused = CellBuffer::new(Size { width: 40, height: 12 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Table(table2).paint(&theme, rect, &mut focused, true);
        assert_eq!(focused.get(4, 4).unwrap().bg, theme.role(Role::ActiveBase));
        assert_eq!(focused.get(4, 4).unwrap().fg, theme.role(Role::ActiveForeground));
    }

    #[test]
    fn table_flex_column_fills_remaining_width_and_right_aligns_numeric_column() {
        let theme = Theme::new(AppearanceName::Dark);
        let table = sample_table();
        let rect = Rect::new(0, 0, 20, 6);
        let mut buf = CellBuffer::new(Size { width: 20, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Table(table).paint(&theme, rect, &mut buf, false);
        let header = row_text(&buf, 0);
        assert_eq!(header.len(), 20);
        assert!(header.trim_end().ends_with("React"), "the fixed-width numeric column should sit right-aligned at the row's end: {header:?}");
        let _ = theme.surface(Surface::Window);
    }

    #[test]
    fn diff_emits_minimal_runs() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let prev = CellBuffer::new(Size { width: 10, height: 2 }, blank);
        let mut next = prev.clone();
        next.put(3, 0, Cell { ch: 'x', ..blank });
        let runs = diff(&prev, &next);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0], crate::cell::DiffRun { y: 0, x: 3, len: 1 });
    }

    #[test]
    fn diff_merges_nearby_runs() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let prev = CellBuffer::new(Size { width: 20, height: 1 }, blank);
        let mut next = prev.clone();
        next.put(0, 0, Cell { ch: 'a', ..blank });
        next.put(3, 0, Cell { ch: 'b', ..blank });
        let runs = diff(&prev, &next);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].len, 4);
    }

    #[test]
    fn parser_roundtrip_arrow_keys_with_modifiers() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[1;5C", &mut events);
        assert_eq!(events.len(), 1);
        match &events[0] {
            Event::Key(k) => {
                assert_eq!(k.key, Key::Right);
                assert_eq!(k.mods, crate::event::mods::CTRL);
            }
            _ => panic!("expected key event"),
        }
    }

    #[test]
    fn parser_sgr_mouse_click() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[<0;10;5M", &mut events);
        assert_eq!(events.len(), 1);
        match &events[0] {
            Event::Mouse(m) => {
                assert_eq!(m.pos.x, 9);
                assert_eq!(m.pos.y, 4);
            }
            _ => panic!("expected mouse event"),
        }
    }

    #[test]
    fn parser_bracketed_paste() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[200~hello\x1b[201~", &mut events);
        assert_eq!(events, vec![Event::Paste("hello".to_string())]);
    }

    #[test]
    fn parser_lone_esc_flushes_on_timeout() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b", &mut events);
        assert!(events.is_empty());
        parser.flush_escape(&mut events);
        assert_eq!(events, vec![Event::Key(crate::event::KeyEvent { key: Key::Esc, mods: 0 })]);
    }

    #[test]
    fn parser_split_utf8_across_feeds() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        let bytes = "ü".as_bytes();
        parser.feed(&bytes[..1], &mut events);
        assert!(events.is_empty());
        parser.feed(&bytes[1..], &mut events);
        assert_eq!(events, vec![Event::Key(crate::event::KeyEvent { key: Key::Char('ü'), mods: 0 })]);
    }

    #[test]
    fn text_display_width_and_truncate() {
        assert_eq!(display_width("abc"), 3);
        let (s, w) = truncate_to("abcdef", 3);
        assert_eq!(s, "abc");
        assert_eq!(w, 3);
    }

    #[test]
    fn theme_light_and_dark_differ() {
        use crate::theme::Surface;
        let light = Theme::new(AppearanceName::Light);
        let dark = Theme::new(AppearanceName::Dark);
        assert_ne!(light.surface(Surface::Base), dark.surface(Surface::Base));
    }

    #[test]
    fn wasm_host_feed_and_render_smoke() {
        let mut host = crate::host::WasmHost::new(40, 10, true);
        host.feed(b"\r");
        let patch = host.render();
        assert!(!patch.is_empty());
    }
}
// #endregion 🔖Tests
