use crate::tui::ansi::{emit_runs, AnsiPatch};
use crate::tui::cell::{diff, Cell, CellBuffer, DiffRun};
use crate::tui::chrome::{ChromeState, WindowHit};
use crate::tui::dialog::DialogState;
use crate::tui::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::menu::{MenuItem, MenuState};
use crate::tui::palette::PaletteState;
use crate::tui::scene::{AxisState, Node, NodeContent, NodeId, Scene, LAYOUT_DIRTY, PAINT_DIRTY};
use crate::tui::text::WidthMode;
use crate::tui::theme::{Role, Status, Theme};
use crate::tui::tooltip::{TooltipState, TOOLTIP_DELAY_MS};
use crate::tui::widget::{CursorSpec, WidgetSignal, WidgetState};
use ui_styling::appearance::AppearanceName;

/// ⏱️ Presses on the same cell within this window count as one multi-click.
const CLICK_WINDOW_MS: u64 = 500;
/// 🎞️ Minimum spacing between painted frames when the host paces with `render_due`.
const FRAME_BUDGET_MS: u64 = 16;
/// 🌀️ How long one spinner frame of a running tab or widget shows.
const SPIN_FRAME_MS: u64 = 125;
/// 📱️ Terminals shorter than this many rows get the one-row window chrome.
pub const COMPACT_ROWS: u16 = 30;

type Signals = Vec<(NodeId, WidgetSignal)>;

/// 🖱️ What the pointer is holding between a button press and its release.
enum Press {
    Widget { node: NodeId, button: MouseButton, clicks: u8 },
    Splitter { axis: NodeId, index: usize, last: u16 },
    Tab { chrome: NodeId, from: usize, over: usize, start: Pos, moved: bool },
}

struct Click {
    button: MouseButton,
    pos: Pos,
    at_ms: u64,
    count: u8,
}

/// 🥞️ Where an overlay sits relative to the terminal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Placement {
    Anchored(Pos),
    Centered,
    TopCentered,
    Tip(Pos),
}

struct Overlay {
    id: NodeId,
    placement: Placement,
    modal: bool,
    dismiss_outside: bool,
    saved_focus: Option<NodeId>,
}

/// 🏭 The retained-mode pipeline: layout-if-dirty, paint, damage-diff, ANSI.
pub struct Tui {
    pub scene: Scene,
    pub theme: Theme,
    size: Size,
    front: CellBuffer,
    back: CellBuffer,
    focus: Option<NodeId>,
    hovered: Option<NodeId>,
    capture: Option<NodeId>,
    press: Option<Press>,
    last_click: Option<Click>,
    overlays: Vec<Overlay>,
    now_ms: u64,
    ticked: bool,
    last_render_ms: Option<u64>,
    hover_pos: Option<Pos>,
    hover_since_ms: u64,
    hover_tip: Option<String>,
    tooltip: Option<NodeId>,
    layout_stale: bool,
    full_redraw: bool,
    app_focused: bool,
    pending: Signals,
    animated: bool,
    width_mode: WidthMode,
}

fn focusable(scene: &Scene, id: NodeId) -> bool {
    matches!(scene.try_node(id).map(|node| &node.content), Some(NodeContent::Widget(widget)) if widget.interactive()) && scene.shown(id)
}

fn collect_focusables(scene: &Scene, id: NodeId, out: &mut Vec<NodeId>) {
    let node = scene.node(id);
    if !node.visible {
        return;
    }
    if matches!(&node.content, NodeContent::Widget(widget) if widget.interactive()) {
        out.push(id);
    }
    for &child in node.children() {
        collect_focusables(scene, child, out);
    }
}

fn collect_windows(scene: &Scene, id: NodeId, out: &mut Vec<NodeId>) {
    let node = scene.node(id);
    if matches!(&node.content, NodeContent::Chrome(ChromeState::Window(_))) {
        out.push(id);
    }
    for &child in node.children() {
        collect_windows(scene, child, out);
    }
}

fn collect_widgets(scene: &Scene, id: NodeId, out: &mut Vec<NodeId>) {
    let node = scene.node(id);
    if !node.visible {
        return;
    }
    if matches!(node.content, NodeContent::Widget(_)) {
        out.push(id);
    }
    for &child in node.children() {
        collect_widgets(scene, child, out);
    }
}

fn clamp_to(rect: Rect, viewport: Size) -> Rect {
    let width = rect.width.min(viewport.width);
    let height = rect.height.min(viewport.height);
    Rect::new(rect.x.min(viewport.width - width), rect.y.min(viewport.height - height), width, height)
}

impl Tui {
    pub fn new(size: Size, theme: Theme) -> Self {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let mut front = CellBuffer::new(size, blank);
        let mut back = CellBuffer::new(size, blank);
        front.set_width_mode(WidthMode::Scalar);
        back.set_width_mode(WidthMode::Scalar);
        Self {
            scene: Scene::new(),
            theme,
            size,
            front,
            back,
            focus: None,
            hovered: None,
            capture: None,
            press: None,
            last_click: None,
            overlays: Vec::new(),
            now_ms: 0,
            ticked: false,
            last_render_ms: None,
            hover_pos: None,
            hover_since_ms: 0,
            hover_tip: None,
            tooltip: None,
            layout_stale: true,
            full_redraw: true,
            app_focused: true,
            pending: Vec::new(),
            animated: false,
            width_mode: WidthMode::Scalar,
        }
    }

