import { patch } from "./r2-q1a-edit.ts";

const target = (name: string) => `🧱️elements/${name}/🎯️targets/⌨️tui/🦀️.rs`;

patch("⌨️tui/🧪️tests/🔬️text-elements/🦀️.rs", [["\"aé\".replace('é', \"e\\u{301}ü\").to_string()", "\"aé\".replace('é', \"e\\u{301}ü\")"]]);

patch("⌨️tui/📝️text/🦀️.rs", [["self.regional % 2 == 0", "self.regional.is_multiple_of(2)"]]);

patch("⌨️tui/📟️vt/🖥️pane/🦀️.rs", [
  ["Key::Char('i') => return self.resume_passthrough(),", "Key::Char('i') => return Some(self.resume_passthrough()),"],
  ["                return self.resume_passthrough();", "                return Some(self.resume_passthrough());"],
  ["fn resume_passthrough(&mut self) -> Option<WidgetSignal> {\n        self.passthrough = true;\n        self.screen.scroll_view_to_bottom();\n        Some(WidgetSignal::Toggled(true))", "fn resume_passthrough(&mut self) -> WidgetSignal {\n        self.passthrough = true;\n        self.screen.scroll_view_to_bottom();\n        WidgetSignal::Toggled(true)"],
  ["let thumb_start = if span == 0 { 0 } else { (top.min(span) * (track_cells - thumb_len) / span) as u16 };", "let thumb_start = (top.min(span) * (track_cells - thumb_len)).checked_div(span).unwrap_or(0) as u16;"],
  ["self.paint_span(buf, rect, top, hit.row, hit.col, hit.col + hit.cells - 1, |cell|", "self.paint_span(buf, rect, top, hit.row, hit.col..=hit.col + hit.cells - 1, |cell|"],
  ["self.paint_span(buf, rect, top, row, from, to, |cell|", "self.paint_span(buf, rect, top, row, from..=to, |cell|"],
  ["fn paint_span(&self, buf: &mut CellBuffer, rect: Rect, top: u64, row: u64, from: u16, to: u16, restyle: impl Fn(Cell) -> Cell) {", "fn paint_span(&self, buf: &mut CellBuffer, rect: Rect, top: u64, row: u64, columns: std::ops::RangeInclusive<u16>, restyle: impl Fn(Cell) -> Cell) {"],
  ["for col in from..=to.min(rect.width - 1) {", "for col in *columns.start()..=(*columns.end()).min(rect.width - 1) {"],
]);

patch("⌨️tui/📟️vt/🧱️screen/🦀️.rs", [
  ["let mut parser = std::mem::replace(&mut self.parser, VtParser::new());", "let mut parser = std::mem::take(&mut self.parser);"],
  ["row.cells.splice(at..at, std::iter::repeat(blank).take(n));", "row.cells.splice(at..at, std::iter::repeat_n(blank, n));"],
  ["row.cells.extend(std::iter::repeat(blank).take(n));", "row.cells.extend(std::iter::repeat_n(blank, n));"],
  ["let sub = subs.get(i).map(Vec::as_slice).unwrap_or(&[]);", "let sub = subs.get(i).map_or(&[][..], Vec::as_slice);"],
  ["row.cells.get(i).map_or(true, |cell| cell.ch == ' ')", "row.cells.get(i).is_none_or(|cell| cell.ch == ' ')"],
]);

patch("⌨️tui/🔲️cell/🦀️.rs", [
  [
    "    /// 🧩️ Writes one whole grapheme cluster of `cells` width at `(x, y)`.\n    pub fn put_cluster(&mut self, x: u16, y: u16, cluster: &str, cells: u8, fg: Rgb, bg: Rgb, attrs: u8) {",
    "    /// 🧩️ Writes one whole grapheme cluster at `at`; `template` supplies the colours, attributes and cell width, its character is replaced by the cluster's lead scalar.\n    pub fn put_cluster(&mut self, at: Pos, cluster: &str, template: Cell) {",
  ],
  ["self.write(x, y, Cell { ch: lead, fg, bg, attrs, width: cells }, tail);", "self.write(at.x, at.y, Cell { ch: lead, ..template }, tail);"],
  ["self.put_cluster(x, pos.y, cluster, w, fg, bg, attrs);", "self.put_cluster(Pos { x, y: pos.y }, cluster, Cell { ch: ' ', fg, bg, attrs, width: w });"],
]);

