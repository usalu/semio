//! 🖱️ Theme-aware Semio cursor URLs for wgpu canvas hover parity with React.

use crate::input::{DragAxis, HitKind, HitTarget};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemioCursor {
    Default,
    Pointer,
    Selectable,
    Foldable,
    Grab,
    Grabbing,
    Text,
    EwResize,
    NsResize,
    NwseResize,
    NeswResize,
    Move,
    Crosshair,
    NotAllowed,
}

#[derive(Clone, Debug, Default)]
pub struct CursorDragState {
    pub tree_drag: bool,
    pub dock_drag: bool,
    pub pointer_drag_active: bool,
    pub pointer_drag_axis: Option<DragAxis>,
    pub pointer_drag_kind: Option<HitKind>,
}

pub fn resolve_semio_cursor<E>(hit: Option<&HitTarget<E>>, drag: CursorDragState) -> SemioCursor {
    if drag.tree_drag || drag.dock_drag {
        return SemioCursor::Grabbing;
    }
    if drag.pointer_drag_active {
        return cursor_for_active_drag(drag.pointer_drag_kind, drag.pointer_drag_axis);
    }
    let Some(hit) = hit else {
        return SemioCursor::Default;
    };
    if let Some(id) = hit.control_id.as_deref() {
        if id.contains(".chevron.") || id.starts_with("section.chevron.") {
            return SemioCursor::Foldable;
        }
    }
    if matches!(hit.kind, HitKind::PanelResize) {
        return SemioCursor::EwResize;
    }
    if hit.kind == HitKind::DockJoinCorner {
        return SemioCursor::Move;
    }
    if hit.kind == HitKind::DockSplit {
        return hit.drag_axis.map(axis_cursor).unwrap_or(SemioCursor::Default);
    }
    if hit.kind == HitKind::ScrollRegion {
        if let Some(axis) = hit.drag_axis {
            return axis_cursor(axis);
        }
    }
    match hit.kind {
        HitKind::Input => SemioCursor::Text,
        HitKind::Select => SemioCursor::Foldable,
        HitKind::Slider => SemioCursor::Grab,
        HitKind::Window => SemioCursor::Grab,
        HitKind::TreeItem => {
            if hit.drag_data.is_some() {
                SemioCursor::Grab
            } else {
                SemioCursor::Selectable
            }
        }
        HitKind::TreeDropTarget => SemioCursor::Move,
        HitKind::World3d => SemioCursor::Default,
        HitKind::Button | HitKind::Toggle | HitKind::PanelTab | HitKind::NavbarItem
        | HitKind::ContextMenu | HitKind::DropdownItem => SemioCursor::Selectable,
        HitKind::ScrollRegion | HitKind::PanelResize | HitKind::DockSplit | HitKind::DockJoinCorner => {
            SemioCursor::Default
        }
        HitKind::Generic => SemioCursor::Selectable,
    }
}

fn cursor_for_active_drag(kind: Option<HitKind>, axis: Option<DragAxis>) -> SemioCursor {
    match kind {
        Some(HitKind::Slider) => SemioCursor::Grabbing,
        Some(HitKind::PanelResize) => SemioCursor::EwResize,
        Some(HitKind::DockSplit) => axis.map(axis_cursor).unwrap_or(SemioCursor::Default),
        Some(HitKind::DockJoinCorner) => SemioCursor::Move,
        Some(HitKind::ScrollRegion) => axis.map(axis_cursor).unwrap_or(SemioCursor::Default),
        _ => axis.map(axis_cursor).unwrap_or(SemioCursor::Grabbing),
    }
}

fn axis_cursor(axis: DragAxis) -> SemioCursor {
    match axis {
        DragAxis::Horizontal => SemioCursor::EwResize,
        DragAxis::Vertical => SemioCursor::NsResize,
        DragAxis::Both => SemioCursor::NwseResize,
        DragAxis::Ring => SemioCursor::Crosshair,
    }
}