    /// 🖼️ The last fully-composed frame, for hosts/tests that need to inspect the actual render.
    pub fn frame(&self) -> &CellBuffer {
        &self.front
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        self.front.resize(size, blank);
        self.back.resize(size, blank);
        self.layout_stale = true;
        self.full_redraw = true;
    }

    /// 🔤️ Chooses how text is measured (clusters or scalars) for every buffer from the terminal's reported Unicode support; repaints in full.
    pub fn set_width_mode(&mut self, mode: WidthMode) {
        if self.width_mode == mode {
            return;
        }
        self.width_mode = mode;
        self.front.set_width_mode(mode);
        self.back.set_width_mode(mode);
        self.layout_stale = true;
        self.full_redraw = true;
    }

    pub fn width_mode(&self) -> WidthMode {
        self.width_mode
    }

    pub fn set_appearance(&mut self, appearance: AppearanceName) {
        self.theme.set_appearance(appearance);
        self.full_redraw = true;
    }

    /// 🎯 The node holding keyboard focus; `None` once it was removed or hidden.
    pub fn focus(&self) -> Option<NodeId> {
        self.focus.filter(|&id| self.scene.contains(id) && self.scene.shown(id))
    }

    /// 🔦 Moves keyboard focus to `id` (any live node); the engine is the only source of focus.
    /// While a modal overlay is open the request waits: the overlay keeps focus and `id` receives it when the overlay closes.
    pub fn set_focus(&mut self, id: Option<NodeId>) {
        let id = id.filter(|&id| self.scene.contains(id));
        if let Some(at) = self.overlays.iter().rposition(|overlay| overlay.modal) {
            let modal = self.overlays[at].id;
            if id.is_none_or(|node| !self.scene.lineage(node).contains(&modal)) {
                if id.is_some() {
                    self.overlays[at].saved_focus = id;
                }
                return;
            }
        }
        self.assign_focus(id);
    }

    fn assign_focus(&mut self, id: Option<NodeId>) {
        if self.focus == id {
            return;
        }
        let old = self.focus;
        for node in [old, id].into_iter().flatten() {
            if self.scene.contains(node) {
                self.scene.mark_dirty(node, PAINT_DIRTY);
            }
        }
        self.focus = id;
        if let Some(old) = old {
            self.notify_focus(old, false);
        }
        if let (Some(new), true) = (id, self.app_focused) {
            self.notify_focus(new, true);
        }
    }

    /// 🔔️ Tells a widget it gained or lost keyboard focus; whatever it answers (a terminal's focus report) is delivered with the next dispatch result.
    fn notify_focus(&mut self, id: NodeId, gained: bool) {
        if !self.scene.contains(id) {
            return;
        }
        let signal = match &mut self.scene.node_raw_mut(id).content {
            NodeContent::Widget(widget) => widget.on_focus(gained),
            _ => None,
        };
        self.pending.extend(signal.map(|signal| (id, signal)));
    }

    /// 🪟 The window chrome node that contains `id`.
    pub fn window_of(&self, id: NodeId) -> Option<NodeId> {
        self.scene.lineage(id).into_iter().find(|&node| matches!(self.scene.node(node).content, NodeContent::Chrome(ChromeState::Window(_))))
    }

    /// 🥇 The window that holds keyboard focus.
    pub fn focused_window(&self) -> Option<NodeId> {
        self.focus().and_then(|id| self.window_of(id))
    }

    fn modal(&self) -> Option<&Overlay> {
        self.overlays.iter().rev().find(|overlay| overlay.modal)
    }

    /// 🔁 The focus ring: interactive, visible widgets in tree order; a modal overlay traps it.
    pub fn focus_ring(&self) -> Vec<NodeId> {
        let mut ring = Vec::new();
        match self.modal() {
            Some(overlay) => collect_focusables(&self.scene, overlay.id, &mut ring),
            None => collect_focusables(&self.scene, self.scene.root(), &mut ring),
        }
        ring
    }

    fn step_focus(&mut self, forward: bool, out: &mut Signals) {
        let ring = self.focus_ring();
        if ring.is_empty() {
            return;
        }
        let current = self.focus().and_then(|focus| ring.iter().position(|node| *node == focus));
        let next = match (current, forward) {
            (Some(i), true) => (i + 1) % ring.len(),
            (Some(i), false) => (i + ring.len() - 1) % ring.len(),
            (None, true) => 0,
            (None, false) => ring.len() - 1,
        };
        self.focus_node(Some(ring[next]), out);
    }

    pub fn focus_next(&mut self) {
        self.step_focus(true, &mut Vec::new());
    }

    pub fn focus_prev(&mut self) {
        self.step_focus(false, &mut Vec::new());
    }