patch("⌨️tui/🖥️chrome/🦀️.rs", [
  [
    "    fn style_window(scene: &mut Scene, windows: &[(String, NodeId)], win_id: NodeId, stack: &WindowLayoutStackNode, own: &str, active: &str, peers: usize, zoomed: bool) {",
    "    /// 🧿 Which tab of a stack a window node shows, how many stacks share the canvas and whether it is zoomed.\n    struct StackSlot<'a> {\n        own: &'a str,\n        active: &'a str,\n        peers: usize,\n        zoomed: bool,\n    }\n\n    fn style_window(scene: &mut Scene, windows: &[(String, NodeId)], win_id: NodeId, stack: &WindowLayoutStackNode, slot: &StackSlot) {\n        let StackSlot { own, active, peers, zoomed } = *slot;",
  ],
  ["let active = stack.active_window_kind_id.as_deref().unwrap_or_else(|| stack.children.first().map(|c| c.window_kind_id.as_str()).unwrap_or(\"\"));", "let active = stack.active_window_kind_id.as_deref().unwrap_or_else(|| stack.children.first().map_or(\"\", |c| c.window_kind_id.as_str()));"],
  ["style_window(scene, windows, win_id, stack, &child.window_kind_id, active, peers, false);", "style_window(scene, windows, win_id, stack, &StackSlot { own: &child.window_kind_id, active, peers, zoomed: false });"],
  ["Some(stack) => style_window(scene, windows, zoomed, &stack, zid, zid, peers, true),", "Some(stack) => style_window(scene, windows, zoomed, &stack, &StackSlot { own: zid, active: zid, peers, zoomed: true }),"],
  ["windows: &[(String, NodeId)], path: Vec<usize>, peers: usize) {\n        match child {", "windows: &[(String, NodeId)], path: &[usize], peers: usize) {\n        match child {"],
  ["AxisState::new(is_row, path.clone())", "AxisState::new(is_row, path.to_vec())"],
  ["let mut child_path = path.clone();", "let mut child_path = path.to_vec();"],
  ["mount_child(scene, box_id, c, windows, child_path, peers);", "mount_child(scene, box_id, c, windows, &child_path, peers);"],
  ["&WindowLayoutChild::Axis(axis.clone()), windows, Vec::new(), peers),", "&WindowLayoutChild::Axis(axis.clone()), windows, &[], peers),"],
]);

patch("⌨️tui/⚙️engine/🦀️.rs", [["buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(crate::tui::theme::Surface::Base))", "buf.get(rect.x, rect.y).map_or(theme.surface(crate::tui::theme::Surface::Base), |c| c.bg)"]]);
patch("⌨️tui/🪀️widget/🦀️.rs", [[`s.options.get(s.index).map(String::as_str).unwrap_or("")`, `s.options.get(s.index).map_or("", String::as_str)`]]);

patch(target("➖️Divider"), [["buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Base))", "buf.get(rect.x, rect.y).map_or(theme.surface(Surface::Base), |c| c.bg)"]]);
patch(target("🏷️Label"), [["buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Base))", "buf.get(rect.x, rect.y).map_or(theme.surface(Surface::Base), |c| c.bg)"]]);

patch(target("🌳️Tree"), [["t.set_query(&query).then(|| WidgetSignal::ValueChanged(query))", "t.set_query(&query).then_some(WidgetSignal::ValueChanged(query))"]]);
patch(target("🧙️Wizard"), [["w.list.set_query(&query).then(|| WidgetSignal::ValueChanged(query))", "w.list.set_query(&query).then_some(WidgetSignal::ValueChanged(query))"]]);

patch(target("✏️Input"), [
  ["    fn changed(&self) -> Option<WidgetSignal> {\n        Some(WidgetSignal::ValueChanged(self.value.clone()))", "    fn changed(&self) -> WidgetSignal {\n        WidgetSignal::ValueChanged(self.value.clone())"],
  ["i.cursor = caret + c.len_utf8();\n            i.changed()", "i.cursor = caret + c.len_utf8();\n            Some(i.changed())"],
  ["(caret > 0).then(|| i.changed()).flatten()", "(caret > 0).then(|| i.changed())"],
  ["removed.then(|| i.changed()).flatten()", "removed.then(|| i.changed())"],
  ["i.cursor = to;\n    i.changed()", "i.cursor = to;\n    Some(i.changed())"],
  ["i.cursor = caret;\n    i.changed()", "i.cursor = caret;\n    Some(i.changed())"],
  ["i.cursor = caret + clean.len();\n    i.changed()", "i.cursor = caret + clean.len();\n    Some(i.changed())"],
]);