pub fn semio_cursor_css(cursor: SemioCursor, dark: bool) -> &'static str {
    match (cursor, dark) {
        (SemioCursor::Default, false) => "url(/asset/cursor/cursor.svg) 0 0, default",
        (SemioCursor::Default, true) => "url(/asset/cursor/cursor_dark.svg) 0 0, default",
        (SemioCursor::Pointer, false) => "url(/asset/cursor/cursor_pointer.svg) 0 0, pointer",
        (SemioCursor::Pointer, true) => {
            "url(/asset/cursor/cursor_pointer_dark_inkscape.svg) 0 0, pointer"
        }
        (SemioCursor::Selectable, false) => "url(/asset/cursor/cursor_selectable.svg) 0 0, pointer",
        (SemioCursor::Selectable, true) => {
            "url(/asset/cursor/cursor_selectable_dark.svg) 0 0, pointer"
        }
        (SemioCursor::Foldable, false) => "url(/asset/cursor/cursor_foldable.svg) 0 0, pointer",
        (SemioCursor::Foldable, true) => "url(/asset/cursor/cursor_foldable_dark.svg) 0 0, pointer",
        (SemioCursor::Grab, false) => "url(/asset/cursor/cursor_grab.svg) 0 0, grab",
        (SemioCursor::Grab, true) => "url(/asset/cursor/cursor_grab_dark.svg) 0 0, grab",
        (SemioCursor::Grabbing, _) => "url(/asset/cursor/cursor_grabbing.svg) 0 0, grabbing",
        (SemioCursor::Text, _) => "text",
        (SemioCursor::EwResize, false) => "url(/asset/cursor/cursor_ew-resize.svg) 16 2, ew-resize",
        (SemioCursor::EwResize, true) => {
            "url(/asset/cursor/cursor_ew-resize_dark.svg) 16 2, ew-resize"
        }
        (SemioCursor::NsResize, false) => "url(/asset/cursor/cursor_ns-resize.svg) 2 16, ns-resize",
        (SemioCursor::NsResize, true) => {
            "url(/asset/cursor/cursor_ns-resize_dark.svg) 2 16, ns-resize"
        }
        (SemioCursor::NwseResize, false) => {
            "url(/asset/cursor/cursor_nwse-resize.svg) 16 16, nwse-resize"
        }
        (SemioCursor::NwseResize, true) => {
            "url(/asset/cursor/cursor_nwse-resize_dark.svg) 16 16, nwse-resize"
        }
        (SemioCursor::NeswResize, false) => {
            "url(/asset/cursor/cursor_nesw-resize_dark.svg) 16 16, nesw-resize"
        }
        (SemioCursor::NeswResize, true) => {
            "url(/asset/cursor/cursor_nesw-resize_dark.svg) 16 16, nesw-resize"
        }
        (SemioCursor::Move, false) => "url(/asset/cursor/cursor_move_inkscape.svg) 16 16, move",
        (SemioCursor::Move, true) => "url(/asset/cursor/cursor_move_dark.svg) 16 16, move",
        (SemioCursor::Crosshair, false) => "url(/asset/cursor/cursor_crosshair.svg) 16 16, crosshair",
        (SemioCursor::Crosshair, true) => {
            "url(/asset/cursor/cursor_crosshair_dark.svg) 16 16, crosshair"
        }
        (SemioCursor::NotAllowed, false) => {
            "url(/asset/cursor/cursor_not-allowed.svg) 0 0, not-allowed"
        }
        (SemioCursor::NotAllowed, true) => {
            "url(/asset/cursor/cursor_not-allowed_dark.svg) 0 0, not-allowed"
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn apply_canvas_cursor(
    canvas: &web_sys::HtmlCanvasElement,
    cursor: SemioCursor,
    dark: bool,
    last: &mut Option<(SemioCursor, bool)>,
) {
    use wasm_bindgen::JsCast;
    let key = (cursor, dark);
    if last.as_ref() == Some(&key) {
        return;
    }
    *last = Some(key);
    let css = semio_cursor_css(cursor, dark);
    if let Some(element) = canvas.dyn_ref::<web_sys::HtmlElement>() {
        let _ = element.style().set_property("cursor", css);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Rect;
    use std::collections::HashMap;

    fn hit(kind: HitKind, axis: Option<DragAxis>) -> HitTarget<()> {
        HitTarget {
            rect: Rect::new(0.0, 0.0, 10.0, 10.0),
            event: None,
            control_id: None,
            kind,
            drag_axis: axis,
            drag_data: None,
        }
    }

    #[test]
    fn dock_split_horizontal_uses_ew_cursor() {
        let mut target = hit(HitKind::DockSplit, Some(DragAxis::Horizontal));
        target.control_id = Some("dock.split.0.0".into());
        let cursor = resolve_semio_cursor(Some(&target), CursorDragState::default());
        assert_eq!(cursor, SemioCursor::EwResize);
    }

    #[test]
    fn dock_join_corner_uses_move_cursor() {
        let target = hit(HitKind::DockJoinCorner, Some(DragAxis::Both));
        let cursor = resolve_semio_cursor(Some(&target), CursorDragState::default());
        assert_eq!(cursor, SemioCursor::Move);
    }

    #[test]
    fn dock_tab_uses_grab_cursor() {
        let cursor = resolve_semio_cursor(Some(&hit(HitKind::Window, None)), CursorDragState::default());
        assert_eq!(cursor, SemioCursor::Grab);
    }

    #[test]
    fn panel_resize_uses_ew_cursor() {
        let cursor = resolve_semio_cursor(
            Some(&hit(HitKind::PanelResize, Some(DragAxis::Horizontal))),
            CursorDragState::default(),
        );
        assert_eq!(cursor, SemioCursor::EwResize);
    }

    #[test]
    fn active_slider_drag_uses_grabbing() {
        let cursor = resolve_semio_cursor::<()>(
            None,
            CursorDragState {
                pointer_drag_active: true,
                pointer_drag_axis: Some(DragAxis::Horizontal),
                pointer_drag_kind: Some(HitKind::Slider),
                ..CursorDragState::default()
            },
        );
        assert_eq!(cursor, SemioCursor::Grabbing);
    }

    #[test]
    fn tree_draggable_label_uses_grab() {
        let mut target = hit(HitKind::TreeItem, Some(DragAxis::Both));
        target.drag_data = Some(HashMap::from([("id".into(), "x".into())]));
        let cursor = resolve_semio_cursor(Some(&target), CursorDragState::default());
        assert_eq!(cursor, SemioCursor::Grab);
    }

    #[test]
    fn dark_theme_cursor_urls_use_dark_assets() {
        assert!(semio_cursor_css(SemioCursor::Default, true).contains("cursor_dark.svg"));
        assert!(semio_cursor_css(SemioCursor::Selectable, false).contains("cursor_selectable.svg"));
    }
}