    /// ⏭️ Moves focus to the first widget of the next (`delta > 0`) or previous window.
    pub fn focus_window(&mut self, delta: i32) -> Signals {
        let mut windows = Vec::new();
        collect_windows(&self.scene, self.scene.root(), &mut windows);
        windows.retain(|&window| self.scene.shown(window));
        let mut out = Vec::new();
        if windows.is_empty() {
            return out;
        }
        let current = self.focused_window().and_then(|window| windows.iter().position(|w| *w == window));
        let next = match current {
            Some(i) => (i as i32 + delta).rem_euclid(windows.len() as i32) as usize,
            None => 0,
        };
        let mut ring = Vec::new();
        collect_focusables(&self.scene, windows[next], &mut ring);
        if let Some(&target) = ring.first() {
            self.focus_node(Some(target), &mut out);
        }
        out
    }

    fn focus_node(&mut self, id: Option<NodeId>, out: &mut Signals) {
        let before = self.focused_window();
        self.set_focus(id);
        let after = self.focused_window();
        if after != before {
            if let Some(window) = after {
                out.push((window, WidgetSignal::WindowFocus));
            }
        }
    }

    fn focus_target(&self, hit: NodeId) -> Option<NodeId> {
        let lineage = self.scene.lineage(hit);
        if let Some(&node) = lineage.iter().find(|&&node| focusable(&self.scene, node)) {
            return Some(node);
        }
        let window = lineage.iter().find(|&&node| matches!(self.scene.node(node).content, NodeContent::Chrome(ChromeState::Window(_))))?;
        let mut ring = Vec::new();
        collect_focusables(&self.scene, *window, &mut ring);
        ring.first().copied()
    }

    /// ✏️ The focused widget's terminal cursor, placed by its current rect.
    pub fn cursor(&self) -> Option<CursorSpec> {
        let node = self.scene.node(self.focus()?);
        let NodeContent::Widget(widget) = &node.content else { return None };
        widget.cursor(node.rect).filter(|spec| spec.pos.x < self.size.width && spec.pos.y < self.size.height)
    }

    /// 👆️ The node under the pointer.
    pub fn hovered(&self) -> Option<NodeId> {
        self.hovered.filter(|&id| self.scene.contains(id))
    }

    /// 🪤️ Names the node that owns the pointer until released with `None`.
    pub fn capture(&mut self, node: Option<NodeId>) {
        self.capture = node.filter(|&id| self.scene.contains(id));
    }

    /// ⏰ Advances the clock to `now_ms` (any monotonic millisecond source), ticks timed widgets, opens due tooltips;
    /// true when a repaint is due.
    pub fn tick(&mut self, now_ms: u64) -> bool {
        self.now_ms = now_ms;
        self.ticked = true;
        let mut widgets = Vec::new();
        collect_widgets(&self.scene, self.scene.root(), &mut widgets);
        let mut animated = false;
        for id in widgets {
            let due = match &mut self.scene.node_raw_mut(id).content {
                NodeContent::Widget(widget) => widget.tick(now_ms),
                _ => false,
            };
            if due {
                animated = true;
                self.scene.mark_dirty(id, PAINT_DIRTY);
            }
        }
        let mut windows = Vec::new();
        collect_windows(&self.scene, self.scene.root(), &mut windows);
        let frame = now_ms / SPIN_FRAME_MS;
        for id in windows {
            let spinning = matches!(&self.scene.node(id).content, NodeContent::Chrome(ChromeState::Window(w)) if w.stack_tabs.iter().any(|tab| tab.status == Some(Status::Running)));
            animated |= spinning;
            if let NodeContent::Chrome(ChromeState::Window(w)) = &mut self.scene.node_raw_mut(id).content {
                if spinning && w.spin != frame {
                    w.spin = frame;
                    self.scene.mark_dirty(id, PAINT_DIRTY);
                }
            }
        }
        self.animated = animated;
        if self.tooltip.is_none() && now_ms.saturating_sub(self.hover_since_ms) >= TOOLTIP_DELAY_MS {
            if let (Some(text), Some(pos)) = (self.hover_tip.clone(), self.hover_pos) {
                self.show_tooltip(pos, text);
            }
        }
        self.dirty()
    }

    /// 🛎️ Whether anything changed since the last painted frame.
    pub fn dirty(&self) -> bool {
        self.full_redraw || self.layout_stale || self.scene.dirty_flags() != 0
    }

    /// ⏳ Milliseconds until the engine next needs a tick: a pending tooltip or a deferred frame; `None` when idle.
    pub fn deadline_ms(&self) -> Option<u64> {
        let tooltip = (self.tooltip.is_none() && self.hover_tip.is_some()).then(|| (self.hover_since_ms + TOOLTIP_DELAY_MS).saturating_sub(self.now_ms));
        let frame = self.dirty().then(|| self.last_render_ms.map_or(0, |at| (at + FRAME_BUDGET_MS).saturating_sub(self.now_ms)));
        let animation = self.animated.then(|| SPIN_FRAME_MS - self.now_ms % SPIN_FRAME_MS);
        [tooltip, frame, animation].into_iter().flatten().min()
    }

    /// 🚀 Renders when something changed and the frame budget since the last frame has elapsed.
    pub fn render_due(&mut self, now_ms: u64) -> Option<AnsiPatch> {
        self.now_ms = now_ms;
        if !self.dirty() || self.last_render_ms.is_some_and(|at| now_ms.saturating_sub(at) < FRAME_BUDGET_MS) {
            return None;
        }
        Some(self.render())
    }