patch(target("🔀️Toggle"), [
  ["    fn flip(&mut self) -> Option<WidgetSignal> {\n        self.on = !self.on;\n        Some(WidgetSignal::Toggled(self.on))", "    fn flip(&mut self) -> WidgetSignal {\n        self.on = !self.on;\n        WidgetSignal::Toggled(self.on)"],
  ["Key::Char(' ') | Key::Enter => t.flip(),", "Key::Char(' ') | Key::Enter => Some(t.flip()),"],
  ["Key::Left | Key::Home if t.on => t.flip(),", "Key::Left | Key::Home if t.on => Some(t.flip()),"],
  ["Key::Right | Key::End if !t.on => t.flip(),", "Key::Right | Key::End if !t.on => Some(t.flip()),"],
  [".then(|| t.flip()).flatten()", ".then(|| t.flip())"],
]);

patch(target("🔽️Select"), [
  ["fn cycle(s: &mut SelectState, forward: bool) -> Option<WidgetSignal> {", "fn cycle(s: &mut SelectState, forward: bool) -> WidgetSignal {"],
  ["    Some(WidgetSignal::SelectionChanged(s.index))\n}\n\npub(crate) fn select_on_key", "    WidgetSignal::SelectionChanged(s.index)\n}\n\npub(crate) fn select_on_key"],
  ["Key::Left | Key::Up | Key::PageUp => cycle(s, false),", "Key::Left | Key::Up | Key::PageUp => Some(cycle(s, false)),"],
  ["Key::Right | Key::Down | Key::PageDown | Key::Enter => cycle(s, true),", "Key::Right | Key::Down | Key::PageDown | Key::Enter => Some(cycle(s, true)),"],
  ["MouseKind::Down(MouseButton::Left) => cycle(s, true),", "MouseKind::Down(MouseButton::Left) => Some(cycle(s, true)),"],
  ["MouseKind::Down(MouseButton::Right) => cycle(s, false),", "MouseKind::Down(MouseButton::Right) => Some(cycle(s, false)),"],
  ["MouseKind::Scroll { dy, .. } if dy != 0 => cycle(s, dy > 0),", "MouseKind::Scroll { dy, .. } if dy != 0 => Some(cycle(s, dy > 0)),"],
  ["buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Panel))", "buf.get(rect.x, rect.y).map_or(theme.surface(Surface::Panel), |c| c.bg)"],
  [`s.options.get(s.index).map(String::as_str).unwrap_or("")`, `s.options.get(s.index).map_or("", String::as_str)`],
]);

patch(target("📊️Table"), [
  ["let flex_width = if flex_count > 0 { remaining / flex_count } else { 0 };", "let flex_width = remaining.checked_div(flex_count).unwrap_or(0);"],
  [
    "pub(crate) fn paint_table_cell(buf: &mut CellBuffer, x: u16, y: u16, width: u16, text: &str, fg: [u8; 3], bg: [u8; 3], attrs: u8, align: TableAlign, clip: Rect) {\n    let (t, tw) = truncate_to(text, width);\n    let cell_x = match align {\n        TableAlign::Left => x,\n        TableAlign::Right => x + width.saturating_sub(tw),\n    };\n    buf.put_str(Pos { x: cell_x, y }, t, fg, bg, attrs, clip);",
    "/// 🖍️ Colours and attributes one table cell is painted with.\n#[derive(Clone, Copy)]\nstruct Ink {\n    fg: [u8; 3],\n    bg: [u8; 3],\n    attrs: u8,\n}\n\nfn paint_table_cell(buf: &mut CellBuffer, at: Pos, width: u16, text: &str, ink: Ink, align: TableAlign, clip: Rect) {\n    let (t, tw) = truncate_to(text, width);\n    let cell_x = match align {\n        TableAlign::Left => at.x,\n        TableAlign::Right => at.x + width.saturating_sub(tw),\n    };\n    buf.put_str(Pos { x: cell_x, y: at.y }, t, ink.fg, ink.bg, ink.attrs, clip);",
  ],
  ["paint_table_cell(buf, cx, content.y, w, &col.label, theme.role(Role::MutedForeground), bg, attr::BOLD, col.align, content);", "paint_table_cell(buf, Pos { x: cx, y: content.y }, w, &col.label, Ink { fg: theme.role(Role::MutedForeground), bg, attrs: attr::BOLD }, col.align, content);"],
  ["paint_table_cell(buf, cx, y, w, &text, row_fg, row_bg, attrs, col.align, content);", "paint_table_cell(buf, Pos { x: cx, y }, w, &text, Ink { fg: row_fg, bg: row_bg, attrs }, col.align, content);"],
  ["buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Window))", "buf.get(rect.x, rect.y).map_or(theme.surface(Surface::Window), |c| c.bg)"],
  [`row.cells.first().map(String::as_str).unwrap_or("")`, `row.cells.first().map_or("", String::as_str)`],
]);

