//! 🖌️ Paint and gutters: the paint pass, the diffed and full renders and the gutter hit testing and painting. See [`super`].

use super::*;

impl Tui {
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
pub(super) fn gutter_at(scene: &Scene, axis: &AxisState, rect: Rect, children: &[NodeId], pos: Pos) -> Option<usize> {
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