    /// 📐️ Solves the scene's layout for the current size without painting.
    pub fn layout(&mut self) {
        self.sync_chrome();
        let viewport = Rect::new(0, 0, self.size.width, self.size.height);
        crate::tui::layout::solve(&mut self.scene, viewport);
        let overlay_root = self.scene.overlay_root();
        self.scene.node_raw_mut(overlay_root).rect = viewport;
        self.place_overlays();
        self.scene.clear_dirty(LAYOUT_DIRTY);
        self.layout_stale = false;
    }

    fn ensure_layout(&mut self) {
        if self.layout_stale || self.scene.dirty_flags() & LAYOUT_DIRTY != 0 {
            self.layout();
        }
    }

    fn sync_chrome(&mut self) {
        let mut windows = Vec::new();
        collect_windows(&self.scene, self.scene.root(), &mut windows);
        let focused = self.focused_window();
        let compact = self.size.height < COMPACT_ROWS;
        for id in windows {
            let mut flags = None;
            if let NodeContent::Chrome(ChromeState::Window(window)) = &self.scene.node(id).content {
                let focus = if focused.is_some() { Some(focused == Some(id)) } else { None };
                if focus.is_some_and(|f| f != window.focused) || window.compact != compact {
                    flags = Some(focus);
                }
            }
            if let Some(focus) = flags {
                if let NodeContent::Chrome(ChromeState::Window(window)) = &mut self.scene.node_raw_mut(id).content {
                    if let Some(focus) = focus {
                        window.focused = focus;
                    }
                    window.compact = compact;
                }
                self.scene.mark_dirty(id, LAYOUT_DIRTY | PAINT_DIRTY);
            }
        }
    }

    fn open_overlay(&mut self, widget: WidgetState, placement: Placement, modal: bool, dismiss_outside: bool) -> NodeId {
        let mut node = Node::new(NodeContent::Widget(widget));
        node.hittable = modal;
        let id = self.scene.add(self.scene.overlay_root(), node);
        let saved_focus = self.focus();
        if modal {
            self.set_hover_node(None);
        }
        self.overlays.push(Overlay { id, placement, modal, dismiss_outside, saved_focus });
        if modal {
            self.assign_focus(Some(id));
        }
        self.layout_stale = true;
        id
    }

    /// 🧾 Opens a context menu at `anchor`; it traps focus until an item is chosen or it is dismissed.
    pub fn open_menu(&mut self, anchor: Pos, items: Vec<MenuItem>) -> NodeId {
        self.open_overlay(WidgetState::Menu(MenuState::new(items)), Placement::Anchored(anchor), true, true)
    }

    /// 💬️ Opens a centred modal dialog; it traps focus until a button is chosen or it is dismissed.
    pub fn open_dialog(&mut self, dialog: DialogState) -> NodeId {
        let dismiss_outside = dialog.dismiss_outside;
        self.open_overlay(WidgetState::Dialog(dialog), Placement::Centered, true, dismiss_outside)
    }

    /// ⌨️ Opens a command palette near the top of the terminal; it traps focus until an entry is chosen or it is dismissed.
    pub fn open_palette(&mut self, palette: PaletteState) -> NodeId {
        self.open_overlay(WidgetState::Palette(palette), Placement::TopCentered, true, true)
    }

    /// 💡️ Shows a tooltip beside `anchor`; it never takes focus or the pointer.
    pub fn show_tooltip(&mut self, anchor: Pos, text: impl Into<String>) -> NodeId {
        self.hide_tooltip();
        let id = self.open_overlay(WidgetState::Tooltip(TooltipState::new(text)), Placement::Tip(anchor), false, false);
        self.tooltip = Some(id);
        id
    }

    fn hide_tooltip(&mut self) {
        if let Some(id) = self.tooltip.take() {
            self.close_overlay(id);
        }
    }

    /// 📚 Overlay nodes in z-order, topmost last.
    pub fn overlays(&self) -> Vec<NodeId> {
        self.overlays.iter().map(|overlay| overlay.id).collect()
    }

    /// 🚪 Closes an overlay and gives focus back to what held it before.
    pub fn close_overlay(&mut self, id: NodeId) {
        let Some(at) = self.overlays.iter().position(|overlay| overlay.id == id) else { return };
        let overlay = self.overlays.remove(at);
        if self.tooltip == Some(id) {
            self.tooltip = None;
        }
        for other in &mut self.overlays {
            if other.saved_focus == Some(id) {
                other.saved_focus = overlay.saved_focus;
            }
        }
        if self.focus == Some(id) {
            let restore = overlay.saved_focus.filter(|&saved| self.scene.contains(saved));
            self.focus = restore;
            if let Some(node) = restore {
                self.scene.mark_dirty(node, PAINT_DIRTY);
            }
        }
        self.scene.remove(id);
        self.layout_stale = true;
    }