patch(target("🪟️Window"), [
  [
    "const LIGHT: Lines",
    "/// 🖼️ What the outline painters share: the line set, the border and surface colours and the window's left and right columns.\n#[derive(Clone, Copy)]\nstruct Frame<'a> {\n    lines: &'a Lines,\n    border: Rgb,\n    bg: Rgb,\n    left: u16,\n    right: u16,\n}\n\nconst LIGHT: Lines",
  ],
  [
    "fn paint_corner_tab(buf: &mut CellBuffer, y: u16, tab: &WindowCornerTab, is_bottom: bool, g: &Lines, border: Rgb, bg: Rgb, paint_text: &dyn Fn(&mut CellBuffer, &WindowCornerTab, u16)) {",
    "fn paint_corner_tab(buf: &mut CellBuffer, y: u16, tab: &WindowCornerTab, is_bottom: bool, frame: &Frame, paint_text: &dyn Fn(&mut CellBuffer, &WindowCornerTab, u16)) {\n    let Frame { lines: g, border, bg, .. } = *frame;",
  ],
  [
    "fn paint_tab_seam(buf: &mut CellBuffer, tab: &WindowTab, y: u16, window_left: u16, window_right: u16, active: bool, is_bottom: bool, g: &Lines, border: Rgb, bg: Rgb) {",
    "fn paint_tab_seam(buf: &mut CellBuffer, tab: &WindowTab, y: u16, active: bool, is_bottom: bool, frame: &Frame) {\n    let Frame { lines: g, border, bg, left: window_left, right: window_right } = *frame;",
  ],
  [
    "fn paint_compact(buf: &mut CellBuffer, rect: Rect, w: &WindowState, theme: &Theme, layout: &WindowChipLayout, g: &Lines, border: Rgb, bg: Rgb) {",
    "fn paint_compact(buf: &mut CellBuffer, rect: Rect, w: &WindowState, theme: &Theme, layout: &WindowChipLayout, frame: &Frame) {\n    let Frame { lines: g, border, bg, .. } = *frame;",
  ],
  ["    let layout = window_chip_layout(w, rect);\n    fill_silhouette(", "    let layout = window_chip_layout(w, rect);\n    let frame = Frame { lines: g, border, bg, left: rect.x, right: right_x };\n    fill_silhouette("],
  ["paint_compact(buf, rect, w, theme, &layout, g, border, bg);", "paint_compact(buf, rect, w, theme, &layout, &frame);"],
  ["paint_corner_tab(buf, y, tab, is_bottom, g, border, bg, &|buf", "paint_corner_tab(buf, y, tab, is_bottom, &frame, &|buf"],
  ["paint_tab_seam(buf, &tab.as_window_tab(), top_body_y, rect.x, right_x, open, false, g, border, bg);", "paint_tab_seam(buf, &tab.as_window_tab(), top_body_y, open, false, &frame);"],
  ["paint_tab_seam(buf, &tab.as_window_tab(), bottom_body_y, rect.x, right_x, open, true, g, border, bg);", "paint_tab_seam(buf, &tab.as_window_tab(), bottom_body_y, open, true, &frame);"],
]);

console.log("clippy patches applied");