    fn place_overlays(&mut self) {
        let viewport = self.size;
        for overlay in &self.overlays {
            let Some(node) = self.scene.try_node(overlay.id) else { continue };
            let NodeContent::Widget(widget) = &node.content else { continue };
            let size = widget.overlay_size(viewport);
            let (w, h) = (size.width.min(viewport.width), size.height.min(viewport.height));
            let (x, y) = match overlay.placement {
                Placement::Anchored(anchor) => {
                    let x = if anchor.x + w > viewport.width { anchor.x.saturating_sub(w.saturating_sub(1)) } else { anchor.x };
                    let y = if anchor.y + h > viewport.height { anchor.y.saturating_sub(h.saturating_sub(1)) } else { anchor.y };
                    (x, y)
                }
                Placement::Centered => (viewport.width.saturating_sub(w) / 2, viewport.height.saturating_sub(h) / 2),
                Placement::TopCentered => (viewport.width.saturating_sub(w) / 2, (viewport.height / 6).max(1)),
                Placement::Tip(anchor) => {
                    let x = if anchor.x + 1 + w > viewport.width { anchor.x.saturating_sub(w) } else { anchor.x + 1 };
                    let y = if anchor.y + 1 + h > viewport.height { anchor.y.saturating_sub(h) } else { anchor.y + 1 };
                    (x, y)
                }
            };
            let rect = clamp_to(Rect::new(x, y, w, h), viewport);
            let id = overlay.id;
            self.scene.node_raw_mut(id).rect = rect;
        }
    }

    fn overlay_signals(&mut self, node: NodeId, signal: &Option<WidgetSignal>, key: Option<&KeyEvent>, out: &mut Signals) {
        if !self.overlays.iter().any(|overlay| overlay.id == node && overlay.modal) {
            return;
        }
        match (signal, key) {
            (Some(WidgetSignal::Activated(_)), _) => self.close_overlay(node),
            (None, Some(KeyEvent { key: Key::Esc, .. })) => {
                out.push((node, WidgetSignal::Dismissed));
                self.close_overlay(node);
            }
            _ => {}
        }
    }

    /// 🎹 Routes one input event: keys to the focused widget, pointer events through hit-test, hover, capture and drag state.
    pub fn dispatch(&mut self, event: &Event) -> Signals {
        self.dispatch_at(event, self.now_ms)
    }

    /// 🕰️ Like `dispatch` with an explicit clock reading, so click counting and tooltip dwell are deterministic.
    pub fn dispatch_at(&mut self, event: &Event, now_ms: u64) -> Signals {
        self.now_ms = self.now_ms.max(now_ms);
        self.ticked |= now_ms > 0;
        self.sanitize();
        let mut out = Vec::new();
        match event {
            Event::Resize(size) => {
                self.resize(*size);
                self.set_hover_node(None);
            }
            Event::Key(key) => self.dispatch_key(key, &mut out),
            Event::Paste(text) => {
                if let Some(id) = self.focus() {
                    let signal = match &mut self.scene.node_raw_mut(id).content {
                        NodeContent::Widget(widget) => widget.on_paste(text),
                        _ => None,
                    };
                    self.scene.mark_dirty(id, PAINT_DIRTY);
                    out.extend(signal.map(|signal| (id, signal)));
                }
            }
            Event::Mouse(mouse) => {
                self.ensure_layout();
                self.dispatch_mouse(mouse, &mut out);
            }
            Event::FocusLost => {
                self.set_hover_node(None);
                self.press = None;
                self.app_focused = false;
                if let Some(id) = self.focus() {
                    self.notify_focus(id, false);
                }
            }
            Event::FocusGained => {
                self.app_focused = true;
                if let Some(id) = self.focus() {
                    self.notify_focus(id, true);
                }
            }
            Event::Wake => {}
        }
        let mut delivered = std::mem::take(&mut self.pending);
        delivered.extend(out);
        delivered
    }

    fn sanitize(&mut self) {
        if self.focus.is_some_and(|id| !self.scene.contains(id)) {
            self.focus = None;
        }
        if self.hovered.is_some_and(|id| !self.scene.contains(id)) {
            self.hovered = None;
        }
        if self.capture.is_some_and(|id| !self.scene.contains(id)) {
            self.capture = None;
        }
        let stale = match &self.press {
            Some(Press::Widget { node, .. }) => !self.scene.contains(*node),
            Some(Press::Splitter { axis, .. }) => !self.scene.contains(*axis),
            Some(Press::Tab { chrome, .. }) => !self.scene.contains(*chrome),
            None => false,
        };
        if stale {
            self.press = None;
        }
        let gone: Vec<NodeId> = self.overlays.iter().filter(|overlay| !self.scene.contains(overlay.id)).map(|overlay| overlay.id).collect();
        for id in gone {
            self.overlays.retain(|overlay| overlay.id != id);
            if self.tooltip == Some(id) {
                self.tooltip = None;
            }
        }
    }

    fn dispatch_key(&mut self, key: &KeyEvent, out: &mut Signals) {
        self.hide_tooltip();
        let target = self.focus();
        let claims = target.is_some_and(|id| matches!(&self.scene.node(id).content, NodeContent::Widget(widget) if widget.claims_tab()));
        if matches!(key.key, Key::Tab | Key::BackTab) && !claims {
            self.step_focus(key.key == Key::Tab, out);
            return;
        }
        let Some(id) = target else { return };
        let signal = match &mut self.scene.node_raw_mut(id).content {
            NodeContent::Widget(widget) => widget.on_key(key),
            _ => None,
        };
        self.scene.mark_dirty(id, PAINT_DIRTY);
        self.overlay_signals(id, &signal, Some(key), out);
        out.extend(signal.map(|signal| (id, signal)));
    }

    fn widget_at(&self, hit: NodeId) -> Option<NodeId> {
        self.scene.lineage(hit).into_iter().find(|&node| matches!(self.scene.node(node).content, NodeContent::Widget(_)))
    }

    fn route(&mut self, id: NodeId, mouse: &MouseEvent) -> Option<WidgetSignal> {
        let rect = self.scene.try_node(id)?.rect;
        let signal = match &mut self.scene.node_raw_mut(id).content {
            NodeContent::Widget(widget) => widget.on_mouse(rect, mouse),
            _ => None,
        };
        self.scene.mark_dirty(id, PAINT_DIRTY);
        signal
    }

    fn inside_modal(&self, hit: Option<NodeId>) -> bool {
        match (self.modal(), hit) {
            (None, _) => true,
            (Some(modal), Some(hit)) => self.scene.lineage(hit).contains(&modal.id),
            (Some(_), None) => false,
        }
    }

    fn count_clicks(&mut self, mouse: &MouseEvent) -> MouseEvent {
        let MouseKind::Down(button) = mouse.kind else {
            if let Some(Press::Widget { clicks, .. }) = &self.press {
                return MouseEvent { clicks: *clicks, ..*mouse };
            }
            return *mouse;
        };
        let count = if mouse.clicks > 1 {
            mouse.clicks
        } else if let (true, Some(last)) = (self.ticked, &self.last_click) {
            if last.button == button && last.pos == mouse.pos && self.now_ms.saturating_sub(last.at_ms) <= CLICK_WINDOW_MS {
                last.count % 3 + 1
            } else {
                1
            }
        } else {
            1
        };
        self.last_click = Some(Click { button, pos: mouse.pos, at_ms: self.now_ms, count });
        MouseEvent { clicks: count, ..*mouse }
    }

    fn dispatch_mouse(&mut self, mouse: &MouseEvent, out: &mut Signals) {
        let mouse = self.count_clicks(mouse);
        let hit = self.scene.hit(mouse.pos);
        match mouse.kind {
            MouseKind::Move => self.mouse_move(&mouse, hit, out),
            MouseKind::Down(button) => self.mouse_down(&mouse, button, hit, out),
            MouseKind::Drag(button) => self.mouse_drag(&mouse, button, hit, out),
            MouseKind::Up(button) => self.mouse_up(&mouse, button, out),
            MouseKind::Scroll { .. } => {
                if !self.inside_modal(hit) {
                    return;
                }
                self.hide_tooltip();
                let target = self.capture.filter(|&id| self.scene.contains(id)).or_else(|| hit.and_then(|hit| self.widget_at(hit)));
                if let Some(id) = target {
                    out.extend(self.route(id, &mouse).map(|signal| (id, signal)));
                }
            }
        }
    }

    fn mouse_move(&mut self, mouse: &MouseEvent, hit: Option<NodeId>, out: &mut Signals) {
        let hit = if self.inside_modal(hit) { hit } else { None };
        self.update_hover(hit, Some(mouse.pos));
        let target = self.capture.filter(|&id| self.scene.contains(id)).or_else(|| hit.and_then(|hit| self.widget_at(hit)));
        if let Some(id) = target {
            out.extend(self.route(id, mouse).map(|signal| (id, signal)));
        }
    }

    fn mouse_down(&mut self, mouse: &MouseEvent, button: MouseButton, hit: Option<NodeId>, out: &mut Signals) {
        if !self.inside_modal(hit) {
            if let Some(modal) = self.modal() {
                let (id, dismiss) = (modal.id, modal.dismiss_outside);
                if dismiss {
                    out.push((id, WidgetSignal::Dismissed));
                    self.close_overlay(id);
                }
            }
            return;
        }
        let Some(hit) = hit else { return };
        self.hide_tooltip();
        if let Some(captured) = self.capture.filter(|&id| self.scene.contains(id)) {
            self.press = Some(Press::Widget { node: captured, button, clicks: mouse.clicks });
            out.extend(self.route(captured, mouse).map(|signal| (captured, signal)));
            return;
        }
        let is_window = matches!(self.scene.node(hit).content, NodeContent::Chrome(ChromeState::Window(_)));
        let is_axis = matches!(self.scene.node(hit).content, NodeContent::Axis(_));
        if is_window && self.chrome_down(hit, mouse, button, out) {
            return;
        }
        if button == MouseButton::Left && is_axis {
            let (rect, children) = (self.scene.node(hit).rect, self.scene.node(hit).children().to_vec());
            if let NodeContent::Axis(axis) = &self.scene.node(hit).content {
                if let Some(index) = gutter_at(&self.scene, axis, rect, &children, mouse.pos) {
                    let last = if axis.horizontal { mouse.pos.x } else { mouse.pos.y };
                    self.press = Some(Press::Splitter { axis: hit, index, last });
                    return;
                }
            }
        }
        let target = self.focus_target(hit);
        if target.is_some() {
            self.focus_node(target, out);
        }
        let widget = self.widget_at(hit);
        let mut signal = None;
        if let Some(id) = widget {
            self.press = Some(Press::Widget { node: id, button, clicks: mouse.clicks });
            signal = self.route(id, mouse);
            out.extend(signal.clone().map(|signal| (id, signal)));
            self.overlay_signals(id, &signal, None, out);
        }
        if button == MouseButton::Right && signal.is_none() {
            out.push((widget.unwrap_or(hit), WidgetSignal::ContextMenu { pos: mouse.pos, item: None }));
        }
    }

    fn chrome_down(&mut self, node: NodeId, mouse: &MouseEvent, button: MouseButton, out: &mut Signals) -> bool {
        let rect = self.scene.node(node).rect;
        let NodeContent::Chrome(chrome) = &self.scene.node(node).content else { return false };
        let Some(target) = chrome.window_target(rect, mouse.pos) else { return false };
        match (button, target) {
            (MouseButton::Left, WindowHit::Close(i)) => out.push((node, WidgetSignal::WindowClose(i))),
            (MouseButton::Left, WindowHit::Maximize(i)) => out.push((node, WidgetSignal::WindowMaximize(i))),
            (MouseButton::Left, WindowHit::NewTab(i)) => out.push((node, WidgetSignal::WindowNewTab(i))),
            (MouseButton::Left, WindowHit::OverflowPrev(i) | WindowHit::OverflowNext(i)) => out.push((node, WidgetSignal::WindowTabActivated(i))),
            (MouseButton::Left, WindowHit::Tab(i)) => {
                let target = self.focus_target(node);
                self.focus_node(target, out);
                out.push((node, WidgetSignal::WindowTabActivated(i)));
                self.press = Some(Press::Tab { chrome: node, from: i, over: i, start: mouse.pos, moved: false });
            }
            (MouseButton::Right, WindowHit::Tab(i)) => out.push((node, WidgetSignal::ContextMenu { pos: mouse.pos, item: Some(i) })),
            _ => {}
        }
        true
    }

    fn mouse_drag(&mut self, mouse: &MouseEvent, button: MouseButton, hit: Option<NodeId>, out: &mut Signals) {
        match self.press.take() {
            Some(Press::Widget { node, button: held, clicks }) if held == button => {
                self.press = Some(Press::Widget { node, button: held, clicks });
                out.extend(self.route(node, mouse).map(|signal| (node, signal)));
            }
            Some(Press::Splitter { axis, index, last }) => {
                let mut next = last;
                if let NodeContent::Axis(state) = &self.scene.node(axis).content {
                    let at = if state.horizontal { mouse.pos.x } else { mouse.pos.y };
                    let delta = i32::from(at) - i32::from(last);
                    if delta != 0 {
                        let mut path = state.path.clone();
                        path.push(index);
                        out.push((axis, WidgetSignal::SplitterDragged { path, delta: delta.clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16 }));
                        next = at;
                    }
                }
                self.press = Some(Press::Splitter { axis, index, last: next });
            }
            Some(Press::Tab { chrome, from, over, start, moved }) => {
                let rect = self.scene.node(chrome).rect;
                let mut over = over;
                let mut moved = moved || mouse.pos != start;
                if let NodeContent::Chrome(state) = &self.scene.node(chrome).content {
                    if let Some(index) = state.window_drop_index(rect, mouse.pos) {
                        over = index;
                    }
                }
                if moved && over != from {
                    moved = true;
                }
                if let NodeContent::Chrome(state) = &mut self.scene.node_raw_mut(chrome).content {
                    state.set_drop_target((moved && over != from).then_some(over));
                }
                self.scene.mark_dirty(chrome, PAINT_DIRTY);
                self.press = Some(Press::Tab { chrome, from, over, start, moved });
            }
            other => {
                self.press = other;
                self.mouse_move(mouse, hit, out);
            }
        }
    }

    fn mouse_up(&mut self, mouse: &MouseEvent, button: MouseButton, out: &mut Signals) {
        match self.press.take() {
            Some(Press::Widget { node, button: held, clicks }) => {
                if held == button {
                    out.extend(self.route(node, mouse).map(|signal| (node, signal)));
                } else {
                    self.press = Some(Press::Widget { node, button: held, clicks });
                }
            }
            Some(Press::Tab { chrome, from, over, moved, .. }) => {
                if let NodeContent::Chrome(state) = &mut self.scene.node_raw_mut(chrome).content {
                    state.set_drop_target(None);
                }
                self.scene.mark_dirty(chrome, PAINT_DIRTY);
                if moved && over != from {
                    out.push((chrome, WidgetSignal::TabMoved { from, to: over }));
                }
            }
            Some(Press::Splitter { .. }) | None => {}
        }
    }

    fn hover_target(&self, hit: NodeId) -> Option<NodeId> {
        self.scene.lineage(hit).into_iter().find(|&node| matches!(self.scene.node(node).content, NodeContent::Widget(_) | NodeContent::Chrome(_) | NodeContent::Axis(_)))
    }

    fn apply_hover(&mut self, id: NodeId, pos: Option<Pos>) -> bool {
        let rect = self.scene.node(id).rect;
        let children = self.scene.node(id).children().to_vec();
        let changed = match &self.scene.node(id).content {
            NodeContent::Axis(axis) => {
                let hover = pos.and_then(|pos| gutter_at(&self.scene, axis, rect, &children, pos));
                let changed = axis.hover != hover;
                if changed {
                    if let NodeContent::Axis(axis) = &mut self.scene.node_raw_mut(id).content {
                        axis.hover = hover;
                    }
                }
                changed
            }
            NodeContent::Widget(_) => match &mut self.scene.node_raw_mut(id).content {
                NodeContent::Widget(widget) => widget.set_hover(rect, pos),
                _ => false,
            },
            NodeContent::Chrome(_) => match &mut self.scene.node_raw_mut(id).content {
                NodeContent::Chrome(chrome) => chrome.set_hover(rect, pos),
                _ => false,
            },
            _ => false,
        };
        if changed {
            self.scene.mark_dirty(id, PAINT_DIRTY);
        }
        changed
    }

    fn set_hover_node(&mut self, node: Option<NodeId>) {
        if self.hovered == node {
            return;
        }
        if let Some(old) = self.hovered.filter(|&id| self.scene.contains(id)) {
            self.apply_hover(old, None);
        }
        self.hovered = node;
        self.hover_pos = None;
        self.hover_tip = None;
        self.hover_since_ms = self.now_ms;
        self.hide_tooltip();
    }

    fn update_hover(&mut self, hit: Option<NodeId>, pos: Option<Pos>) {
        let target = hit.and_then(|hit| self.hover_target(hit));
        self.set_hover_node(target);
        let (Some(id), Some(pos)) = (target, pos) else { return };
        self.apply_hover(id, Some(pos));
        let tip = self.scene.node(id).tooltip.clone().or_else(|| match &self.scene.node(id).content {
            NodeContent::Chrome(chrome) => chrome.tooltip(self.scene.node(id).rect, pos),
            _ => None,
        });
        if tip != self.hover_tip || (self.hover_pos != Some(pos) && self.tooltip.is_some()) {
            self.hide_tooltip();
            self.hover_since_ms = self.now_ms;
        }
        self.hover_tip = tip;
        self.hover_pos = Some(pos);
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
                    let bg = buf.get(rect.x, rect.y).map_or(theme.surface(crate::tui::theme::Surface::Base), |c| c.bg);
                    buf.put_str(Pos { x: rect.x, y: rect.y }, s, theme.role(Role::Foreground), bg, 0, rect);
                }
                NodeContent::Axis(axis) => paint_gutters(scene, theme, axis, node, buf),
                NodeContent::Box => {}
            }
            for &child in node.children() {
                walk(scene, theme, focus, child, buf);
            }
        }
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        self.back.resize(self.size, blank);
        let focus = self.focus();
        walk(&self.scene, &self.theme, focus, self.scene.root(), &mut self.back);
        walk(&self.scene, &self.theme, focus, self.scene.overlay_root(), &mut self.back);
    }

    /// 🖨️ Solves layout if dirty, repaints, diffs against the last frame and emits the patch; an unchanged scene emits nothing.
    pub fn render(&mut self) -> AnsiPatch {
        self.sanitize();
        self.sync_chrome();
        if !self.dirty() {
            return AnsiPatch::default();
        }
        self.ensure_layout();
        self.paint();
        let mut patch = AnsiPatch::default();
        if self.full_redraw {
            let rows: Vec<DiffRun> = (0..self.size.height).map(|y| DiffRun { y, x: 0, len: self.size.width }).collect();
            emit_runs(&self.back, &rows, &mut patch);
            self.full_redraw = false;
        } else {
            let runs = diff(&self.front, &self.back);
            emit_runs(&self.back, &runs, &mut patch);
        }
        std::mem::swap(&mut self.front, &mut self.back);
        self.scene.clear_dirty(LAYOUT_DIRTY | PAINT_DIRTY);
        self.last_render_ms = Some(self.now_ms);
        patch
    }

    /// ♻️ Forces a full-frame repaint regardless of dirty state.
    pub fn render_full(&mut self) -> AnsiPatch {
        self.full_redraw = true;
        self.render()
    }
}

/// 📏️ The index of the gap between children `index` and `index + 1` that contains `pos`.
fn gutter_at(scene: &Scene, axis: &AxisState, rect: Rect, children: &[NodeId], pos: Pos) -> Option<usize> {
    if !rect.contains(pos) {
        return None;
    }
    children.windows(2).position(|pair| {
        let (a, b) = (scene.node(pair[0]).rect, scene.node(pair[1]).rect);
        if axis.horizontal {
            pos.x >= a.x + a.width && pos.x < b.x
        } else {
            pos.y >= a.y + a.height && pos.y < b.y
        }
    })
}

fn paint_gutters(scene: &Scene, theme: &Theme, axis: &AxisState, node: &Node, buf: &mut CellBuffer) {
    let Some(hover) = axis.hover else { return };
    let children = node.children();
    let (Some(&first), Some(&second)) = (children.get(hover), children.get(hover + 1)) else { return };
    let (a, b) = (scene.node(first).rect, scene.node(second).rect);
    let gutter = if axis.horizontal { Rect::new(a.x + a.width, node.rect.y, b.x.saturating_sub(a.x + a.width), node.rect.height) } else { Rect::new(node.rect.x, a.y + a.height, node.rect.width, b.y.saturating_sub(a.y + a.height)) };
    buf.fill_rect(gutter, Cell::blank(theme.role(Role::Foreground), theme.role(Role::HoverInteractive)));
}
