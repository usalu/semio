//! 🖱️ Declarative UI components (default) and retained-mode wgpu engine (feature "engine").

#[cfg(feature = "engine")]
pub mod chrome {
// #region chrome
//! 🎛 Bordered chrome primitives shared by widgets and shell renderers.

use crate::draw::DrawList;
use crate::draw::IconAtlas;
use crate::geometry::Rect;
use crate::text::FontAtlas;
use crate::theme::{Rgba, Theme};

pub const ICON_TINY: f32 = 14.0;

pub const TRANSPARENT: Rgba = Rgba::new(0.0, 0.0, 0.0, 0.0);

pub fn push_chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
    let hair = theme.stroke_hairline;
    push_chrome_border(draw, rect, hair, theme.border_normal, true, true, true, true);
}

pub fn push_chrome_border(
    draw: &mut DrawList,
    rect: Rect,
    stroke: f32,
    color: Rgba,
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
) {
    if top {
        draw.push_solid([rect.x, rect.y, rect.w, stroke], color);
    }
    if bottom {
        draw.push_solid([rect.x, rect.y + rect.h - stroke, rect.w, stroke], color);
    }
    if left {
        draw.push_solid([rect.x, rect.y, stroke, rect.h], color);
    }
    if right {
        draw.push_solid([rect.x + rect.w - stroke, rect.y, stroke, rect.h], color);
    }
}

pub fn push_window_cap_border(draw: &mut DrawList, rect: Rect, stroke: f32, color: Rgba) {
    push_chrome_border(draw, rect, stroke, color, true, true, false, true);
}

pub fn push_control_border(draw: &mut DrawList, rect: Rect, theme: &Theme, border: Rgba, bg: Rgba) {
    if bg.a > 0.0 {
        draw.push_solid([rect.x, rect.y, rect.w, rect.h], bg);
    }
    let hair = theme.stroke_hairline;
    draw.push_solid([rect.x, rect.y, rect.w, hair], border);
    draw.push_solid([rect.x, rect.y + rect.h - hair, rect.w, hair], border);
    draw.push_solid([rect.x, rect.y, hair, rect.h], border);
    draw.push_solid([rect.x + rect.w - hair, rect.y, hair, rect.h], border);
}

pub fn push_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32, color: Rgba) {
    if let Some(uv) = icons.icon_uv(icon_id) {
        draw.push_textured([x, y, size, size], uv, color);
    }
}

pub fn measure_action_item(
    atlas: &mut FontAtlas,
    theme: &Theme,
    icon: bool,
    label: Option<&str>,
) -> f32 {
    let icon_w = if icon {
        ICON_TINY + theme.gap_standard
    } else {
        0.0
    };
    let text_w = label
        .map(|value| atlas.measure_text(value, theme.font_size_small).0)
        .unwrap_or(0.0);
    theme.padding_standard * 2.0 + icon_w + text_w
}

pub fn chrome_item_bg(theme: &Theme, active: bool, hovered: bool) -> Rgba {
    if active {
        if hovered {
            theme.accent_hover
        } else {
            theme.selected
        }
    } else if hovered {
        theme.button_hover
    } else {
        TRANSPARENT
    }
}

pub fn chrome_item_text(theme: &Theme, active: bool, hovered: bool) -> Rgba {
    if active {
        theme.active_foreground
    } else if hovered {
        theme.border_emphasized
    } else {
        theme.text_element
    }
}

pub fn item_bg(theme: &Theme, pressed: bool, hovered: bool) -> Rgba {
    chrome_item_bg(theme, pressed, hovered)
}

pub fn item_text(theme: &Theme, pressed: bool, hovered: bool) -> Rgba {
    chrome_item_text(theme, pressed, hovered)
}
// #endregion chrome
}

#[cfg(feature = "engine")]
pub mod cursor {
// #region cursor
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

pub fn apply_window_cursor(
    window: &winit::window::Window,
    cursor: SemioCursor,
    dark: bool,
    last: &mut Option<(SemioCursor, bool)>,
) {
    let key = (cursor, dark);
    if last.as_ref() == Some(&key) {
        return;
    }
    *last = Some(key);
    let _ = dark;
    window.set_cursor(winit_cursor_icon(cursor));
}

fn winit_cursor_icon(cursor: SemioCursor) -> winit::window::CursorIcon {
    use winit::window::CursorIcon;
    match cursor {
        SemioCursor::Default => CursorIcon::Default,
        SemioCursor::Pointer | SemioCursor::Selectable | SemioCursor::Foldable => CursorIcon::Pointer,
        SemioCursor::Grab => CursorIcon::Grab,
        SemioCursor::Grabbing => CursorIcon::Grabbing,
        SemioCursor::Text => CursorIcon::Text,
        SemioCursor::EwResize => CursorIcon::EwResize,
        SemioCursor::NsResize => CursorIcon::NsResize,
        SemioCursor::NwseResize => CursorIcon::NwseResize,
        SemioCursor::NeswResize => CursorIcon::NeswResize,
        SemioCursor::Move => CursorIcon::Move,
        SemioCursor::Crosshair => CursorIcon::Crosshair,
        SemioCursor::NotAllowed => CursorIcon::NotAllowed,
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
// #endregion cursor
}

#[cfg(feature = "engine")]
pub mod draw {
// #region draw
//! 🖌️ Draw list and GPU pipeline for UI quads, vector geometry, and 3D scene passes.

use kernel_3d_scene::ScenePass3d;
use crate::shaders::{BLUR_DOWNSAMPLE_SHADER, GLASS_SHADER, SCENE_BLIT_SHADER, UI_SHADER, VECTOR_SHADER, WORLD3D_LINES_SHADER, WORLD3D_SHADER};
use crate::theme::{GlassTier, Rgba, Theme};
use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu::util::DeviceExt;

pub const KIND_SOLID: f32 = 3.0;
pub const KIND_ROUNDED: f32 = 1.0;
pub const KIND_GLYPH: f32 = 2.0;
pub const KIND_TEXTURED: f32 = 4.0;
pub const KIND_RASTER: f32 = 5.0;
pub const SCENE_MIP_LEVELS: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct BlurGlobals {
    src_mip: f32,
    _pad: [f32; 7],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GlassInstance {
    pub rect: [f32; 4],
    pub tint: [f32; 4],
    pub params: [f32; 4],
}

#[derive(Clone, Copy, Debug)]
pub struct GlassRegion {
    pub rect: [f32; 4],
    pub radius: f32,
    pub tint: Rgba,
    pub alpha: f32,
    pub blur_px: f32,
    pub saturate: f32,
}

pub struct SceneColorTarget {
    texture: wgpu::Texture,
    blur_scratch: wgpu::Texture,
    blur_scratch_mip_views: Vec<wgpu::TextureView>,
    sample_view: wgpu::TextureView,
    mip_views: Vec<wgpu::TextureView>,
    sampler: wgpu::Sampler,
    width: u32,
    height: u32,
}

impl SceneColorTarget {
    pub fn ensure(
        device: &wgpu::Device,
        target: &mut Option<Self>,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
    ) {
        let width = width.max(1);
        let height = height.max(1);
        if let Some(existing) = target {
            if existing.width == width && existing.height == height {
                return;
            }
        }
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("scene_color"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: SCENE_MIP_LEVELS,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[format],
        });
        let blur_scratch = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("scene_blur_scratch"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: SCENE_MIP_LEVELS,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[format],
        });
        let blur_scratch_mip_views = (0..SCENE_MIP_LEVELS)
            .map(|level| {
                blur_scratch.create_view(&wgpu::TextureViewDescriptor {
                    label: Some(&format!("scene_blur_scratch_mip_{level}")),
                    format: Some(format),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    base_mip_level: level,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let sample_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("scene_color_sample"),
            format: Some(format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            base_mip_level: 0,
            mip_level_count: Some(SCENE_MIP_LEVELS),
            ..Default::default()
        });
        let mip_views = (0..SCENE_MIP_LEVELS)
            .map(|level| {
                texture.create_view(&wgpu::TextureViewDescriptor {
                    label: Some(&format!("scene_color_mip_{level}")),
                    format: Some(format),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    base_mip_level: level,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("scene_color_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        *target = Some(Self {
            texture,
            blur_scratch,
            blur_scratch_mip_views,
            sample_view,
            mip_views,
            sampler,
            width,
            height,
        });
    }

    pub fn mip_view(&self, level: u32) -> &wgpu::TextureView {
        &self.mip_views[level as usize]
    }

    pub fn sample_view(&self) -> &wgpu::TextureView {
        &self.sample_view
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    pub fn blur_scratch_mip_view(&self, level: u32) -> &wgpu::TextureView {
        &self.blur_scratch_mip_views[level as usize]
    }

    fn mip_extent(&self, level: u32) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: (self.width >> level).max(1),
            height: (self.height >> level).max(1),
            depth_or_array_layers: 1,
        }
    }

    pub fn copy_mip_to_blur_scratch(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        src_mip: u32,
    ) {
        let extent = self.mip_extent(src_mip);
        encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: src_mip,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyTextureInfo {
                texture: &self.blur_scratch,
                mip_level: src_mip,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            extent,
        );
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UiGlobals {
    pub screen_size: [f32; 2],
    pub _pad: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UiInstance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub params: [f32; 4],
    pub uv_rect: [f32; 4],
}

impl UiInstance {
    pub fn solid(rect: [f32; 4], color: Rgba) -> Self {
        Self {
            rect,
            color: [color.r, color.g, color.b, color.a],
            params: [0.0, 0.0, KIND_SOLID, 0.0],
            uv_rect: [0.0, 0.0, 1.0, 1.0],
        }
    }

    pub fn rounded(rect: [f32; 4], color: Rgba, radius: f32, border: f32, border_color: Rgba) -> Self {
        Self {
            rect,
            color: [color.r, color.g, color.b, color.a],
            params: [radius, border, KIND_ROUNDED, border_color.a],
            uv_rect: [0.0, 0.0, 1.0, 1.0],
        }
    }

    pub fn glyph(rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) -> Self {
        Self {
            rect,
            color: [color.r, color.g, color.b, color.a],
            params: [0.0, 0.0, KIND_GLYPH, 0.0],
            uv_rect,
        }
    }

    pub fn textured(rect: [f32; 4], uv_rect: [f32; 4], color: Rgba) -> Self {
        Self {
            rect,
            color: [color.r, color.g, color.b, color.a],
            params: [0.0, 0.0, KIND_TEXTURED, 0.0],
            uv_rect,
        }
    }

    pub fn raster(rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) -> Self {
        Self {
            rect,
            color: [1.0, 1.0, 1.0, alpha],
            params: [0.0, 0.0, KIND_RASTER, 0.0],
            uv_rect,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct VectorVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Clone, Copy, Debug)]
pub struct ScissorRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl ScissorRect {
    pub fn from_rect(rect: crate::geometry::Rect, _screen_h: f32) -> Self {
        let x = rect.x.max(0.0) as u32;
        let y = rect.y.max(0.0) as u32;
        let w = rect.w.max(0.0) as u32;
        let h = rect.h.max(0.0) as u32;
        Self { x, y, w, h }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        let x0 = self.x.max(other.x);
        let y0 = self.y.max(other.y);
        let x1 = (self.x + self.w).min(other.x + other.w);
        let y1 = (self.y + self.h).min(other.y + other.h);
        Self {
            x: x0,
            y: y0,
            w: x1.saturating_sub(x0),
            h: y1.saturating_sub(y0),
        }
    }
}

pub struct DrawLayer {
    pub scissor: Option<ScissorRect>,
    pub foreground_of: Option<usize>,
    pub ui_instances: Vec<UiInstance>,
    pub raster_instances: Vec<(String, UiInstance)>,
    pub vector_vertices: Vec<VectorVertex>,
    pub overlay_ui_instances: Vec<UiInstance>,
    pub overlay_vector_vertices: Vec<VectorVertex>,
}

impl Default for DrawLayer {
    fn default() -> Self {
        Self {
            scissor: None,
            foreground_of: None,
            ui_instances: Vec::new(),
            raster_instances: Vec::new(),
            vector_vertices: Vec::new(),
            overlay_ui_instances: Vec::new(),
            overlay_vector_vertices: Vec::new(),
        }
    }
}

pub struct DrawList {
    pub scene_passes: Vec<ScenePass3d>,
    pub layers: Vec<DrawLayer>,
    pub glass_regions: Vec<GlassRegion>,
    scissor_stack: Vec<ScissorRect>,
    glass_content_stack: Vec<usize>,
    screen_h: f32,
}

impl Default for DrawList {
    fn default() -> Self {
        let mut list = Self {
            scene_passes: Vec::new(),
            layers: Vec::new(),
            glass_regions: Vec::new(),
            scissor_stack: Vec::new(),
            glass_content_stack: Vec::new(),
            screen_h: 720.0,
        };
        list.layers.push(DrawLayer::default());
        list
    }
}

impl DrawList {
    pub fn set_screen_height(&mut self, height: f32) {
        self.screen_h = height;
    }

    fn active_foreground_of(&self) -> Option<usize> {
        self.glass_content_stack.last().copied()
    }

    fn active_layer(&mut self) -> &mut DrawLayer {
        if self.layers.is_empty() {
            self.layers.push(DrawLayer::default());
        }
        self.layers.last_mut().expect("layer")
    }

    pub fn clear(&mut self) {
        self.scene_passes.clear();
        self.layers.clear();
        self.layers.push(DrawLayer::default());
        self.glass_regions.clear();
        self.scissor_stack.clear();
        self.glass_content_stack.clear();
    }

    pub fn push_scissor(&mut self, rect: crate::geometry::Rect) {
        let mut scissor = ScissorRect::from_rect(rect, self.screen_h);
        if let Some(parent) = self.scissor_stack.last() {
            scissor = parent.intersect(&scissor);
        }
        self.scissor_stack.push(scissor);
        self.layers.push(DrawLayer {
            scissor: Some(scissor),
            foreground_of: self.active_foreground_of(),
            ..DrawLayer::default()
        });
    }

    pub fn pop_scissor(&mut self) {
        self.scissor_stack.pop();
        let parent = self.scissor_stack.last().cloned();
        self.layers.push(DrawLayer {
            scissor: parent,
            foreground_of: self.active_foreground_of(),
            ..DrawLayer::default()
        });
    }

    pub fn push_scene_pass(&mut self, mut pass: ScenePass3d) {
        if self.layers.is_empty() {
            self.layers.push(DrawLayer::default());
        }
        let layer_index = self.layers.len() - 1;
        let layer = &self.layers[layer_index];
        pass.layer_index = layer_index;
        pass.ui_watermark = layer.ui_instances.len();
        pass.vector_watermark = layer.vector_vertices.len();
        self.scene_passes.push(pass);
    }

    pub fn push_solid(&mut self, rect: [f32; 4], color: Rgba) {
        self.active_layer()
            .ui_instances
            .push(UiInstance::solid(rect, color));
    }

    pub fn push_rounded(&mut self, rect: [f32; 4], color: Rgba, radius: f32) {
        self.active_layer()
            .ui_instances
            .push(UiInstance::rounded(rect, color, radius, 0.0, color));
    }

    pub fn push_glass(&mut self, rect: [f32; 4], radius: f32, tier: GlassTier, theme: &Theme) -> usize {
        let style = theme.glass(tier);
        let index = self.glass_regions.len();
        self.glass_regions.push(GlassRegion {
            rect,
            radius,
            tint: style.tint,
            alpha: style.alpha,
            blur_px: style.blur_px,
            saturate: style.saturate,
        });
        index
    }

    pub fn begin_glass_content(&mut self, region: usize) {
        self.glass_content_stack.push(region);
        self.layers.push(DrawLayer {
            scissor: None,
            foreground_of: Some(region),
            ..DrawLayer::default()
        });
    }

    pub fn end_glass_content(&mut self) {
        self.glass_content_stack.pop();
        self.layers.push(DrawLayer {
            scissor: self.scissor_stack.last().cloned(),
            foreground_of: self.active_foreground_of(),
            ..DrawLayer::default()
        });
    }

    pub fn push_glyph(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
        self.active_layer()
            .ui_instances
            .push(UiInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_glyph_overlay(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
        self.active_layer()
            .overlay_ui_instances
            .push(UiInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_solid_overlay(&mut self, rect: [f32; 4], color: Rgba) {
        self.active_layer()
            .overlay_ui_instances
            .push(UiInstance::solid(rect, color));
    }

    pub fn push_textured(&mut self, rect: [f32; 4], uv_rect: [f32; 4], color: Rgba) {
        self.active_layer()
            .ui_instances
            .push(UiInstance::textured(rect, uv_rect, color));
    }

    pub fn push_raster_quad(&mut self, key: &str, rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) {
        self.active_layer().raster_instances.push((
            key.to_string(),
            UiInstance::raster(rect, uv_rect, alpha),
        ));
    }

    pub fn push_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt().max(0.001);
        let nx = -dy / len * width * 0.5;
        let ny = dx / len * width * 0.5;
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_layer();
        layer.vector_vertices.extend_from_slice(&[
            VectorVertex { position: [x0 + nx, y0 + ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x1 - nx, y1 - ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
        ]);
    }

    pub fn push_line_overlay(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt().max(0.001);
        let nx = -dy / len * width * 0.5;
        let ny = dx / len * width * 0.5;
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_layer();
        layer.overlay_vector_vertices.extend_from_slice(&[
            VectorVertex { position: [x0 + nx, y0 + ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x1 - nx, y1 - ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
        ]);
    }

    pub fn push_triangle_fan(&mut self, points: &[[f32; 2]], color: Rgba) {
        if points.len() < 3 {
            return;
        }
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_layer();
        for tri in 1..points.len() - 1 {
            layer.vector_vertices.push(VectorVertex { position: points[0], color: c });
            layer.vector_vertices
                .push(VectorVertex { position: points[tri], color: c });
            layer.vector_vertices
                .push(VectorVertex { position: points[tri + 1], color: c });
        }
    }

    pub fn push_triangle_fan_overlay(&mut self, points: &[[f32; 2]], color: Rgba) {
        if points.len() < 3 {
            return;
        }
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_layer();
        for tri in 1..points.len() - 1 {
            layer
                .overlay_vector_vertices
                .push(VectorVertex { position: points[0], color: c });
            layer
                .overlay_vector_vertices
                .push(VectorVertex { position: points[tri], color: c });
            layer
                .overlay_vector_vertices
                .push(VectorVertex { position: points[tri + 1], color: c });
        }
    }

    pub fn push_dashed_line(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: Rgba,
        width: f32,
        dash: f32,
        gap: f32,
    ) {
        for (sx0, sy0, sx1, sy1) in dashed_line_segments(x0, y0, x1, y1, dash, gap) {
            self.push_line(sx0, sy0, sx1, sy1, color, width);
        }
    }

    pub fn push_dashed_line_overlay(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: Rgba,
        width: f32,
        dash: f32,
        gap: f32,
    ) {
        for (sx0, sy0, sx1, sy1) in dashed_line_segments(x0, y0, x1, y1, dash, gap) {
            self.push_line_overlay(sx0, sy0, sx1, sy1, color, width);
        }
    }
}

pub const SELECTION_MARQUEE_STROKE_WIDTH: f32 = 1.5;
pub const SELECTION_MARQUEE_FILL_ALPHA: f32 = 0.12;
pub const SELECTION_MARQUEE_DASH_LEN: f32 = 5.0;
pub const SELECTION_MARQUEE_DASH_GAP: f32 = 4.0;

pub fn selection_marquee_stroke(theme: &Theme) -> Rgba {
    theme.selected
}

pub fn selection_marquee_fill(theme: &Theme) -> Rgba {
    theme.selected.with_alpha(SELECTION_MARQUEE_FILL_ALPHA)
}

fn dashed_line_segments(
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    dash: f32,
    gap: f32,
) -> Vec<(f32, f32, f32, f32)> {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let ux = dx / len;
    let uy = dy / len;
    let mut traveled = 0.0f32;
    let mut drawing = true;
    let mut segments = Vec::new();
    while traveled < len {
        let segment = if drawing { dash } else { gap };
        let next = (traveled + segment).min(len);
        if drawing {
            segments.push((
                x0 + ux * traveled,
                y0 + uy * traveled,
                x0 + ux * next,
                y0 + uy * next,
            ));
        }
        traveled = next;
        drawing = !drawing;
    }
    segments
}

#[cfg(test)]
mod selection_marquee_tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn dashed_line_segments_emit_dashes_along_segment() {
        let segments = dashed_line_segments(0.0, 0.0, 20.0, 0.0, 5.0, 4.0);
        assert!(!segments.is_empty());
        let span: f32 = segments.iter().map(|(x0, _, x1, _)| x1 - x0).sum();
        assert!(span > 0.0 && span <= 20.0);
    }

    #[test]
    fn selection_marquee_colors_use_active_token_only() {
        let theme = Theme::default();
        assert_eq!(selection_marquee_stroke(&theme), theme.selected);
        assert_eq!(selection_marquee_fill(&theme).a, SELECTION_MARQUEE_FILL_ALPHA);
    }
}

fn push_marquee_segment(
    draw: &mut DrawList,
    overlay: bool,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    stroke: Rgba,
    dashed: bool,
) {
    if dashed {
        if overlay {
            draw.push_dashed_line_overlay(
                x0,
                y0,
                x1,
                y1,
                stroke,
                SELECTION_MARQUEE_STROKE_WIDTH,
                SELECTION_MARQUEE_DASH_LEN,
                SELECTION_MARQUEE_DASH_GAP,
            );
        } else {
            draw.push_dashed_line(
                x0,
                y0,
                x1,
                y1,
                stroke,
                SELECTION_MARQUEE_STROKE_WIDTH,
                SELECTION_MARQUEE_DASH_LEN,
                SELECTION_MARQUEE_DASH_GAP,
            );
        }
    } else if overlay {
        draw.push_line_overlay(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH);
    } else {
        draw.push_line(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH);
    }
}

pub fn paint_selection_marquee(
    draw: &mut DrawList,
    theme: &Theme,
    crossing: bool,
    lasso: bool,
    points: &[[f32; 2]],
    overlay: bool,
) {
    if points.len() < 2 {
        return;
    }
    let stroke = selection_marquee_stroke(theme);
    let fill = selection_marquee_fill(theme);
    let dashed = crossing;
    if lasso {
        if points.len() >= 3 {
            if overlay {
                draw.push_triangle_fan_overlay(points, fill);
            } else {
                draw.push_triangle_fan(points, fill);
            }
        }
        for window in points.windows(2) {
            push_marquee_segment(
                draw,
                overlay,
                window[0][0],
                window[0][1],
                window[1][0],
                window[1][1],
                stroke,
                dashed,
            );
        }
        let first = points[0];
        let last = points[points.len() - 1];
        push_marquee_segment(
            draw,
            overlay,
            last[0],
            last[1],
            first[0],
            first[1],
            stroke,
            dashed,
        );
        return;
    }
    let start = points[0];
    let end = points[points.len() - 1];
    let rx = start[0].min(end[0]);
    let ry = start[1].min(end[1]);
    let rw = (end[0] - start[0]).abs();
    let rh = (end[1] - start[1]).abs();
    if overlay {
        draw.push_solid_overlay([rx, ry, rw, rh], fill);
    } else {
        draw.push_solid([rx, ry, rw, rh], fill);
    }
    push_marquee_segment(draw, overlay, start[0], start[1], end[0], start[1], stroke, dashed);
    push_marquee_segment(draw, overlay, end[0], start[1], end[0], end[1], stroke, dashed);
    push_marquee_segment(draw, overlay, end[0], end[1], start[0], end[1], stroke, dashed);
    push_marquee_segment(draw, overlay, start[0], end[1], start[0], start[1], stroke, dashed);
}

pub fn ear_clip_polygon(points: &[[f32; 2]]) -> Vec<[f32; 2]> {
    if points.len() < 3 {
        return Vec::new();
    }
    let mut indices: Vec<usize> = (0..points.len()).collect();
    let mut triangles = Vec::new();
    let mut guard = 0usize;
    while indices.len() > 3 && guard < points.len() * points.len() {
        guard += 1;
        let mut ear_found = false;
        for i in 0..indices.len() {
            let prev = indices[(i + indices.len() - 1) % indices.len()];
            let curr = indices[i];
            let next = indices[(i + 1) % indices.len()];
            let a = points[prev];
            let b = points[curr];
            let c = points[next];
            let cross = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            if cross <= 0.0 {
                continue;
            }
            let mut contains = false;
            for &idx in &indices {
                if idx == prev || idx == curr || idx == next {
                    continue;
                }
                let p = points[idx];
                if point_in_triangle(p, a, b, c) {
                    contains = true;
                    break;
                }
            }
            if contains {
                continue;
            }
            triangles.push(a);
            triangles.push(b);
            triangles.push(c);
            indices.remove(i);
            ear_found = true;
            break;
        }
        if !ear_found {
            break;
        }
    }
    if indices.len() == 3 {
        triangles.push(points[indices[0]]);
        triangles.push(points[indices[1]]);
        triangles.push(points[indices[2]]);
    }
    triangles
}

fn point_in_triangle(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dGlobals {
    pub view_proj: [f32; 16],
    pub light_dir: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dGpuInstance {
    pub model0: [f32; 4],
    pub model1: [f32; 4],
    pub model2: [f32; 4],
    pub model3: [f32; 4],
    pub color: [f32; 4],
    pub flags: [f32; 4],
}

impl World3dGpuInstance {
    pub fn from_instance(model: [f32; 16], color: [f32; 4], selected: bool, hovered: bool) -> Self {
        Self {
            model0: [model[0], model[1], model[2], model[3]],
            model1: [model[4], model[5], model[6], model[7]],
            model2: [model[8], model[9], model[10], model[11]],
            model3: [model[12], model[13], model[14], model[15]],
            color,
            flags: [
                if selected { 1.0 } else { 0.0 },
                if hovered { 1.0 } else { 0.0 },
                0.0,
                0.0,
            ],
        }
    }
}

pub struct GpuMeshBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

pub struct MeshGpuStore {
    meshes: std::collections::HashMap<String, GpuMeshBuffers>,
}

pub fn mesh_content_version(positions: &[f32], normals: &[f32], indices: &[u32]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for value in positions.iter().chain(normals.iter()) {
        hash ^= value.to_bits() as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for value in indices {
        hash ^= *value as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

impl Default for MeshGpuStore {
    fn default() -> Self {
        Self {
            meshes: std::collections::HashMap::new(),
        }
    }
}

impl MeshGpuStore {
    pub fn get(&self, key: &str) -> Option<&GpuMeshBuffers> {
        self.meshes.get(key)
    }

    pub fn lookup_key(mesh_key: &str, version: u64) -> String {
        format!("{mesh_key}:{version}")
    }

    pub fn get_versioned(&self, mesh_key: &str, version: u64) -> Option<&GpuMeshBuffers> {
        self.get(&Self::lookup_key(mesh_key, version))
    }

    pub fn ensure_mesh(
        &mut self,
        device: &wgpu::Device,
        key: &str,
        version: u64,
        positions: &[f32],
        normals: &[f32],
        indices: &[u32],
    ) {
        let store_key = format!("{key}:{version}");
        if self.meshes.contains_key(&store_key) {
            return;
        }
        let prefix = format!("{key}:");
        self.meshes.retain(|existing, _| !existing.starts_with(&prefix) || existing == &store_key);
        let mut vertices = Vec::with_capacity(positions.len() / 3);
        for index in 0..positions.len() / 3 {
            vertices.push(World3dVertex {
                position: [
                    positions[index * 3],
                    positions[index * 3 + 1],
                    positions[index * 3 + 2],
                ],
                normal: [
                    normals.get(index * 3).copied().unwrap_or(0.0),
                    normals.get(index * 3 + 1).copied().unwrap_or(1.0),
                    normals.get(index * 3 + 2).copied().unwrap_or(0.0),
                ],
            });
        }
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("world3d_vertices"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("world3d_indices"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        self.meshes.insert(
            store_key,
            GpuMeshBuffers {
                vertex_buffer,
                index_buffer,
                index_count: indices.len() as u32,
            },
        );
    }

    pub fn evict_mesh(&mut self, key: &str) {
        let prefix = format!("{key}:");
        self.meshes.retain(|existing, _| !existing.starts_with(&prefix));
    }
}

pub const WORLD_GLOBALS_SLOT_SIZE: u64 = 256;

pub struct GrowBuffer {
    buffer: Option<wgpu::Buffer>,
    capacity: usize,
}

impl Default for GrowBuffer {
    fn default() -> Self {
        Self {
            buffer: None,
            capacity: 0,
        }
    }
}

impl GrowBuffer {
    pub fn slice(&self) -> Option<wgpu::BufferSlice<'_>> {
        self.buffer.as_ref().map(|buffer| buffer.slice(..))
    }

    pub fn upload<T: Pod>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[T],
        usage: wgpu::BufferUsages,
        label: &str,
    ) -> Option<wgpu::BufferSlice<'_>> {
        if data.is_empty() {
            return None;
        }
        let bytes = bytemuck::cast_slice(data);
        let required = bytes.len();
        if self.capacity < required {
            self.capacity = required.next_power_of_two().max(256);
            self.buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: self.capacity as u64,
                usage,
                mapped_at_creation: false,
            }));
        }
        let buffer = self.buffer.as_ref()?;
        queue.write_buffer(buffer, 0, bytes);
        Some(buffer.slice(..))
    }
}

pub struct FrameBuffers {
    pub world_instances: GrowBuffer,
    pub world_lines: GrowBuffer,
    pub ui_instances: GrowBuffer,
    pub vector_vertices: GrowBuffer,
    pub glass_instances: GrowBuffer,
}

impl Default for FrameBuffers {
    fn default() -> Self {
        Self {
            world_instances: GrowBuffer::default(),
            world_lines: GrowBuffer::default(),
            ui_instances: GrowBuffer::default(),
            vector_vertices: GrowBuffer::default(),
            glass_instances: GrowBuffer::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct WorldLineGpuVertex {
    position: [f32; 3],
    color: [f32; 4],
}

struct WorldDrawRange {
    mesh_key: String,
    mesh_version: u64,
    instance_offset: u32,
    instance_count: u32,
}

struct PreparedWorldPass {
    globals: World3dGlobals,
    viewport: [f32; 4],
    draws: Vec<WorldDrawRange>,
    translucent_draws: Vec<WorldDrawRange>,
    line_start: u32,
    line_count: u32,
}

struct WorldGlobalsRing {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    slot_stride: u32,
    capacity_slots: u32,
}

impl WorldGlobalsRing {
    fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, initial_slots: u32) -> Self {
        let slot_stride = WORLD_GLOBALS_SLOT_SIZE as u32;
        let capacity_slots = initial_slots.max(1);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("world3d_globals_ring"),
            size: slot_stride as u64 * capacity_slots as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("world3d_bind_group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: std::num::NonZeroU64::new(mem::size_of::<World3dGlobals>() as u64),
                }),
            }],
        });
        Self {
            buffer,
            bind_group,
            slot_stride,
            capacity_slots,
        }
    }

    fn ensure_slots(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, slots: u32) {
        if slots <= self.capacity_slots {
            return;
        }
        self.capacity_slots = slots.next_power_of_two().max(4);
        self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("world3d_globals_ring"),
            size: self.slot_stride as u64 * self.capacity_slots as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("world3d_bind_group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &self.buffer,
                    offset: 0,
                    size: std::num::NonZeroU64::new(mem::size_of::<World3dGlobals>() as u64),
                }),
            }],
        });
    }

    fn write_passes(&self, queue: &wgpu::Queue, passes: &[World3dGlobals]) {
        for (index, globals) in passes.iter().enumerate() {
            let offset = (index as u64) * self.slot_stride as u64;
            queue.write_buffer(&self.buffer, offset, bytemuck::bytes_of(globals));
        }
    }

    fn offset_for_slot(&self, slot: u32) -> u32 {
        slot * self.slot_stride
    }
}

fn sign(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) -> f32 {
    (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
}

pub const ICON_ATLAS_TEXTURE_SIZE: u32 = 2048;

pub struct IconAtlas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    entries: std::collections::HashMap<String, [f32; 4]>,
}

impl Default for IconAtlas {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            pixels: vec![0, 0, 0, 0],
            entries: std::collections::HashMap::new(),
        }
    }
}

impl IconAtlas {
    pub fn from_packed(width: u32, height: u32, pixels: Vec<u8>, entries: Vec<(String, [f32; 4])>) -> Self {
        Self {
            width,
            height,
            pixels,
            entries: entries.into_iter().collect(),
        }
    }

    pub fn icon_uv(&self, icon_id: &str) -> Option<[f32; 4]> {
        self.entries.get(icon_id).copied()
    }
}

pub struct RasterTexture {
    pub texture: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
}

pub struct RasterTextureStore {
    textures: std::collections::HashMap<String, RasterTexture>,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

impl RasterTextureStore {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("raster_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        Self {
            textures: std::collections::HashMap::new(),
            layout: layout.clone(),
            sampler,
        }
    }

    pub fn ensure_raster(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        globals_buffer: &wgpu::Buffer,
        glyph_view: &wgpu::TextureView,
        glyph_sampler: &wgpu::Sampler,
        icon_view: &wgpu::TextureView,
        icon_sampler: &wgpu::Sampler,
        key: &str,
        pixels: &[u8],
        width: u32,
        height: u32,
    ) {
        if let Some(existing) = self.textures.get(key) {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &existing.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                pixels,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(width * 4),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            );
            return;
        }
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("raster_texture"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("raster_texture_bind_group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(glyph_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(glyph_sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        self.textures.insert(
            key.to_string(),
            RasterTexture {
                texture,
                bind_group,
                width,
                height,
            },
        );
    }

    pub fn get(&self, key: &str) -> Option<&RasterTexture> {
        self.textures.get(key)
    }

    pub fn replace_gpu_bind_group(
        &mut self,
        device: &wgpu::Device,
        globals_buffer: &wgpu::Buffer,
        glyph_view: &wgpu::TextureView,
        glyph_sampler: &wgpu::Sampler,
        key: &str,
        raster_view: &wgpu::TextureView,
        texture: wgpu::Texture,
        width: u32,
        height: u32,
    ) {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("raster_bind_group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(glyph_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(glyph_sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(raster_view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        self.textures.insert(
            key.to_string(),
            RasterTexture { texture, bind_group, width, height },
        );
    }
}

pub(crate) struct UiPipelines {
    ui_pipeline: wgpu::RenderPipeline,
    vector_pipeline: wgpu::RenderPipeline,
    world_pipeline: wgpu::RenderPipeline,
    world_pipeline_translucent: wgpu::RenderPipeline,
    world_line_pipeline: wgpu::RenderPipeline,
    blur_downsample_pipeline: wgpu::RenderPipeline,
    scene_blit_pipeline: wgpu::RenderPipeline,
    glass_pipeline: wgpu::RenderPipeline,
    quad_vertex_buffer: wgpu::Buffer,
    globals_buffer: wgpu::Buffer,
    blur_globals_buffer: wgpu::Buffer,
    world_globals_ring: WorldGlobalsRing,
    world_bind_group_layout: wgpu::BindGroupLayout,
    blur_bind_group_layout: wgpu::BindGroupLayout,
    scene_bind_group_layout: wgpu::BindGroupLayout,
    glyph_texture: wgpu::Texture,
    glyph_sampler: wgpu::Sampler,
    icon_texture: wgpu::Texture,
    icon_sampler: wgpu::Sampler,
    glyph_bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

struct LayerBatch {
    layer_index: usize,
    scissor: Option<ScissorRect>,
    ui_start: u32,
    ui_count: u32,
    vec_start: u32,
    vec_count: u32,
}

enum LayerBatchFilter {
    Backdrop,
    Foreground,
}

impl Copy for LayerBatchFilter {}

impl Clone for LayerBatchFilter {
    fn clone(&self) -> Self {
        *self
    }
}

fn layer_matches_filter(layer: &DrawLayer, filter: LayerBatchFilter) -> bool {
    match filter {
        LayerBatchFilter::Backdrop => layer.foreground_of.is_none(),
        LayerBatchFilter::Foreground => layer.foreground_of.is_some(),
    }
}

fn build_layer_batches(
    draw: &DrawList,
    filter: LayerBatchFilter,
) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
    let mut all_ui = Vec::new();
    let mut all_vec = Vec::new();
    let mut batches = Vec::new();
    let scene_layers: std::collections::HashSet<usize> = draw
        .scene_passes
        .iter()
        .filter(|pass| layer_matches_filter(&draw.layers[pass.layer_index], filter))
        .map(|pass| pass.layer_index)
        .collect();
    for (layer_index, layer) in draw.layers.iter().enumerate() {
        if !layer_matches_filter(layer, filter) {
            continue;
        }
        if layer.ui_instances.is_empty()
            && layer.vector_vertices.is_empty()
            && !scene_layers.contains(&layer_index)
        {
            continue;
        }
        let ui_start = all_ui.len() as u32;
        all_ui.extend_from_slice(&layer.ui_instances);
        let vec_start = all_vec.len() as u32;
        all_vec.extend_from_slice(&layer.vector_vertices);
        batches.push(LayerBatch {
            layer_index,
            scissor: layer.scissor,
            ui_start,
            ui_count: layer.ui_instances.len() as u32,
            vec_start,
            vec_count: layer.vector_vertices.len() as u32,
        });
    }
    (all_ui, all_vec, batches)
}

fn build_overlay_layer_batches(
    draw: &DrawList,
    filter: LayerBatchFilter,
) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
    let mut all_ui = Vec::new();
    let mut all_vec = Vec::new();
    let mut batches = Vec::new();
    for (layer_index, layer) in draw.layers.iter().enumerate() {
        if !layer_matches_filter(layer, filter) {
            continue;
        }
        if layer.overlay_ui_instances.is_empty() && layer.overlay_vector_vertices.is_empty() {
            continue;
        }
        let ui_start = all_ui.len() as u32;
        all_ui.extend_from_slice(&layer.overlay_ui_instances);
        let vec_start = all_vec.len() as u32;
        all_vec.extend_from_slice(&layer.overlay_vector_vertices);
        batches.push(LayerBatch {
            layer_index,
            scissor: layer.scissor,
            ui_start,
            ui_count: layer.overlay_ui_instances.len() as u32,
            vec_start,
            vec_count: layer.overlay_vector_vertices.len() as u32,
        });
    }
    (all_ui, all_vec, batches)
}

fn set_pass_scissor(pass: &mut wgpu::RenderPass<'_>, scissor: Option<ScissorRect>, width: f32, height: f32) {
    if let Some(scissor) = scissor {
        pass.set_scissor_rect(scissor.x, scissor.y, scissor.w, scissor.h);
    } else {
        pass.set_scissor_rect(0, 0, width as u32, height as u32);
    }
}

impl UiPipelines {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let globals_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ui_globals_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let glyph_bind_group_layout = globals_bind_group_layout.clone();
        let _ = glyph_bind_group_layout;

        let ui_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ui_shader"),
            source: wgpu::ShaderSource::Wgsl(UI_SHADER.into()),
        });
        let vector_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("vector_shader"),
            source: wgpu::ShaderSource::Wgsl(VECTOR_SHADER.into()),
        });
        let world_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("world3d_shader"),
            source: wgpu::ShaderSource::Wgsl(WORLD3D_SHADER.into()),
        });
        let world_lines_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("world3d_lines_shader"),
            source: wgpu::ShaderSource::Wgsl(WORLD3D_LINES_SHADER.into()),
        });

        let depth_state = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        let overlay_depth_state = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::Always,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });

        let quad_vertices: &[f32] = &[
            0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0,
        ];
        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ui_quad_vertices"),
            contents: bytemuck::cast_slice(quad_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ui_globals"),
            contents: bytemuck::bytes_of(&UiGlobals {
                screen_size: [1.0, 1.0],
                _pad: [0.0, 0.0],
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let glyph_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glyph_atlas"),
            size: wgpu::Extent3d { width: ICON_ATLAS_TEXTURE_SIZE, height: ICON_ATLAS_TEXTURE_SIZE, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let glyph_view = glyph_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let glyph_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("glyph_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let icon_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("icon_atlas"),
            size: wgpu::Extent3d { width: ICON_ATLAS_TEXTURE_SIZE, height: ICON_ATLAS_TEXTURE_SIZE, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let icon_view = icon_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let icon_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("icon_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let glyph_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ui_bind_group"),
            layout: &globals_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&glyph_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&glyph_sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&icon_view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&icon_sampler) },
            ],
        });
        let ui_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ui_pipeline_layout"),
            bind_group_layouts: &[&globals_bind_group_layout],
            push_constant_ranges: &[],
        });
        let ui_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ui_pipeline"),
            layout: Some(&ui_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &ui_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 8,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<UiInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 48, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &ui_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: overlay_depth_state.clone(),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let vector_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("vector_pipeline_layout"),
            bind_group_layouts: &[&globals_bind_group_layout],
            push_constant_ranges: &[],
        });
        let vector_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("vector_pipeline"),
            layout: Some(&vector_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vector_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<VectorVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 },
                        wgpu::VertexAttribute { offset: 8, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &vector_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: overlay_depth_state,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let world_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("world3d_globals_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: std::num::NonZeroU64::new(mem::size_of::<World3dGlobals>() as u64),
                },
                count: None,
            }],
        });

        let world_globals_ring = WorldGlobalsRing::new(device, &world_bind_group_layout, 8);

        let world_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("world3d_pipeline_layout"),
            bind_group_layouts: &[&world_bind_group_layout],
            push_constant_ranges: &[],
        });
        let world_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("world3d_pipeline"),
            layout: Some(&world_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &world_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<World3dVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: 12,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<World3dGpuInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 48, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 64, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 80, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: depth_state.clone(),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        let translucent_depth_state = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState {
                constant: -2,
                slope_scale: -1.0,
                clamp: 0.0,
            },
        });
        let world_line_depth_state = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        let world_pipeline_translucent = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("world3d_pipeline_translucent"),
            layout: Some(&world_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &world_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<World3dVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: 12,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<World3dGpuInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 48, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 64, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 80, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: translucent_depth_state.clone(),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        let world_line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("world3d_line_pipeline"),
            layout: Some(&world_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &world_lines_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<WorldLineGpuVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: 12,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &world_lines_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: world_line_depth_state.clone(),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let blur_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("blur_downsample_shader"),
            source: wgpu::ShaderSource::Wgsl(BLUR_DOWNSAMPLE_SHADER.into()),
        });
        let scene_blit_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("scene_blit_shader"),
            source: wgpu::ShaderSource::Wgsl(SCENE_BLIT_SHADER.into()),
        });
        let glass_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("glass_shader"),
            source: wgpu::ShaderSource::Wgsl(GLASS_SHADER.into()),
        });

        let blur_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("blur_downsample_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: std::num::NonZeroU64::new(mem::size_of::<BlurGlobals>() as u64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let scene_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("scene_sample_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let blur_globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("blur_globals"),
            contents: bytemuck::bytes_of(&BlurGlobals {
                src_mip: 0.0,
                _pad: [0.0; 7],
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let blur_downsample_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("blur_downsample_pipeline_layout"),
            bind_group_layouts: &[&blur_bind_group_layout],
            push_constant_ranges: &[],
        });
        let blur_downsample_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blur_downsample_pipeline"),
            layout: Some(&blur_downsample_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &blur_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &blur_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let scene_blit_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("scene_blit_pipeline_layout"),
            bind_group_layouts: &[&scene_bind_group_layout],
            push_constant_ranges: &[],
        });
        let scene_blit_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("scene_blit_pipeline"),
            layout: Some(&scene_blit_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &scene_blit_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &scene_blit_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let glass_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("glass_pipeline_layout"),
            bind_group_layouts: &[&globals_bind_group_layout, &scene_bind_group_layout],
            push_constant_ranges: &[],
        });
        let glass_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("glass_pipeline"),
            layout: Some(&glass_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &glass_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 8,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<GlassInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &glass_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let _ = queue;
        Self {
            ui_pipeline,
            vector_pipeline,
            world_pipeline,
            world_pipeline_translucent,
            world_line_pipeline,
            blur_downsample_pipeline,
            scene_blit_pipeline,
            glass_pipeline,
            quad_vertex_buffer,
            globals_buffer,
            blur_globals_buffer,
            world_globals_ring,
            world_bind_group_layout,
            blur_bind_group_layout,
            scene_bind_group_layout,
            glyph_texture,
            glyph_sampler,
            icon_texture,
            icon_sampler,
            glyph_bind_group,
            bind_group_layout: globals_bind_group_layout,
        }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn globals_buffer(&self) -> &wgpu::Buffer {
        &self.globals_buffer
    }

    pub fn glyph_view(&self) -> wgpu::TextureView {
        self.glyph_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn glyph_sampler(&self) -> &wgpu::Sampler {
        &self.glyph_sampler
    }

    pub fn icon_view(&self) -> wgpu::TextureView {
        self.icon_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn icon_sampler(&self) -> &wgpu::Sampler {
        &self.icon_sampler
    }

    pub fn depth_format(&self) -> wgpu::TextureFormat {
        wgpu::TextureFormat::Depth24Plus
    }

    fn prepare_world_passes(
        draw: &DrawList,
        filter: LayerBatchFilter,
    ) -> (
        Vec<PreparedWorldPass>,
        Vec<World3dGpuInstance>,
        Vec<WorldLineGpuVertex>,
        Vec<Option<usize>>,
    ) {
        let mut prepared = Vec::new();
        let mut all_instances = Vec::new();
        let mut all_lines = Vec::new();
        let mut pass_index_map = vec![None; draw.scene_passes.len()];
        for (source_index, scene) in draw.scene_passes.iter().enumerate() {
            if !layer_matches_filter(&draw.layers[scene.layer_index], filter) {
                continue;
            }
            let mut pass_draws = Vec::new();
            for draw_call in &scene.draws {
                if draw_call.instances.is_empty() {
                    continue;
                }
                let instance_offset = all_instances.len() as u32;
                let instance_count = draw_call.instances.len() as u32;
                for instance in &draw_call.instances {
                    all_instances.push(World3dGpuInstance::from_instance(
                        instance.model.to_cols_array(),
                        instance.color,
                        instance.selected,
                        instance.hovered,
                    ));
                }
                pass_draws.push(WorldDrawRange {
                    mesh_key: draw_call.mesh_key.clone(),
                    mesh_version: draw_call.mesh_version,
                    instance_offset,
                    instance_count,
                });
            }
            let mut translucent_draws = Vec::new();
            for draw_call in &scene.translucent_draws {
                if draw_call.instances.is_empty() {
                    continue;
                }
                let instance_offset = all_instances.len() as u32;
                let instance_count = draw_call.instances.len() as u32;
                for instance in &draw_call.instances {
                    all_instances.push(World3dGpuInstance::from_instance(
                        instance.model.to_cols_array(),
                        instance.color,
                        instance.selected,
                        instance.hovered,
                    ));
                }
                translucent_draws.push(WorldDrawRange {
                    mesh_key: draw_call.mesh_key.clone(),
                    mesh_version: draw_call.mesh_version,
                    instance_offset,
                    instance_count,
                });
            }
            let line_start = all_lines.len() as u32;
            for line_draw in &scene.line_draws {
                for vertex in &line_draw.vertices {
                    all_lines.push(WorldLineGpuVertex {
                        position: vertex.position,
                        color: vertex.color,
                    });
                }
            }
            let line_count = all_lines.len() as u32 - line_start;
            pass_index_map[source_index] = Some(prepared.len());
            prepared.push(PreparedWorldPass {
                globals: World3dGlobals {
                    view_proj: scene.view_proj,
                    light_dir: [
                        scene.light_dir[0],
                        scene.light_dir[1],
                        scene.light_dir[2],
                        0.0,
                    ],
                },
                viewport: scene.viewport,
                draws: pass_draws,
                translucent_draws,
                line_start,
                line_count,
            });
        }
        (prepared, all_instances, all_lines, pass_index_map)
    }

    fn upload_world_passes(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        draw: &DrawList,
        frame_buffers: &mut FrameBuffers,
        filter: LayerBatchFilter,
    ) -> Option<(Vec<PreparedWorldPass>, Vec<Option<usize>>)> {
        if draw.scene_passes.is_empty() {
            return None;
        }
        let (prepared, all_instances, all_lines, pass_index_map) =
            Self::prepare_world_passes(draw, filter);
        if prepared.is_empty() {
            return None;
        }
        if all_instances.is_empty() && all_lines.is_empty() {
            return Some((prepared, pass_index_map));
        }
        self.world_globals_ring.ensure_slots(
            device,
            &self.world_bind_group_layout,
            prepared.len() as u32,
        );
        let globals: Vec<World3dGlobals> = prepared.iter().map(|pass| pass.globals).collect();
        self.world_globals_ring.write_passes(queue, &globals);
        if !all_instances.is_empty() {
            frame_buffers.world_instances.upload(
                device,
                queue,
                &all_instances,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "world3d_instances",
            );
        }
        if !all_lines.is_empty() {
            frame_buffers.world_lines.upload(
                device,
                queue,
                &all_lines,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "world3d_lines",
            );
        }
        Some((prepared, pass_index_map))
    }

    fn draw_world_pass_at<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        mesh_store: &MeshGpuStore,
        prepared: &PreparedWorldPass,
        slot: u32,
        instance_buffer: wgpu::BufferSlice<'a>,
        line_buffer: Option<wgpu::BufferSlice<'a>>,
        screen_w: f32,
        screen_h: f32,
    ) {
        let instance_stride = mem::size_of::<World3dGpuInstance>() as u64;
        pass.set_pipeline(&self.world_pipeline);
        let viewport = prepared.viewport;
        pass.set_viewport(viewport[0], viewport[1], viewport[2], viewport[3], 0.0, 1.0);
        pass.set_scissor_rect(
            viewport[0] as u32,
            viewport[1] as u32,
            viewport[2] as u32,
            viewport[3] as u32,
        );
        pass.set_bind_group(
            0,
            &self.world_globals_ring.bind_group,
            &[self.world_globals_ring.offset_for_slot(slot)],
        );
        for draw_call in &prepared.draws {
            Self::draw_world_range(pass, mesh_store, draw_call, instance_buffer.clone(), instance_stride);
        }
        if prepared.line_count > 0 {
            if let Some(line_buffer) = line_buffer {
                pass.set_pipeline(&self.world_line_pipeline);
                pass.set_bind_group(
                    0,
                    &self.world_globals_ring.bind_group,
                    &[self.world_globals_ring.offset_for_slot(slot)],
                );
                let line_stride = mem::size_of::<WorldLineGpuVertex>() as u64;
                let byte_offset = prepared.line_start as u64 * line_stride;
                pass.set_vertex_buffer(
                    0,
                    line_buffer.slice(byte_offset..byte_offset + prepared.line_count as u64 * line_stride),
                );
                pass.draw(0..prepared.line_count, 0..1);
            }
        }
        if !prepared.translucent_draws.is_empty() {
            pass.set_pipeline(&self.world_pipeline_translucent);
            pass.set_bind_group(
                0,
                &self.world_globals_ring.bind_group,
                &[self.world_globals_ring.offset_for_slot(slot)],
            );
            for draw_call in &prepared.translucent_draws {
                Self::draw_world_range(pass, mesh_store, draw_call, instance_buffer.clone(), instance_stride);
            }
        }
        pass.set_viewport(0.0, 0.0, screen_w, screen_h, 0.0, 1.0);
        pass.set_scissor_rect(0, 0, screen_w as u32, screen_h as u32);
        pass.set_pipeline(&self.ui_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
    }

    fn draw_world_range<'a>(
        pass: &mut wgpu::RenderPass<'a>,
        mesh_store: &MeshGpuStore,
        draw_call: &WorldDrawRange,
        instance_buffer: wgpu::BufferSlice<'a>,
        instance_stride: u64,
    ) {
        let store_key = MeshGpuStore::lookup_key(&draw_call.mesh_key, draw_call.mesh_version);
        let Some(mesh) = mesh_store.get(&store_key) else {
            return;
        };
        let byte_offset = draw_call.instance_offset as u64 * instance_stride;
        pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(
            1,
            instance_buffer.slice(byte_offset..byte_offset + draw_call.instance_count as u64 * instance_stride),
        );
        pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..mesh.index_count, 0, 0..draw_call.instance_count);
    }

    fn draw_ui_instances<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        instance_buffer: &wgpu::BufferSlice<'a>,
        start: u32,
        count: u32,
    ) {
        if count == 0 {
            return;
        }
        pass.set_pipeline(&self.ui_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
        pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.clone());
        pass.draw(0..6, start..start + count);
    }

    fn draw_raster_layers(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        raster_store: &RasterTextureStore,
        draw: &DrawList,
        frame_buffers: &mut FrameBuffers,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: f32,
        height: f32,
        filter: LayerBatchFilter,
    ) {
        for layer in &draw.layers {
            if !layer_matches_filter(layer, filter) {
                continue;
            }
            if layer.raster_instances.is_empty() {
                continue;
            }
            if let Some(scissor) = layer.scissor {
                set_pass_scissor(pass, Some(scissor), width, height);
            } else {
                pass.set_scissor_rect(0, 0, width as u32, height as u32);
            }
            let mut batch_key: Option<String> = None;
            let mut batch_instances: Vec<UiInstance> = Vec::new();
            let mut flush = |key: &str, instances: &[UiInstance]| {
                if instances.is_empty() {
                    return;
                }
                let Some(rt) = raster_store.get(key) else {
                    return;
                };
                pass.set_pipeline(&self.ui_pipeline);
                pass.set_bind_group(0, &rt.bind_group, &[]);
                let Some(buffer) = frame_buffers.ui_instances.upload(
                    device,
                    queue,
                    instances,
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    "raster_instances",
                ) else {
                    return;
                };
                pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                pass.set_vertex_buffer(1, buffer);
                pass.draw(0..6, 0..instances.len() as u32);
            };
            for (key, instance) in &layer.raster_instances {
                if batch_key.as_deref() != Some(key.as_str()) {
                    if let Some(ref prior) = batch_key {
                        flush(prior, &batch_instances);
                    }
                    batch_key = Some(key.clone());
                    batch_instances.clear();
                }
                batch_instances.push(*instance);
            }
            if let Some(ref key) = batch_key {
                flush(key, &batch_instances);
            }
        }
        pass.set_scissor_rect(0, 0, width as u32, height as u32);
    }

    fn draw_vector_vertices<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        vector_buffer: &wgpu::BufferSlice<'a>,
        start: u32,
        count: u32,
    ) {
        if count == 0 {
            return;
        }
        pass.set_pipeline(&self.vector_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
        pass.set_vertex_buffer(0, vector_buffer.clone());
        pass.draw(start..start + count, 0..1);
    }

    fn render_interleaved_layers<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        draw: &DrawList,
        batches: &[LayerBatch],
        ui_buffer: Option<&wgpu::BufferSlice<'a>>,
        vector_buffer: Option<&wgpu::BufferSlice<'a>>,
        world_prepared: Option<&[PreparedWorldPass]>,
        pass_index_map: &[Option<usize>],
        instance_buffer: Option<wgpu::BufferSlice<'a>>,
        line_buffer: Option<wgpu::BufferSlice<'a>>,
        mesh_store: &MeshGpuStore,
        width: f32,
        height: f32,
        depth_enabled: bool,
    ) {
        for batch in batches {
            set_pass_scissor(pass, batch.scissor, width, height);
            let mut layer_passes: Vec<(usize, usize, usize)> = draw
                .scene_passes
                .iter()
                .enumerate()
                .filter(|(_, scene)| scene.layer_index == batch.layer_index)
                .map(|(index, scene)| (index, scene.ui_watermark, scene.vector_watermark))
                .collect();
            layer_passes.sort_by_key(|(_, ui, vec)| (*ui, *vec));
            if layer_passes.is_empty() {
                if let Some(instance_buffer) = ui_buffer {
                    self.draw_ui_instances(pass, instance_buffer, batch.ui_start, batch.ui_count);
                }
                if let Some(vector_buffer) = vector_buffer {
                    self.draw_vector_vertices(pass, vector_buffer, batch.vec_start, batch.vec_count);
                }
                continue;
            }
            let mut ui_local = 0u32;
            let mut vec_local = 0u32;
            for (pass_index, ui_mark, vec_mark) in layer_passes {
                let ui_mark = ui_mark as u32;
                let vec_mark = vec_mark as u32;
                if ui_mark > ui_local {
                    if let Some(instance_buffer) = ui_buffer {
                        self.draw_ui_instances(
                            pass,
                            instance_buffer,
                            batch.ui_start + ui_local,
                            ui_mark - ui_local,
                        );
                    }
                    ui_local = ui_mark;
                }
                if vec_mark > vec_local {
                    if let Some(vector_buffer) = vector_buffer {
                        self.draw_vector_vertices(
                            pass,
                            vector_buffer,
                            batch.vec_start + vec_local,
                            vec_mark - vec_local,
                        );
                    }
                    vec_local = vec_mark;
                }
                if depth_enabled {
                    if let (Some(prepared), Some(instance_buffer)) =
                        (world_prepared, instance_buffer.as_ref())
                    {
                        if let Some(prepared_slot) =
                            pass_index_map.get(pass_index).and_then(|slot| *slot)
                        {
                            if let Some(scene) = prepared.get(prepared_slot) {
                                self.draw_world_pass_at(
                                    pass,
                                    mesh_store,
                                    scene,
                                    prepared_slot as u32,
                                    instance_buffer.clone(),
                                    line_buffer.clone(),
                                    width,
                                    height,
                                );
                            }
                        }
                    }
                }
            }
            if ui_local < batch.ui_count {
                if let Some(instance_buffer) = ui_buffer {
                    self.draw_ui_instances(
                        pass,
                        instance_buffer,
                        batch.ui_start + ui_local,
                        batch.ui_count - ui_local,
                    );
                }
            }
            if vec_local < batch.vec_count {
                if let Some(vector_buffer) = vector_buffer {
                    self.draw_vector_vertices(
                        pass,
                        vector_buffer,
                        batch.vec_start + vec_local,
                        batch.vec_count - vec_local,
                    );
                }
            }
        }
        pass.set_scissor_rect(0, 0, width as u32, height as u32);
    }

    pub fn update_globals(&self, queue: &wgpu::Queue, width: f32, height: f32) {
        queue.write_buffer(
            &self.globals_buffer,
            0,
            bytemuck::bytes_of(&UiGlobals {
                screen_size: [width, height],
                _pad: [0.0, 0.0],
            }),
        );
    }

    pub fn upload_glyph_atlas(&self, queue: &wgpu::Queue, pixels: &[u8], width: u32, height: u32) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.glyph_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }

    pub fn upload_icon_atlas(&self, queue: &wgpu::Queue, pixels: &[u8], width: u32, height: u32) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.icon_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }

    pub fn render_scene_content<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        scene: &'a SceneColorTarget,
        depth_view: Option<&'a wgpu::TextureView>,
        draw: &DrawList,
        mesh_store: &MeshGpuStore,
        raster_store: &RasterTextureStore,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        self.update_globals(queue, width, height);
        let scene_view = scene.mip_view(0);
        let world_upload = if depth_view.is_some() {
            self.upload_world_passes(
                device,
                queue,
                draw,
                frame_buffers,
                LayerBatchFilter::Backdrop,
            )
        } else {
            None
        };
        let (prepared_holder, pass_index_map) = match world_upload {
            Some((prepared, map)) => (Some(prepared), map),
            None => (None, vec![None; draw.scene_passes.len()]),
        };
        let world_prepared = prepared_holder.as_ref().map(|prepared| prepared.as_slice());
        let (all_ui, all_vec, batches) = build_layer_batches(draw, LayerBatchFilter::Backdrop);
        let ui_buffer = if all_ui.is_empty() {
            None
        } else {
            frame_buffers.ui_instances.upload(
                device,
                queue,
                &all_ui,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "ui_instances",
            )
        };
        let vector_buffer = if all_vec.is_empty() {
            None
        } else {
            frame_buffers.vector_vertices.upload(
                device,
                queue,
                &all_vec,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "vector_vertices",
            )
        };
        let instance_buffer = frame_buffers.world_instances.slice();
        let line_buffer = frame_buffers.world_lines.slice();
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ui_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: scene_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.05,
                        b: 0.06,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        self.render_interleaved_layers(
            &mut pass,
            draw,
            &batches,
            ui_buffer.as_ref(),
            vector_buffer.as_ref(),
            world_prepared,
            &pass_index_map,
            instance_buffer,
            line_buffer,
            mesh_store,
            width,
            height,
            depth_view.is_some(),
        );
        drop(pass);
        if draw.layers.iter().any(|layer| {
            layer_matches_filter(layer, LayerBatchFilter::Backdrop) && !layer.raster_instances.is_empty()
        }) {
            let mut raster_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ui_raster_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: scene_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.draw_raster_layers(
                &mut raster_pass,
                raster_store,
                draw,
                frame_buffers,
                device,
                queue,
                width,
                height,
                LayerBatchFilter::Backdrop,
            );
        }
        let (overlay_ui, overlay_vec, overlay_batches) =
            build_overlay_layer_batches(draw, LayerBatchFilter::Backdrop);
        if !overlay_ui.is_empty() || !overlay_vec.is_empty() {
            let overlay_ui_buffer = if overlay_ui.is_empty() {
                None
            } else {
                frame_buffers.ui_instances.upload(
                    device,
                    queue,
                    &overlay_ui,
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    "overlay_ui_instances",
                )
            };
            let overlay_vector_buffer = if overlay_vec.is_empty() {
                None
            } else {
                frame_buffers.vector_vertices.upload(
                    device,
                    queue,
                    &overlay_vec,
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    "overlay_vector_vertices",
                )
            };
            let mut overlay_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ui_overlay_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: scene_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.render_interleaved_layers(
                &mut overlay_pass,
                draw,
                &overlay_batches,
                overlay_ui_buffer.as_ref(),
                overlay_vector_buffer.as_ref(),
                None,
                &[],
                None,
                None,
                mesh_store,
                width,
                height,
                depth_view.is_some(),
            );
        }
    }

    fn has_glass_foreground(draw: &DrawList) -> bool {
        let layer_content = draw.layers.iter().any(|layer| {
            layer.foreground_of.is_some()
                && (!layer.ui_instances.is_empty()
                    || !layer.vector_vertices.is_empty()
                    || !layer.raster_instances.is_empty())
        });
        let scene_content = draw.scene_passes.iter().any(|pass| {
            layer_matches_filter(&draw.layers[pass.layer_index], LayerBatchFilter::Foreground)
        });
        layer_content || scene_content
    }

    fn render_glass_foreground<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        draw: &DrawList,
        depth_view: Option<&'a wgpu::TextureView>,
        mesh_store: &MeshGpuStore,
        raster_store: &RasterTextureStore,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        if !Self::has_glass_foreground(draw) {
            return;
        }
        let world_upload = if depth_view.is_some() {
            self.upload_world_passes(
                device,
                queue,
                draw,
                frame_buffers,
                LayerBatchFilter::Foreground,
            )
        } else {
            None
        };
        let (prepared_holder, pass_index_map) = match world_upload {
            Some((prepared, map)) => (Some(prepared), map),
            None => (None, vec![None; draw.scene_passes.len()]),
        };
        let world_prepared = prepared_holder.as_ref().map(|prepared| prepared.as_slice());
        let (all_ui, all_vec, batches) = build_layer_batches(draw, LayerBatchFilter::Foreground);
        if all_ui.is_empty()
            && all_vec.is_empty()
            && batches.is_empty()
            && world_prepared.is_none()
        {
            return;
        }
        let ui_buffer = if all_ui.is_empty() {
            None
        } else {
            frame_buffers.ui_instances.upload(
                device,
                queue,
                &all_ui,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "glass_foreground_ui_instances",
            )
        };
        let vector_buffer = if all_vec.is_empty() {
            None
        } else {
            frame_buffers.vector_vertices.upload(
                device,
                queue,
                &all_vec,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "glass_foreground_vector_vertices",
            )
        };
        let instance_buffer = frame_buffers.world_instances.slice();
        let line_buffer = frame_buffers.world_lines.slice();
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("glass_foreground_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        self.render_interleaved_layers(
            &mut pass,
            draw,
            &batches,
            ui_buffer.as_ref(),
            vector_buffer.as_ref(),
            world_prepared,
            &pass_index_map,
            instance_buffer,
            line_buffer,
            mesh_store,
            width,
            height,
            depth_view.is_some(),
        );
        drop(pass);
        if draw.layers.iter().any(|layer| {
            layer_matches_filter(layer, LayerBatchFilter::Foreground) && !layer.raster_instances.is_empty()
        }) {
            let mut raster_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("glass_foreground_raster_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.draw_raster_layers(
                &mut raster_pass,
                raster_store,
                draw,
                frame_buffers,
                device,
                queue,
                width,
                height,
                LayerBatchFilter::Foreground,
            );
        }
        let (overlay_ui, overlay_vec, overlay_batches) =
            build_overlay_layer_batches(draw, LayerBatchFilter::Foreground);
        if !overlay_ui.is_empty() || !overlay_vec.is_empty() {
            let overlay_ui_buffer = if overlay_ui.is_empty() {
                None
            } else {
                frame_buffers.ui_instances.upload(
                    device,
                    queue,
                    &overlay_ui,
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    "glass_foreground_overlay_ui_instances",
                )
            };
            let overlay_vector_buffer = if overlay_vec.is_empty() {
                None
            } else {
                frame_buffers.vector_vertices.upload(
                    device,
                    queue,
                    &overlay_vec,
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    "glass_foreground_overlay_vector_vertices",
                )
            };
            let mut overlay_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("glass_foreground_overlay_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.render_interleaved_layers(
                &mut overlay_pass,
                draw,
                &overlay_batches,
                overlay_ui_buffer.as_ref(),
                overlay_vector_buffer.as_ref(),
                None,
                &[],
                None,
                None,
                mesh_store,
                width,
                height,
                depth_view.is_some(),
            );
        }
    }

    pub fn composite_to_swapchain<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        scene: &'a SceneColorTarget,
        depth_view: Option<&'a wgpu::TextureView>,
        draw: &DrawList,
        overlay: Option<&DrawList>,
        mesh_store: &MeshGpuStore,
        raster_store: &RasterTextureStore,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        self.run_blur_chain(device, queue, scene);
        self.blit_scene_to_swapchain(device, encoder, view, scene);
        let max_mip = SCENE_MIP_LEVELS - 1;
        self.composite_glass_regions(
            device,
            queue,
            encoder,
            view,
            scene,
            frame_buffers,
            &draw.glass_regions,
            max_mip,
            width,
            height,
        );
        self.render_glass_foreground(
            device,
            queue,
            encoder,
            view,
            draw,
            depth_view,
            mesh_store,
            raster_store,
            frame_buffers,
            width,
            height,
        );
        if let Some(overlay) = overlay {
            if !overlay.glass_regions.is_empty() {
                self.composite_glass_regions(
                    device,
                    queue,
                    encoder,
                    view,
                    scene,
                    frame_buffers,
                    &overlay.glass_regions,
                    max_mip,
                    width,
                    height,
                );
            }
            self.render_glass_foreground(
                device,
                queue,
                encoder,
                view,
                overlay,
                depth_view,
                mesh_store,
                raster_store,
                frame_buffers,
                width,
                height,
            );
            let mut overlay_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ui_overlay_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.render_overlay(
                device,
                queue,
                &mut overlay_pass,
                overlay,
                frame_buffers,
                width,
                height,
            );
        }
    }

    pub fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        scene: &'a SceneColorTarget,
        depth_view: Option<&'a wgpu::TextureView>,
        draw: &DrawList,
        overlay: Option<&DrawList>,
        mesh_store: &MeshGpuStore,
        raster_store: &RasterTextureStore,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        self.render_scene_content(
            device,
            queue,
            encoder,
            scene,
            depth_view,
            draw,
            mesh_store,
            raster_store,
            frame_buffers,
            width,
            height,
        );
        self.composite_to_swapchain(
            device,
            queue,
            encoder,
            view,
            scene,
            depth_view,
            draw,
            overlay,
            mesh_store,
            raster_store,
            frame_buffers,
            width,
            height,
        );
    }

    fn run_blur_chain(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &SceneColorTarget,
    ) {
        for mip in 1..SCENE_MIP_LEVELS {
            let src_mip = mip - 1;
            queue.write_buffer(
                &self.blur_globals_buffer,
                0,
                bytemuck::bytes_of(&BlurGlobals {
                    src_mip: 0.0,
                    _pad: [0.0; 7],
                }),
            );
            let blur_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("blur_downsample_bind_group"),
                layout: &self.blur_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.blur_globals_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(scene.blur_scratch_mip_view(src_mip)),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(scene.sampler()),
                    },
                ],
            });
            let mut copy_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("blur_copy_encoder"),
            });
            scene.copy_mip_to_blur_scratch(&mut copy_encoder, src_mip);
            queue.submit(Some(copy_encoder.finish()));
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("blur_downsample_encoder"),
            });
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("blur_downsample_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: scene.mip_view(mip),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.blur_downsample_pipeline);
            pass.set_bind_group(0, &blur_bind_group, &[]);
            pass.draw(0..6, 0..1);
            drop(pass);
            queue.submit(Some(encoder.finish()));
        }
    }

    fn blit_scene_to_swapchain(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        scene: &SceneColorTarget,
    ) {
        let scene_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene_blit_bind_group"),
            layout: &self.scene_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(scene.sample_view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(scene.sampler()),
                },
            ],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scene_blit_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.05,
                        b: 0.06,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&self.scene_blit_pipeline);
        pass.set_bind_group(0, &scene_bind_group, &[]);
        pass.draw(0..6, 0..1);
    }

    fn composite_glass_regions(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        scene: &SceneColorTarget,
        frame_buffers: &mut FrameBuffers,
        regions: &[GlassRegion],
        max_mip: u32,
        width: f32,
        height: f32,
    ) {
        if regions.is_empty() {
            return;
        }
        let instances: Vec<GlassInstance> = regions
            .iter()
            .map(|region| GlassInstance {
                rect: region.rect,
                tint: [
                    region.tint.r,
                    region.tint.g,
                    region.tint.b,
                    region.tint.a,
                ],
                params: [
                    region.radius,
                    region.alpha,
                    Theme::glass_mip_level(region.blur_px, max_mip),
                    region.saturate,
                ],
            })
            .collect();
        let glass_buffer = frame_buffers.glass_instances.upload(
            device,
            queue,
            &instances,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            "glass_instances",
        );
        let Some(glass_buffer) = glass_buffer else {
            return;
        };
        let scene_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("glass_scene_bind_group"),
            layout: &self.scene_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(scene.sample_view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(scene.sampler()),
                },
            ],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("glass_composite_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&self.glass_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
        pass.set_bind_group(1, &scene_bind_group, &[]);
        pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, glass_buffer.slice(..));
        pass.draw(0..6, 0..instances.len() as u32);
        let _ = (width, height);
    }

    pub fn render_overlay<'a>(
        &'a self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'a>,
        overlay: &DrawList,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        pass.set_pipeline(&self.ui_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);

        let (all_ui, all_vec, batches) = build_layer_batches(overlay, LayerBatchFilter::Backdrop);
        let ui_buffer = if all_ui.is_empty() {
            None
        } else {
            frame_buffers.ui_instances.upload(
                device,
                queue,
                &all_ui,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "overlay_ui_instances",
            )
        };
        let vector_buffer = if all_vec.is_empty() {
            None
        } else {
            frame_buffers.vector_vertices.upload(
                device,
                queue,
                &all_vec,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "overlay_vector_vertices",
            )
        };

        for batch in &batches {
            set_pass_scissor(pass, batch.scissor, width, height);
            if batch.ui_count > 0 {
                if let Some(instance_buffer) = &ui_buffer {
                    pass.set_pipeline(&self.ui_pipeline);
                    pass.set_bind_group(0, &self.glyph_bind_group, &[]);
                    pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                    pass.set_vertex_buffer(1, instance_buffer.clone());
                    pass.draw(
                        0..6,
                        batch.ui_start..batch.ui_start + batch.ui_count,
                    );
                }
            }
            if batch.vec_count > 0 {
                if let Some(vector_buffer) = &vector_buffer {
                    pass.set_pipeline(&self.vector_pipeline);
                    pass.set_vertex_buffer(0, vector_buffer.clone());
                    pass.draw(
                        batch.vec_start..batch.vec_start + batch.vec_count,
                        0..1,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ear_clip_polygon, mesh_content_version, DrawList, ScissorRect, WORLD_GLOBALS_SLOT_SIZE};
    use crate::geometry::Rect;
    use kernel_3d_scene::ScenePass3d;
    use crate::theme::Rgba;

    #[test]
    fn scissor_intersects_child() {
        let a = ScissorRect { x: 0, y: 0, w: 100, h: 100 };
        let b = ScissorRect { x: 50, y: 50, w: 100, h: 100 };
        let c = a.intersect(&b);
        assert_eq!(c.w, 50);
        assert_eq!(c.h, 50);
    }

    #[test]
    fn scissor_from_rect_uses_top_left_origin() {
        let scissor = ScissorRect::from_rect(Rect::new(10.0, 20.0, 80.0, 60.0), 720.0);
        assert_eq!(scissor.x, 10);
        assert_eq!(scissor.y, 20);
        assert_eq!(scissor.w, 80);
        assert_eq!(scissor.h, 60);
    }

    #[test]
    fn draw_list_push_scissor_splits_layers() {
        let mut draw = DrawList::default();
        draw.set_screen_height(200.0);
        draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
        draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
        draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
        draw.pop_scissor();
        assert!(draw.layers.len() >= 3);
    }

    #[test]
    fn ear_clip_produces_triangles() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let tris = ear_clip_polygon(&square);
        assert!(tris.len() >= 3);
    }

    #[test]
    fn world_globals_slot_size_is_aligned() {
        assert!(WORLD_GLOBALS_SLOT_SIZE >= 80);
        assert_eq!(WORLD_GLOBALS_SLOT_SIZE % 256, 0);
    }

    #[test]
    fn scene_pass_records_layer_watermarks() {
        let mut draw = DrawList::default();
        draw.push_solid([0.0, 0.0, 10.0, 10.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
        draw.push_solid([1.0, 1.0, 8.0, 8.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
        draw.push_scene_pass(ScenePass3d {
            viewport: [0.0, 0.0, 100.0, 100.0],
            view_proj: [0.0; 16],
            light_dir: [0.0, 0.0, 1.0],
            ..Default::default()
        });
        draw.push_line(0.0, 0.0, 1.0, 1.0, Rgba::new(0.0, 0.0, 1.0, 1.0), 1.0);
        let pass = &draw.scene_passes[0];
        assert_eq!(pass.layer_index, 0);
        assert_eq!(pass.ui_watermark, 2);
        assert_eq!(pass.vector_watermark, 0);
        assert_eq!(draw.layers[0].ui_instances.len(), 2);
        assert_eq!(draw.layers[0].vector_vertices.len(), 6);
    }

    #[test]
    fn mesh_instances_without_lines_are_valid_world_pass() {
        use kernel_3d_scene::{Instance3d, SceneDraw3d, ScenePass3d};

        let pass = ScenePass3d {
            viewport: [0.0, 0.0, 320.0, 240.0],
            view_proj: [0.0; 16],
            light_dir: [0.4, 0.6, 0.8],
            draws: vec![SceneDraw3d {
                mesh_key: "box".into(),
                mesh_version: 1,
                instances: vec![Instance3d {
                    id: "preview".into(),
                    model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]),
                    color: [0.7, 0.7, 0.75, 1.0],
                    selected: false,
                    hovered: false,
                }],
            }],
            ..Default::default()
        };
        assert!(!pass.draws[0].instances.is_empty());
        assert!(pass.line_draws.is_empty());
    }

    #[test]
    fn mesh_content_version_changes_with_indices() {
        let v0 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 1, 2]);
        let v1 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 2, 1]);
        assert_ne!(v0, v1);
    }

    #[test]
    fn overlay_layers_collected_separately_from_backdrop_ui() {
        use super::{build_layer_batches, build_overlay_layer_batches, LayerBatchFilter};
        let mut draw = DrawList::default();
        draw.push_solid([0.0, 0.0, 100.0, 100.0], Rgba::new(0.1, 0.1, 0.1, 1.0));
        draw.push_glyph_overlay([10.0, 10.0, 20.0, 12.0], Rgba::new(1.0, 1.0, 1.0, 1.0), [0.0, 0.0, 0.1, 0.1]);
        draw.push_line_overlay(0.0, 0.0, 50.0, 50.0, Rgba::new(1.0, 0.0, 0.0, 1.0), 1.0);
        let (backdrop_ui, _, _) = build_layer_batches(&draw, LayerBatchFilter::Backdrop);
        let (overlay_ui, overlay_vec, overlay_batches) =
            build_overlay_layer_batches(&draw, LayerBatchFilter::Backdrop);
        assert_eq!(backdrop_ui.len(), 1);
        assert_eq!(overlay_ui.len(), 1);
        assert_eq!(overlay_vec.len(), 6);
        assert_eq!(overlay_batches.len(), 1);
        assert_eq!(draw.layers[overlay_batches[0].layer_index].overlay_ui_instances.len(), 1);
    }

    #[test]
    fn glass_content_layers_tagged_with_foreground_of() {
        use super::{GlassTier, Theme};
        let theme = Theme::default();
        let mut draw = DrawList::default();
        draw.push_solid([0.0, 0.0, 100.0, 100.0], Rgba::new(0.2, 0.2, 0.2, 1.0));
        let glass = draw.push_glass([10.0, 10.0, 80.0, 80.0], 8.0, GlassTier::Panel, &theme);
        assert_eq!(glass, 0);
        draw.begin_glass_content(glass);
        draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
        draw.end_glass_content();
        let backdrop = draw.layers.iter().filter(|layer| layer.foreground_of.is_none()).count();
        let foreground = draw
            .layers
            .iter()
            .filter(|layer| layer.foreground_of == Some(glass))
            .count();
        assert_eq!(backdrop, 2);
        assert_eq!(foreground, 1);
        assert_eq!(draw.layers[1].ui_instances.len(), 1);
    }

    #[test]
    fn glass_foreground_layers_excluded_from_backdrop_batches() {
        use super::{build_layer_batches, GlassTier, LayerBatchFilter, Theme};
        let theme = Theme::default();
        let mut draw = DrawList::default();
        draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(0.1, 0.1, 0.1, 1.0));
        let glass = draw.push_glass([20.0, 20.0, 160.0, 160.0], 8.0, GlassTier::Panel, &theme);
        draw.begin_glass_content(glass);
        draw.push_solid([20.0, 20.0, 160.0, 160.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
        draw.end_glass_content();
        let (backdrop_ui, _, backdrop_batches) = build_layer_batches(&draw, LayerBatchFilter::Backdrop);
        let (foreground_ui, _, foreground_batches) = build_layer_batches(&draw, LayerBatchFilter::Foreground);
        assert_eq!(backdrop_ui.len(), 1);
        assert_eq!(foreground_ui.len(), 1);
        assert_eq!(backdrop_batches.len(), 1);
        assert_eq!(foreground_batches.len(), 1);
        assert!(draw.layers[backdrop_batches[0].layer_index].foreground_of.is_none());
        assert_eq!(
            draw.layers[foreground_batches[0].layer_index].foreground_of,
            Some(glass)
        );
    }

    #[test]
    fn glass_scissor_inherits_foreground_tag() {
        use super::{GlassTier, Theme};
        let theme = Theme::default();
        let mut draw = DrawList::default();
        let glass = draw.push_glass([0.0, 0.0, 100.0, 100.0], 8.0, GlassTier::Panel, &theme);
        draw.begin_glass_content(glass);
        draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
        draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
        draw.pop_scissor();
        draw.end_glass_content();
        let scissor_layer = draw.layers.iter().find(|layer| layer.scissor.is_some()).expect("scissor layer");
        assert_eq!(scissor_layer.foreground_of, Some(glass));
    }
}
// #endregion draw
}

pub mod geometry {
// #region geometry
//! 📐 Axis-aligned rectangles for layout and hit testing.

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && py >= self.y && px < self.x + self.w && py < self.y + self.h
    }

    pub fn inset(&self, amount: f32) -> Self {
        Self {
            x: self.x + amount,
            y: self.y + amount,
            w: (self.w - amount * 2.0).max(0.0),
            h: (self.h - amount * 2.0).max(0.0),
        }
    }
}
// #endregion geometry
}

#[cfg(feature = "engine")]
pub mod gpu {
// #region gpu
//! 🖥️ WebGPU device, surface, and frame loop.

use crate::draw::{DrawList, FrameBuffers, MeshGpuStore, RasterTextureStore, SceneColorTarget, UiPipelines};
use crate::text::FontAtlas;
use wgpu::Surface;

pub struct GpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    color_target_format: wgpu::TextureFormat,
    pipelines: UiPipelines,
    frame_buffers: FrameBuffers,
    depth_texture: Option<wgpu::Texture>,
    depth_view: Option<wgpu::TextureView>,
    mesh_store: MeshGpuStore,
    raster_store: RasterTextureStore,
    scene_color: Option<SceneColorTarget>,
    width: u32,
    height: u32,
    dpr: f32,
}

impl GpuContext {
    pub async fn from_window(window: std::sync::Arc<winit::window::Window>) -> Result<Self, String> {
        let dpr = window.scale_factor() as f32;
        let size = window.inner_size();
        let css_width = size.width as f32 / dpr;
        let css_height = size.height as f32 / dpr;
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: if cfg!(target_arch = "wasm32") {
                wgpu::Backends::BROWSER_WEBGPU
            } else {
                wgpu::Backends::PRIMARY
            },
            ..Default::default()
        });
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Window(Box::new(window)))
            .map_err(|err| format!("surface: {err:?}"))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|err| format!("adapter: {err:?}"))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("ui_wgpu"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
                experimental_features: Default::default(),
            })
            .await
            .map_err(|err| format!("device: {err:?}"))?;
        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps
            .formats
            .iter()
            .copied()
            .find(|f| !f.is_srgb())
            .unwrap_or(caps.formats[0]);
        let color_target_format = if surface_format.is_srgb() {
            surface_format
        } else {
            surface_format.add_srgb_suffix()
        };
        let width = (css_width * dpr).max(1.0) as u32;
        let height = (css_height * dpr).max(1.0) as u32;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![color_target_format],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        let pipelines = UiPipelines::new(&device, &queue, color_target_format);
        let raster_store = RasterTextureStore::new(&device, pipelines.bind_group_layout());
        let mut gpu = Self {
            device,
            queue,
            surface,
            config,
            color_target_format,
            pipelines,
            frame_buffers: FrameBuffers::default(),
            depth_texture: None,
            depth_view: None,
            mesh_store: MeshGpuStore::default(),
            raster_store,
            scene_color: None,
            width,
            height,
            dpr,
        };
        gpu.ensure_depth();
        Ok(gpu)
    }

    fn ensure_depth(&mut self) {
        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ui_depth"),
            size: wgpu::Extent3d {
                width: self.width.max(1),
                height: self.height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.pipelines.depth_format(),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.depth_texture = Some(depth_texture);
        self.depth_view = Some(depth_view);
    }

    pub fn resize(&mut self, css_width: f32, css_height: f32, dpr: f32) {
        self.dpr = dpr;
        let width = (css_width * dpr).max(1.0) as u32;
        let height = (css_height * dpr).max(1.0) as u32;
        if width == self.width && height == self.height {
            return;
        }
        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.scene_color = None;
        self.ensure_depth();
    }

    fn ensure_scene_color(&mut self) {
        SceneColorTarget::ensure(
            &self.device,
            &mut self.scene_color,
            self.width,
            self.height,
            self.color_target_format,
        );
    }

    pub fn mesh_store_mut(&mut self) -> &mut MeshGpuStore {
        &mut self.mesh_store
    }

    pub fn ensure_mesh(&mut self, key: &str, version: u64, positions: &[f32], normals: &[f32], indices: &[u32]) {
        self.mesh_store
            .ensure_mesh(&self.device, key, version, positions, normals, indices);
    }

    pub fn evict_mesh(&mut self, key: &str) {
        self.mesh_store.evict_mesh(key);
    }

    pub fn render_frame(&mut self, draw: &DrawList, overlay: Option<&DrawList>) -> Result<(), String> {
        self.ensure_scene_color();
        let scene = self.scene_color.as_ref().expect("scene_color");
        let frame = self
            .surface
            .get_current_texture()
            .map_err(|err| format!("frame: {err:?}"))?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(self.color_target_format),
            ..Default::default()
        });
        let depth_view = self.depth_view.as_ref();
        let mut scene_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("ui_wgpu_scene") });
        self.pipelines.render_scene_content(
            &self.device,
            &self.queue,
            &mut scene_encoder,
            scene,
            depth_view,
            draw,
            &self.mesh_store,
            &self.raster_store,
            &mut self.frame_buffers,
            self.width as f32,
            self.height as f32,
        );
        self.queue.submit(Some(scene_encoder.finish()));
        let mut composite_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("ui_wgpu_composite") });
        self.pipelines.composite_to_swapchain(
            &self.device,
            &self.queue,
            &mut composite_encoder,
            &view,
            scene,
            depth_view,
            draw,
            overlay,
            &self.mesh_store,
            &self.raster_store,
            &mut self.frame_buffers,
            self.width as f32,
            self.height as f32,
        );
        self.queue.submit(Some(composite_encoder.finish()));
        frame.present();
        Ok(())
    }

    pub fn upload_font_atlas(&self, atlas: &FontAtlas) {
        self.pipelines
            .upload_glyph_atlas(&self.queue, &atlas.pixels, atlas.width, atlas.height);
    }

    pub fn upload_icon_atlas(&self, atlas: &crate::draw::IconAtlas) {
        self.pipelines.upload_icon_atlas(
            &self.queue,
            &atlas.pixels,
            atlas.width,
            atlas.height,
        );
    }

    pub fn ensure_raster_texture(&mut self, key: &str, pixels: &[u8], width: u32, height: u32) {
        self.raster_store.ensure_raster(
            &self.device,
            &self.queue,
            self.pipelines.globals_buffer(),
            &self.pipelines.glyph_view(),
            self.pipelines.glyph_sampler(),
            &self.pipelines.icon_view(),
            self.pipelines.icon_sampler(),
            key,
            pixels,
            width,
            height,
        );
    }

    pub fn ensure_world_plane_texture(&mut self, key: &str, pixels: &[u8], width: u32, height: u32) {
        self.ensure_raster_texture(key, pixels, width, height);
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn dpr(&self) -> f32 {
        self.dpr
    }

    pub fn register_engine_texture(
        &mut self,
        key: &str,
        texture: wgpu::Texture,
        view: &wgpu::TextureView,
        width: u32,
        height: u32,
    ) {
        self.raster_store.replace_gpu_bind_group(
            &self.device,
            self.pipelines.globals_buffer(),
            &self.pipelines.glyph_view(),
            self.pipelines.glyph_sampler(),
            key,
            view,
            texture,
            width,
            height,
        );
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

#[cfg(target_arch = "wasm32")]
pub fn schedule_frame(window: &winit::window::Window, callback: impl FnMut() + 'static) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let mut callback = callback;
    let closure = Closure::wrap(Box::new(move || {
        callback();
    }) as Box<dyn FnMut()>);
    web_sys::window()
        .and_then(|w| w.request_animation_frame(closure.as_ref().unchecked_ref()).ok());
    closure.forget();
    let _ = window;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn schedule_frame(window: &winit::window::Window, _callback: impl FnMut() + 'static) {
    window.request_redraw();
}
// #endregion gpu
}

#[cfg(feature = "engine")]
pub mod input {
// #region input
//! 🖱️ Pointer and keyboard input state for hit testing.

use crate::geometry::Rect;
use std::rc::Rc;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct HitTarget<E> {
    pub rect: Rect,
    pub event: Option<E>,
    pub control_id: Option<String>,
    pub kind: HitKind,
    pub drag_axis: Option<DragAxis>,
    pub drag_data: Option<HashMap<String, String>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragAxis {
    Horizontal,
    Vertical,
    Both,
    Ring,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeDropPosition {
    Before,
    After,
    Inside,
}

#[derive(Clone, Debug)]
pub struct TreeDragState {
    pub source_id: String,
    pub drag_data: HashMap<String, String>,
    pub x: f32,
    pub y: f32,
    pub drop_target_id: Option<String>,
    pub drop_position: TreeDropPosition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitKind {
    Button,
    Toggle,
    Input,
    Select,
    Slider,
    TreeItem,
    TreeDropTarget,
    PanelTab,
    NavbarItem,
    Window,
    World3d,
    PanelResize,
    DockSplit,
    DockJoinCorner,
    ScrollRegion,
    ContextMenu,
    DropdownItem,
    Generic,
}

#[derive(Clone, Debug, Default)]
pub struct PointerModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl PointerModifiers {
    pub fn ctrl_or_meta(&self) -> bool {
        self.ctrl || self.meta
    }
}

#[derive(Clone, Debug)]
pub struct DragState {
    pub active: bool,
    pub button: i16,
    pub start_x: f32,
    pub start_y: f32,
    pub current_x: f32,
    pub current_y: f32,
    pub target_id: Option<String>,
    pub axis: Option<DragAxis>,
    pub kind: Option<HitKind>,
    pub points: Vec<[f32; 2]>,
}

impl Default for DragState {
    fn default() -> Self {
        Self {
            active: false,
            button: 0,
            start_x: 0.0,
            start_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
            target_id: None,
            axis: None,
            kind: None,
            points: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyAction {
    Char(String),
    Backspace,
    Delete,
    Enter,
    Escape,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Tab,
    Space(bool),
}

pub struct InputState<E> {
    pub pointer_x: f32,
    pub pointer_y: f32,
    pub pointer_down: bool,
    pub pointer_button: i16,
    pub wheel_delta: f32,
    pub modifiers: PointerModifiers,
    pub drag: DragState,
    pub hovered_id: Option<String>,
    pub focused_id: Option<String>,
    pub text_buffer: String,
    pub cursor_pos: usize,
    pub hit_targets: Vec<HitTarget<E>>,
    pub pending_events: Vec<E>,
    pub pending_keys: Vec<KeyAction>,
    pub right_click_pos: Option<(f32, f32)>,
}

impl<E> Default for InputState<E> {
    fn default() -> Self {
        Self {
            pointer_x: 0.0,
            pointer_y: 0.0,
            pointer_down: false,
            pointer_button: 0,
            wheel_delta: 0.0,
            modifiers: PointerModifiers::default(),
            drag: DragState::default(),
            hovered_id: None,
            focused_id: None,
            text_buffer: String::new(),
            cursor_pos: 0,
            hit_targets: Vec::new(),
            pending_events: Vec::new(),
            pending_keys: Vec::new(),
            right_click_pos: None,
        }
    }
}

impl<E: Clone> InputState<E> {
    pub fn clear_frame(&mut self) {
        self.hit_targets.clear();
        self.wheel_delta = 0.0;
        self.right_click_pos = None;
    }

    pub fn register_hit(&mut self, target: HitTarget<E>) {
        self.hit_targets.push(target);
    }

    pub fn hit_at(&self, x: f32, y: f32) -> Option<&HitTarget<E>> {
        self.hit_targets
            .iter()
            .rev()
            .find(|target| target.rect.contains(x, y))
    }

    pub fn update_hover(&mut self, x: f32, y: f32) {
        self.pointer_x = x;
        self.pointer_y = y;
        self.hovered_id = self
            .hit_at(x, y)
            .and_then(|hit| hit.control_id.clone());
    }

    pub fn begin_drag(
        &mut self,
        x: f32,
        y: f32,
        button: i16,
        target_id: Option<String>,
        axis: Option<DragAxis>,
        kind: Option<HitKind>,
    ) {
        self.drag = DragState {
            active: true,
            button,
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            target_id,
            axis,
            kind,
            points: vec![[x, y]],
        };
    }

    pub fn update_drag(&mut self, x: f32, y: f32) {
        if self.drag.active {
            self.drag.current_x = x;
            self.drag.current_y = y;
            self.drag.points.push([x, y]);
        }
    }

    pub fn end_drag(&mut self) -> DragState {
        let drag = self.drag.clone();
        self.drag = DragState::default();
        drag
    }

    pub fn drain_events(&mut self) -> Vec<E> {
        std::mem::take(&mut self.pending_events)
    }

    pub fn drain_keys(&mut self) -> Vec<KeyAction> {
        std::mem::take(&mut self.pending_keys)
    }

    pub fn queue_event(&mut self, event: E) {
        self.pending_events.push(event);
    }

    pub fn queue_key(&mut self, action: KeyAction) {
        self.pending_keys.push(action);
    }

    pub fn focus_input(&mut self, id: &str, value: &str) {
        self.focused_id = Some(id.to_string());
        self.text_buffer = value.to_string();
        self.cursor_pos = value.len();
    }

    pub fn blur_input(&mut self) {
        self.focused_id = None;
        self.text_buffer.clear();
        self.cursor_pos = 0;
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.cursor_pos <= self.text_buffer.len() {
            self.text_buffer.insert(self.cursor_pos, ch);
            self.cursor_pos += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.text_buffer.remove(self.cursor_pos);
        }
    }

    pub fn delete_forward(&mut self) {
        if self.cursor_pos < self.text_buffer.len() {
            self.text_buffer.remove(self.cursor_pos);
        }
    }

    pub fn move_cursor(&mut self, delta: i32) {
        let len = self.text_buffer.len() as i32;
        self.cursor_pos = ((self.cursor_pos as i32) + delta).clamp(0, len) as usize;
    }
}

#[derive(Clone)]
pub struct PointerCallbacks {
    pub on_move: Rc<dyn Fn(f32, f32, bool, i16, PointerModifiers)>,
    pub on_button: Rc<dyn Fn(f32, f32, bool, i16, PointerModifiers)>,
    pub on_wheel: Rc<dyn Fn(f32, f32, f32, PointerModifiers)>,
    pub on_key: Rc<dyn Fn(KeyAction, PointerModifiers)>,
    pub on_context_menu: Rc<dyn Fn(f32, f32)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_at_prefers_content_registered_after_scroll_region() {
        let mut input = InputState::<()>::default();
        let scroll = Rect::new(0.0, 0.0, 200.0, 200.0);
        let row = Rect::new(0.0, 24.0, 200.0, 24.0);
        input.register_hit(HitTarget {
            rect: scroll,
            event: None,
            control_id: Some("scroll".into()),
            kind: HitKind::ScrollRegion,
            drag_axis: None,
            drag_data: None,
        });
        input.register_hit(HitTarget {
            rect: row,
            event: None,
            control_id: Some("tree.label.item-1".into()),
            kind: HitKind::TreeItem,
            drag_axis: None,
            drag_data: None,
        });
        let hit = input.hit_at(10.0, 36.0).expect("row point should hit");
        assert_eq!(hit.control_id.as_deref(), Some("tree.label.item-1"));
        assert_eq!(hit.kind, HitKind::TreeItem);
    }
}
// #endregion input
}

#[cfg(feature = "engine")]
pub mod layout {
// #region layout
//! 🧮 Flex stack layout for widget trees.

use crate::geometry::Rect;
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Vertical,
    Horizontal,
}

pub fn gap_for_token(theme: &Theme, token: Option<&str>) -> f32 {
    match token {
        Some("tight") => 4.0,
        Some("loose") => 12.0,
        Some("none") | Some("0") => 0.0,
        _ => theme.gap_standard,
    }
}

pub fn padding_for_token(theme: &Theme, token: Option<&str>) -> f32 {
    match token {
        Some("none") | Some("0") => 0.0,
        Some("tight") => 6.0,
        Some("loose") => 16.0,
        _ => theme.padding_standard,
    }
}

pub fn layout_vertical(
    bounds: Rect,
    gap: f32,
    padding: f32,
    child_heights: &[f32],
) -> Vec<Rect> {
    let inner = bounds.inset(padding);
    let total_gap = gap * (child_heights.len().saturating_sub(1) as f32);
    let total_children: f32 = child_heights.iter().sum();
    let mut y = inner.y;
    let mut rects = Vec::with_capacity(child_heights.len());
    let available = (inner.h - total_gap - total_children).max(0.0);
    let extra_per_child = if child_heights.is_empty() {
        0.0
    } else {
        available / child_heights.len() as f32
    };
    for &height in child_heights {
        let h = height + extra_per_child;
        rects.push(Rect::new(inner.x, y, inner.w, h));
        y += h + gap;
    }
    rects
}

pub fn layout_horizontal(
    bounds: Rect,
    gap: f32,
    padding: f32,
    child_widths: &[f32],
) -> Vec<Rect> {
    let inner = bounds.inset(padding);
    let total_gap = gap * (child_widths.len().saturating_sub(1) as f32);
    let total_children: f32 = child_widths.iter().sum();
    let mut x = inner.x;
    let mut rects = Vec::with_capacity(child_widths.len());
    let available = (inner.w - total_gap - total_children).max(0.0);
    let extra_per_child = if child_widths.is_empty() {
        0.0
    } else {
        available / child_widths.len() as f32
    };
    for &width in child_widths {
        let w = width + extra_per_child;
        rects.push(Rect::new(x, inner.y, w, inner.h));
        x += w + gap;
    }
    rects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_layout_distributes_children() {
        let theme = Theme::default();
        let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rects = layout_vertical(bounds, 4.0, 8.0, &[20.0, 30.0]);
        assert_eq!(rects.len(), 2);
        assert!(rects[0].h > 20.0);
        assert!(rects[1].y > rects[0].y);
        let _ = theme;
    }
}
// #endregion layout
}

#[cfg(feature = "engine")]
pub mod shaders {
// #region shaders
//! 🧊 WGSL shader sources for the raw wgpu UI renderer.

pub const UI_SHADER: &str = r#"
struct Globals {
    screen_size: vec2<f32>,
    _pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(0) @binding(1) var glyph_atlas: texture_2d<f32>;
@group(0) @binding(2) var glyph_sampler: sampler;
@group(0) @binding(3) var icon_atlas: texture_2d<f32>;
@group(0) @binding(4) var icon_sampler: sampler;

struct VertexInput {
    @location(0) corner: vec2<f32>,
}

struct InstanceInput {
    @location(1) rect: vec4<f32>,
    @location(2) color: vec4<f32>,
    @location(3) params: vec4<f32>,
    @location(4) uv_rect: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) params: vec4<f32>,
    @location(4) uv: vec2<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let pos = instance.rect.xy + vertex.corner * instance.rect.zw;
    let ndc = (pos / globals.screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
    out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    out.local = vertex.corner * instance.rect.zw;
    out.size = instance.rect.zw;
    out.color = instance.color;
    out.params = instance.params;
    let uv_min = instance.uv_rect.xy;
    let uv_max = instance.uv_rect.zw;
    out.uv = mix(uv_min, uv_max, vertex.corner);
    return out;
}

fn sdf_rounded_rect(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let kind = i32(in.params.z + 0.5);
    let glyph = textureSample(glyph_atlas, glyph_sampler, in.uv);
    let icon = textureSample(icon_atlas, icon_sampler, in.uv);
    if (kind == 1) {
        let half = in.size * 0.5;
        let p = in.local - half;
        let radius = in.params.x;
        let border = in.params.y;
        let dist = sdf_rounded_rect(p, half, radius);
        let fill_alpha = 1.0 - smoothstep(-1.0, 0.0, dist);
        let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        let alpha = max(fill_alpha * in.color.a, border_alpha * in.params.w);
        return vec4<f32>(in.color.rgb, alpha);
    }
    if (kind == 2) {
        return vec4<f32>(in.color.rgb, glyph.r * in.color.a);
    }
    if (kind == 4 || kind == 5) {
        return vec4<f32>(icon.rgb * in.color.rgb, icon.a * in.color.a);
    }
    if (kind == 3) {
        return in.color;
    }
    return in.color;
}
"#;

pub const VECTOR_SHADER: &str = r#"
struct Globals {
    screen_size: vec2<f32>,
    _pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let ndc = (vertex.position / globals.screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
    out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    out.color = vertex.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

pub const WORLD3D_SHADER: &str = r#"
struct Globals {
    view_proj: mat4x4<f32>,
    light_dir: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct InstanceInput {
    @location(3) model0: vec4<f32>,
    @location(4) model1: vec4<f32>,
    @location(5) model2: vec4<f32>,
    @location(6) model3: vec4<f32>,
    @location(7) color: vec4<f32>,
    @location(8) flags: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) flags: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let model = mat4x4<f32>(instance.model0, instance.model1, instance.model2, instance.model3);
    let world_pos = model * vec4<f32>(vertex.position, 1.0);
    out.clip_position = globals.view_proj * world_pos;
    let normal_matrix = mat3x3<f32>(
        model[0].xyz,
        model[1].xyz,
        model[2].xyz
    );
    out.normal = normalize(normal_matrix * vertex.normal);
    out.color = instance.color;
    out.flags = instance.flags;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    let diffuse = max(dot(n, normalize(globals.light_dir.xyz)), 0.28);
    var color = in.color.rgb * diffuse;
    if (in.flags.x > 0.5) {
        color = mix(color, vec3<f32>(0.35, 0.75, 1.0), 0.65);
    }
    if (in.flags.y > 0.5) {
        color = mix(color, vec3<f32>(1.0, 0.85, 0.35), 0.55);
    }
    return vec4<f32>(color, in.color.a);
}
"#;

pub const WORLD3D_LINES_SHADER: &str = r#"
struct Globals {
    view_proj: mat4x4<f32>,
    light_dir: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = globals.view_proj * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

pub const WORLD3D_TEXTURED_SHADER: &str = r#"
struct Globals {
    view_proj: mat4x4<f32>,
    light_dir: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var tex_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct InstanceInput {
    @location(3) model0: vec4<f32>,
    @location(4) model1: vec4<f32>,
    @location(5) model2: vec4<f32>,
    @location(6) model3: vec4<f32>,
    @location(7) tint: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tint: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let model = mat4x4<f32>(instance.model0, instance.model1, instance.model2, instance.model3);
    let world_pos = model * vec4<f32>(vertex.position, 1.0);
    out.clip_position = globals.view_proj * world_pos;
    out.uv = vertex.uv;
    out.tint = instance.tint;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sampled = textureSample(tex, tex_sampler, in.uv);
    return vec4<f32>(sampled.rgb * in.tint.rgb, sampled.a * in.tint.a);
}
"#;

pub const BLUR_DOWNSAMPLE_SHADER: &str = r#"
struct BlurGlobals {
    src_mip: f32,
    _pad: vec3<f32>,
}

@group(0) @binding(0) var<uniform> blur_globals: BlurGlobals;
@group(0) @binding(1) var src_tex: texture_2d<f32>;
@group(0) @binding(2) var src_samp: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0)
    );
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(1.0, 0.0)
    );
    var out: VertexOutput;
    let pos = positions[vid];
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.uv = uvs[vid];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let mip = u32(blur_globals.src_mip);
    let dim = vec2<f32>(textureDimensions(src_tex, mip));
    let texel = vec2<f32>(1.0) / dim;
    let uv = in.uv;
    let src_mip = blur_globals.src_mip;
    var c = textureSampleLevel(src_tex, src_samp, uv, src_mip) * 4.0;
    c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(-texel.x, 0.0), src_mip);
    c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(texel.x, 0.0), src_mip);
    c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(0.0, -texel.y), src_mip);
    c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(0.0, texel.y), src_mip);
    return c / 8.0;
}
"#;

pub const SCENE_BLIT_SHADER: &str = r#"
@group(0) @binding(0) var scene_tex: texture_2d<f32>;
@group(0) @binding(1) var scene_samp: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0)
    );
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(1.0, 0.0)
    );
    var out: VertexOutput;
    let pos = positions[vid];
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.uv = uvs[vid];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSampleLevel(scene_tex, scene_samp, in.uv, 0.0);
}
"#;

pub const GLASS_SHADER: &str = r#"
struct Globals {
    screen_size: vec2<f32>,
    _pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var scene_tex: texture_2d<f32>;
@group(1) @binding(1) var scene_samp: sampler;

struct VertexInput {
    @location(0) corner: vec2<f32>,
}

struct GlassInstanceInput {
    @location(1) rect: vec4<f32>,
    @location(2) tint: vec4<f32>,
    @location(3) params: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) tint: vec4<f32>,
    @location(3) params: vec4<f32>,
    @location(4) scene_uv: vec2<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: GlassInstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let pos = instance.rect.xy + vertex.corner * instance.rect.zw;
    let ndc = (pos / globals.screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
    out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    out.local = vertex.corner * instance.rect.zw;
    out.size = instance.rect.zw;
    out.tint = instance.tint;
    out.params = instance.params;
    out.scene_uv = pos / globals.screen_size;
    return out;
}

fn sdf_rounded_rect(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let half = in.size * 0.5;
    let p = in.local - half;
    let radius = in.params.x;
    let dist = sdf_rounded_rect(p, half, radius);
    let fill_alpha = 1.0 - smoothstep(-1.0, 0.0, dist);
    if (fill_alpha <= 0.001) {
        discard;
    }
    let mip = in.params.z;
    let saturate = in.params.w;
    let tint_alpha = in.params.y;
    let blurred = textureSampleLevel(scene_tex, scene_samp, in.scene_uv, mip);
    let luma = dot(blurred.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let saturated = mix(vec3<f32>(luma), blurred.rgb, saturate);
    let rgb = mix(saturated, in.tint.rgb, tint_alpha);
    return vec4<f32>(rgb, fill_alpha);
}
"#;
// #endregion shaders
}

#[cfg(feature = "engine")]
pub mod text {
// #region text
//! 🖋️ Glyph atlas — fontdue when bytes are available, built-in bitmap fallback.

use std::collections::HashMap;

pub struct GlyphEntry {
    pub atlas_x: u32,
    pub atlas_y: u32,
    pub width: u32,
    pub height: u32,
    pub advance: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
}

enum FontSource {
    Fontdue(fontdue::Font),
    Bitmap,
}

pub struct FontAtlas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    source: FontSource,
    glyphs: HashMap<char, GlyphEntry>,
    cursor_x: u32,
    cursor_y: u32,
    row_height: u32,
    dirty: bool,
}

const BITMAP_GLYPH_W: u32 = 8;
const BITMAP_GLYPH_H: u32 = 16;

static BITMAP_FONT: [[u8; 8]; 95] = [
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x18, 0x3C, 0x3C, 0x18, 0x18, 0x00, 0x18, 0x00],
    [0x36, 0x36, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x36, 0x36, 0x7F, 0x36, 0x7F, 0x36, 0x36, 0x00],
    [0x0C, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x18, 0x00],
    [0x00, 0x63, 0x66, 0x0C, 0x18, 0x33, 0x63, 0x00],
    [0x1C, 0x36, 0x1C, 0x6E, 0x3B, 0x33, 0x6E, 0x00],
    [0x06, 0x06, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x18, 0x30, 0x60, 0x60, 0x60, 0x30, 0x18, 0x00],
    [0x06, 0x0C, 0x18, 0x18, 0x18, 0x0C, 0x06, 0x00],
    [0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00],
    [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30],
    [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00],
    [0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x80, 0x00],
    [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00],
    [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00],
    [0x3C, 0x66, 0x06, 0x1C, 0x30, 0x60, 0x7E, 0x00],
    [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00],
    [0x0C, 0x1C, 0x3C, 0x6C, 0x7E, 0x0C, 0x0C, 0x00],
    [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00],
    [0x1C, 0x30, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00],
    [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00],
    [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00],
    [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x38, 0x00],
    [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00],
    [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x30],
    [0x0C, 0x18, 0x30, 0x60, 0x30, 0x18, 0x0C, 0x00],
    [0x00, 0x00, 0x7E, 0x00, 0x7E, 0x00, 0x00, 0x00],
    [0x30, 0x18, 0x0C, 0x06, 0x0C, 0x18, 0x30, 0x00],
    [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x00, 0x18, 0x00],
    [0x3C, 0x66, 0x6E, 0x6A, 0x6E, 0x60, 0x3C, 0x00],
    [0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
    [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00],
    [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00],
    [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00],
    [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x7E, 0x00],
    [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x60, 0x00],
    [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00],
    [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
    [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
    [0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38, 0x00],
    [0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00],
    [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00],
    [0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00],
    [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00],
    [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
    [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00],
    [0x3C, 0x66, 0x66, 0x66, 0x6E, 0x6C, 0x3A, 0x00],
    [0x7C, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0x66, 0x00],
    [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00],
    [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
    [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
    [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
    [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00],
    [0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00],
    [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00],
    [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0x00],
    [0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, 0x00],
    [0xC0, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x02, 0x00],
    [0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0x00],
    [0x10, 0x38, 0x6C, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF],
    [0x30, 0x18, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x3C, 0x06, 0x3E, 0x66, 0x3E, 0x00],
    [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x7C, 0x00],
    [0x00, 0x00, 0x3C, 0x66, 0x60, 0x66, 0x3C, 0x00],
    [0x06, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x3E, 0x00],
    [0x00, 0x00, 0x3C, 0x66, 0x7E, 0x60, 0x3C, 0x00],
    [0x1C, 0x30, 0x7C, 0x30, 0x30, 0x30, 0x30, 0x00],
    [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x3C],
    [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
    [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x00],
    [0x0C, 0x00, 0x1C, 0x0C, 0x0C, 0x6C, 0x6C, 0x38],
    [0x60, 0x60, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0x00],
    [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
    [0x00, 0x00, 0x36, 0x7F, 0x6B, 0x6B, 0x63, 0x00],
    [0x00, 0x00, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
    [0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00],
    [0x00, 0x00, 0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60],
    [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x06],
    [0x00, 0x00, 0x7C, 0x66, 0x60, 0x60, 0x60, 0x00],
    [0x00, 0x00, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x00],
    [0x30, 0x30, 0x7C, 0x30, 0x30, 0x30, 0x1C, 0x00],
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x3E, 0x00],
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
    [0x00, 0x00, 0x63, 0x6B, 0x6B, 0x7F, 0x36, 0x00],
    [0x00, 0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x00],
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x3E, 0x06, 0x3C],
    [0x00, 0x00, 0x7E, 0x0C, 0x18, 0x30, 0x7E, 0x00],
    [0x0E, 0x18, 0x18, 0x70, 0x18, 0x18, 0x0E, 0x00],
    [0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
    [0x70, 0x18, 0x18, 0x0E, 0x18, 0x18, 0x70, 0x00],
    [0x31, 0x6B, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00],
];

impl FontAtlas {
    pub fn builtin() -> Self {
        Self {
            width: 2048,
            height: 2048,
            pixels: vec![0; 2048 * 2048],
            source: FontSource::Bitmap,
            glyphs: HashMap::new(),
            cursor_x: 1,
            cursor_y: 1,
            row_height: 0,
            dirty: false,
        }
    }

    pub fn take_dirty(&mut self) -> bool {
        let dirty = self.dirty;
        self.dirty = false;
        dirty
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.is_empty() {
            return Ok(Self::builtin());
        }
        let font = match fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default()) {
            Ok(font) => font,
            Err(_) => return Ok(Self::builtin()),
        };
        Ok(Self {
            width: 2048,
            height: 2048,
            pixels: vec![0; 2048 * 2048],
            source: FontSource::Fontdue(font),
            glyphs: HashMap::new(),
            cursor_x: 1,
            cursor_y: 1,
            row_height: 0,
            dirty: false,
        })
    }

    pub fn ensure_glyph(&mut self, ch: char) -> &GlyphEntry {
        if !self.glyphs.contains_key(&ch) {
            self.rasterize_glyph(ch);
        }
        self.glyphs.get(&ch).expect("glyph inserted")
    }

    fn rasterize_glyph(&mut self, ch: char) {
        let (metrics, bitmap, width, height) = match &self.source {
            FontSource::Fontdue(font) => {
                let (metrics, bitmap) = font.rasterize(ch, 16.0);
                (metrics, bitmap, metrics.width as u32, metrics.height as u32)
            }
            FontSource::Bitmap => self.rasterize_bitmap(ch),
        };
        if self.cursor_x + width + 2 >= self.width {
            self.cursor_x = 1;
            self.cursor_y += self.row_height + 2;
            self.row_height = 0;
        }
        let atlas_x = self.cursor_x;
        let atlas_y = self.cursor_y;
        for row in 0..height {
            let dst = ((atlas_y + row) * self.width + atlas_x) as usize;
            let src = (row * width) as usize;
            if !bitmap.is_empty() && width > 0 {
                self.pixels[dst..dst + width as usize]
                    .copy_from_slice(&bitmap[src..src + width as usize]);
            }
        }
        self.glyphs.insert(
            ch,
            GlyphEntry {
                atlas_x,
                atlas_y,
                width,
                height,
                advance: metrics.advance_width,
                bearing_x: metrics.xmin as f32,
                bearing_y: metrics.ymin as f32,
            },
        );
        self.cursor_x += width + 2;
        self.row_height = self.row_height.max(height);
        self.dirty = true;
    }

    fn rasterize_bitmap(&self, ch: char) -> (fontdue::Metrics, Vec<u8>, u32, u32) {
        let index = ch as u32;
        let glyph_index = if (32..127).contains(&index) {
            (index - 32) as usize
        } else {
            0
        };
        let pattern = &BITMAP_FONT[glyph_index.min(BITMAP_FONT.len() - 1)];
        let mut bitmap = vec![0u8; (BITMAP_GLYPH_W * BITMAP_GLYPH_H) as usize];
        for (row, row_bits) in pattern.iter().enumerate() {
            for col in 0..BITMAP_GLYPH_W {
                if (row_bits >> (7 - col)) & 1 == 1 {
                    bitmap[row * BITMAP_GLYPH_W as usize + col as usize] = 255;
                }
            }
        }
        let metrics = fontdue::Metrics {
            xmin: 0,
            ymin: 0,
            width: BITMAP_GLYPH_W as usize,
            height: BITMAP_GLYPH_H as usize,
            advance_width: BITMAP_GLYPH_W as f32 + 2.0,
            advance_height: BITMAP_GLYPH_H as f32,
            bounds: fontdue::OutlineBounds::default(),
        };
        (metrics, bitmap, BITMAP_GLYPH_W, BITMAP_GLYPH_H)
    }

    pub fn measure_text(&mut self, text: &str, size: f32) -> (f32, f32) {
        let scale = size / 16.0;
        let mut width = 0.0f32;
        let mut max_height = 0.0f32;
        for ch in text.chars() {
            let glyph = self.ensure_glyph(ch);
            width += glyph.advance * scale;
            max_height = max_height.max((glyph.height as f32 + glyph.bearing_y) * scale);
        }
        (width, max_height.max(size))
    }

    pub fn measure_text_wrapped(&mut self, text: &str, max_width: f32, size: f32) -> (f32, f32) {
        let mut lines = Vec::new();
        let mut current = String::new();
        for word in text.split_whitespace() {
            let trial = if current.is_empty() {
                word.to_string()
            } else {
                format!("{current} {word}")
            };
            let (w, _) = self.measure_text(&trial, size);
            if w > max_width && !current.is_empty() {
                lines.push(current);
                current = word.to_string();
            } else {
                current = trial;
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        let line_h = size * 1.35;
        let height = lines.len().max(1) as f32 * line_h;
        let width = lines
            .iter()
            .map(|line| self.measure_text(line, size).0)
            .fold(0.0f32, f32::max)
            .min(max_width);
        (width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::FontAtlas;

    #[test]
    fn from_bytes_uses_builtin_for_woff2_and_empty_input() {
        assert!(FontAtlas::from_bytes(&[]).is_ok());
        let woff2 = b"wOF2\x00\x01\x00\x00";
        let mut atlas = FontAtlas::from_bytes(woff2).expect("woff2 should fall back to builtin");
        let glyph = atlas.ensure_glyph('A');
        assert!(glyph.width > 0);
    }
}

pub async fn fetch_font_bytes(url: &str) -> Result<Vec<u8>, String> {
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Uint8Array;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init(url, &opts).map_err(|_| "request failed")?;
        let window = web_sys::window().ok_or("no window")?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| "fetch failed")?;
        let resp: Response = resp_value.dyn_into().map_err(|_| "response cast failed")?;
        if !resp.ok() {
            return Ok(Vec::new());
        }
        let buffer = JsFuture::from(resp.array_buffer().map_err(|_| "array_buffer failed")?)
            .await
            .map_err(|_| "buffer failed")?;
        let array = Uint8Array::new(&buffer);
        let mut bytes = vec![0u8; array.length() as usize];
        array.copy_to(&mut bytes);
        Ok(bytes)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = url;
        Ok(Vec::new())
    }
}
// #endregion text
}

pub mod theme {
// #region theme
//! 🎨 Theme colors and metrics for wgpu UI rendering.

use crate::geometry::Rect;
use ui_styling::{
    metrics::{chrome as chrome_metrics, dom, typography},
    opacities, radii, strokes, ChromePalette, CHROME_DARK, CHROME_LIGHT,
};
use ui_styling::appearance::AppearanceName;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_srgb8(r: u8, g: u8, b: u8, a: u8) -> Self {
        let [lr, lg, lb, la] = ui_styling::color::rgba8_to_linear(r, g, b, a);
        Self::new(lr, lg, lb, la)
    }

    fn from_chrome(c: &[f32; 4]) -> Self {
        Self::new(c[0], c[1], c[2], c[3])
    }

    pub fn with_alpha(self, a: f32) -> Self {
        Self::new(self.r, self.g, self.b, a)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassTier {
    Panel,
    Toolbar,
    Menu,
    WindowOptions,
}

#[derive(Clone, Copy, Debug)]
pub struct GlassStyle {
    pub tint: Rgba,
    pub alpha: f32,
    pub blur_px: f32,
    pub saturate: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub background: Rgba,
    pub panel: Rgba,
    pub panel_border: Rgba,
    pub navbar: Rgba,
    pub text: Rgba,
    pub text_muted: Rgba,
    pub accent: Rgba,
    pub accent_hover: Rgba,
    pub active_foreground: Rgba,
    pub button: Rgba,
    pub button_hover: Rgba,
    pub input_bg: Rgba,
    pub separator: Rgba,
    pub selected: Rgba,
    pub canvas_clear: Rgba,
    pub temporary: Rgba,
    pub gap_standard: f32,
    pub padding_standard: f32,
    pub navbar_height: f32,
    pub panel_header_height: f32,
    pub control_height: f32,
    pub control_height_small: f32,
    pub glass_saturate: f32,
    pub font_size_body: f32,
    pub font_size_small: f32,
    pub font_size_emphasized: f32,
    pub footer_height: f32,
    pub panel_inset: f32,
    pub panel_min_width: f32,
    pub panel_max_width: f32,
    pub window_measures_default_width: f32,
    pub window_engagement_max_width: f32,
    pub overlay_shadow: Rgba,
    pub focus_ring: Rgba,
    pub row_hover: Rgba,
    pub border_radius: f32,
    pub border_normal: Rgba,
    pub border_emphasized: Rgba,
    pub text_element: Rgba,
    pub stroke_hairline: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

fn chrome_px(ui_spacing_mult: f64) -> f32 {
    (chrome_metrics::UI_SPACING_COMPACT_PX * ui_spacing_mult) as f32
}

fn panel_width(ui_spacing_mult: f64) -> f32 {
    (chrome_metrics::UI_SPACING_COMPACT_PX * ui_spacing_mult) as f32
}

fn from_chrome(chrome: &ChromePalette) -> Theme {
    Theme {
        background: Rgba::from_chrome(&chrome.canvas),
        panel: Rgba::from_chrome(&chrome.panel),
        panel_border: Rgba::from_chrome(&chrome.border_normal),
        navbar: Rgba::from_chrome(&chrome.window),
        text: Rgba::from_chrome(&chrome.foreground),
        text_muted: Rgba::from_chrome(&chrome.muted_foreground),
        accent: Rgba::from_chrome(&chrome.accent),
        accent_hover: Rgba::from_chrome(&chrome.active_hover),
        active_foreground: Rgba::from_chrome(&chrome.active_foreground),
        button: Rgba::from_chrome(&chrome.window),
        button_hover: Rgba::from_chrome(&chrome.hover_interactive_fill),
        input_bg: Rgba::from_chrome(&chrome.canvas),
        separator: Rgba::from_chrome(&chrome.border_normal),
        selected: Rgba::from_chrome(&chrome.active_base),
        canvas_clear: Rgba::from_chrome(&chrome.canvas),
        temporary: Rgba::from_chrome(&chrome.temporary),
        gap_standard: chrome_px(chrome_metrics::GAP_STANDARD_UI_SPACING),
        padding_standard: chrome_px(chrome_metrics::PADDING_STANDARD_UI_SPACING),
        navbar_height: chrome_px(chrome_metrics::NAVBAR_HEIGHT_UI_SPACING),
        panel_header_height: chrome_px(chrome_metrics::PANEL_HEADER_HEIGHT_UI_SPACING),
        control_height: chrome_px(chrome_metrics::CONTROL_HEIGHT_UI_SPACING),
        control_height_small: chrome_px(5.0),
        glass_saturate: chrome_metrics::GLASS_SATURATE as f32,
        font_size_body: typography::TEXT_SM_PX as f32,
        font_size_small: typography::TEXT_XS_PX as f32,
        font_size_emphasized: typography::TEXT_BASE_PX as f32,
        footer_height: chrome_px(chrome_metrics::FOOTER_HEIGHT_UI_SPACING),
        panel_inset: chrome_px(chrome_metrics::PANEL_INSET_UI_SPACING),
        panel_min_width: panel_width(dom::LAYOUT_PANEL_MIN_UI_SPACING),
        panel_max_width: panel_width(dom::LAYOUT_PANEL_MAX_UI_SPACING),
        window_measures_default_width: chrome_px(dom::LAYOUT_PANEL_RAIL_UI_SPACING),
        window_engagement_max_width: chrome_px(dom::LAYOUT_ENGAGEMENT_MAX_UI_SPACING),
        overlay_shadow: Rgba::new(0.0, 0.0, 0.0, 0.0),
        focus_ring: Rgba::from_chrome(&chrome.accent).with_alpha(0.6),
        row_hover: Rgba::from_chrome(&chrome.hover_interactive_fill),
        border_radius: radii::CHROME as f32,
        border_normal: Rgba::from_chrome(&chrome.border_normal),
        border_emphasized: Rgba::from_chrome(&chrome.border_emphasized),
        text_element: Rgba::from_chrome(&chrome.border_element),
        stroke_hairline: strokes::CHROME_BORDER_HAIRLINE as f32,
    }
}

impl Theme {
    pub fn light() -> Self {
        from_chrome(&CHROME_LIGHT)
    }

    pub fn dark() -> Self {
        from_chrome(&CHROME_DARK)
    }

    pub fn for_name(name: AppearanceName) -> Self {
        match name {
            AppearanceName::Light => Self::light(),
            AppearanceName::Dark => Self::dark(),
        }
    }

    pub fn glass(&self, tier: GlassTier) -> GlassStyle {
        match tier {
            GlassTier::Panel => GlassStyle {
                tint: self.panel,
                alpha: opacities::GLASS_PANEL_ALPHA as f32,
                blur_px: chrome_metrics::GLASS_PANEL_BLUR_PX as f32,
                saturate: self.glass_saturate,
            },
            GlassTier::Toolbar => GlassStyle {
                tint: self.panel,
                alpha: 0.3,
                blur_px: chrome_metrics::GLASS_BLUR_PX as f32,
                saturate: self.glass_saturate,
            },
            GlassTier::Menu => GlassStyle {
                tint: self.temporary,
                alpha: opacities::GLASS_MENU_ALPHA as f32,
                blur_px: chrome_metrics::GLASS_BLUR_PX as f32,
                saturate: self.glass_saturate,
            },
            GlassTier::WindowOptions => GlassStyle {
                tint: self.panel,
                alpha: opacities::GLASS_WINDOW_OPTIONS_ALPHA as f32,
                blur_px: chrome_metrics::GLASS_WINDOW_OPTIONS_BLUR_PX as f32,
                saturate: self.glass_saturate,
            },
        }
    }

    pub fn glass_mip_level(blur_px: f32, max_mip: u32) -> f32 {
        (blur_px / 4.0).log2().max(0.0).min(max_mip as f32)
    }
}

pub type ThemedRect = Rect;

#[cfg(test)]
mod tests {
    use super::{GlassTier, Theme};
    use ui_styling::color::linear_to_rgba8;

    #[test]
    fn light_window_token_matches_react_navbar_hex() {
        let theme = Theme::light();
        let [r, g, b, _] = linear_to_rgba8(theme.navbar.r, theme.navbar.g, theme.navbar.b, theme.navbar.a);
        assert_eq!([r, g, b], [235, 232, 217]);
    }

    #[test]
    fn light_canvas_token_matches_react_canvas_hex() {
        let theme = Theme::light();
        let [r, g, b, _] = linear_to_rgba8(theme.canvas_clear.r, theme.canvas_clear.g, theme.canvas_clear.b, theme.canvas_clear.a);
        assert_eq!([r, g, b], [240, 236, 221]);
    }

    #[test]
    fn glass_panel_tier_matches_react_tokens() {
        let theme = Theme::light();
        let glass = theme.glass(GlassTier::Panel);
        let [r, g, b, _] = linear_to_rgba8(glass.tint.r, glass.tint.g, glass.tint.b, glass.tint.a);
        assert_eq!([r, g, b], [201, 200, 189]);
        assert!((glass.alpha - 0.58).abs() < f32::EPSILON);
        assert!((glass.blur_px - 40.0).abs() < f32::EPSILON);
        assert!((glass.saturate - 1.45).abs() < f32::EPSILON);
    }

    #[test]
    fn glass_menu_tier_uses_temporary_tint() {
        let theme = Theme::light();
        let glass = theme.glass(GlassTier::Menu);
        let [r, g, b, _] = linear_to_rgba8(glass.tint.r, glass.tint.g, glass.tint.b, glass.tint.a);
        assert_eq!([r, g, b], [151, 155, 148]);
        assert!((glass.alpha - 0.36).abs() < f32::EPSILON);
        assert!((glass.blur_px - 24.0).abs() < f32::EPSILON);
    }

    #[test]
    fn glass_window_options_tier_matches_react_tokens() {
        let theme = Theme::light();
        let glass = theme.glass(GlassTier::WindowOptions);
        assert!((glass.alpha - 0.22).abs() < f32::EPSILON);
        assert!((glass.blur_px - 14.0).abs() < f32::EPSILON);
    }

    #[test]
    fn window_rail_widths_match_react_dom_tokens() {
        let theme = Theme::light();
        assert!((theme.window_measures_default_width - 224.0).abs() < f32::EPSILON);
        assert!((theme.window_engagement_max_width - 448.0).abs() < f32::EPSILON);
    }

    #[test]
    fn chrome_item_default_is_transparent() {
        use crate::chrome::chrome_item_bg;
        let theme = Theme::light();
        let bg = chrome_item_bg(&theme, false, false);
        assert_eq!(bg.a, 0.0);
    }
}
// #endregion theme
}

#[cfg(feature = "engine")]
pub mod widgets {
// #region widgets
//! 🧩 Generic widget tree — layout, measurement, and drawing.

use crate::chrome::{chrome_item_bg, item_bg, item_text, push_control_border, push_icon, ICON_TINY};
use crate::draw::{DrawList, IconAtlas};
use crate::geometry::Rect;
use crate::input::{DragAxis, HitKind, HitTarget, InputState};
use crate::layout::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
use crate::text::FontAtlas;
use crate::theme::{GlassTier, Rgba, Theme};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct InputMeta<E> {
    pub on_change: E,
    pub commit: Option<String>,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct SliderMeta<E> {
    pub on_change: E,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: f64,
    pub bounds_x: f32,
    pub bounds_w: f32,
}

#[derive(Clone, Debug)]
pub struct StepperMeta<E> {
    pub on_absolute: E,
    pub on_delta: E,
    pub step: f64,
    pub value: f64,
}

#[derive(Clone, Debug)]
pub struct RingMeta<E> {
    pub on_change: E,
    pub disabled: bool,
    pub center_x: f32,
    pub center_y: f32,
    pub radius: f32,
}

#[derive(Clone, Debug)]
pub struct Vec3Meta<E> {
    pub on_change: E,
    pub value: [f64; 3],
}

pub struct WidgetInteractionMaps<E> {
    pub input_metas: HashMap<String, InputMeta<E>>,
    pub select_metas: HashMap<String, E>,
    pub toggle_metas: HashMap<String, (bool, E)>,
    pub slider_metas: HashMap<String, SliderMeta<E>>,
    pub stepper_metas: HashMap<String, StepperMeta<E>>,
    pub ring_metas: HashMap<String, RingMeta<E>>,
    pub vec3_metas: HashMap<String, Vec3Meta<E>>,
    pub slider_live_values: HashMap<String, f64>,
    pub ring_live_values: HashMap<String, f64>,
    pub tree_hover_commands: HashMap<String, E>,
    pub tree_unhover_commands: HashMap<String, E>,
    pub tree_selection_change: Option<E>,
}

impl<E> Default for WidgetInteractionMaps<E> {
    fn default() -> Self {
        Self {
            input_metas: HashMap::new(),
            select_metas: HashMap::new(),
            toggle_metas: HashMap::new(),
            slider_metas: HashMap::new(),
            stepper_metas: HashMap::new(),
            ring_metas: HashMap::new(),
            vec3_metas: HashMap::new(),
            slider_live_values: HashMap::new(),
            ring_live_values: HashMap::new(),
            tree_hover_commands: HashMap::new(),
            tree_unhover_commands: HashMap::new(),
            tree_selection_change: None,
        }
    }
}

impl<E> WidgetInteractionMaps<E> {
    pub fn clear_frame(&mut self) {
        self.input_metas.clear();
        self.select_metas.clear();
        self.toggle_metas.clear();
        self.slider_metas.clear();
        self.stepper_metas.clear();
        self.ring_metas.clear();
        self.vec3_metas.clear();
        self.slider_live_values.clear();
        self.ring_live_values.clear();
        self.tree_hover_commands.clear();
        self.tree_unhover_commands.clear();
        self.tree_selection_change = None;
    }
}

pub struct WidgetContext<'a, E> {
    pub draw: &'a mut DrawList,
    pub overlay: Option<&'a mut DrawList>,
    pub atlas: &'a mut FontAtlas,
    pub icons: Option<&'a IconAtlas>,
    pub input: &'a mut InputState<E>,
    pub theme: &'a Theme,
    pub scroll_offsets: &'a mut HashMap<String, f32>,
    pub collapsed_sections: &'a mut HashMap<String, bool>,
    pub open_selects: &'a mut HashMap<String, bool>,
    pub interaction_maps: Option<&'a mut WidgetInteractionMaps<E>>,
    pub pick_clip: Option<crate::geometry::Rect>,
}

#[derive(Clone, Debug)]
pub struct SelectItem {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct KeyValueEntry {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct TreeItemAction<E> {
    pub icon_id: String,
    pub label: Option<String>,
    pub event: E,
    pub reveal_on_hover: bool,
}

#[derive(Clone, Debug)]
pub struct TreeItem<E> {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub icon_id: Option<String>,
    pub selected: bool,
    pub highlighted: bool,
    pub default_open: bool,
    pub is_hidden: bool,
    pub event: Option<E>,
    pub hover_event: Option<E>,
    pub unhover_event: Option<E>,
    pub actions: Vec<TreeItemAction<E>>,
    pub draggable: bool,
    pub drag_data: HashMap<String, String>,
    pub control: Option<Box<WidgetNode<E>>>,
    pub children: Vec<TreeItem<E>>,
}

#[derive(Clone, Debug)]
pub struct TreeSection<E> {
    pub id: String,
    pub label: Option<String>,
    pub default_open: bool,
    pub items: Vec<TreeItem<E>>,
}

#[derive(Clone, Debug)]
pub enum ControlNode<E> {
    Button { id: Option<String>, icon_id: Option<String>, label: String, event: Option<E> },
    Input {
        id: String,
        input_kind: String,
        value: String,
        placeholder: Option<String>,
        commit: Option<String>,
        on_change: Option<E>,
    },
    Select {
        id: String,
        value: String,
        items: Vec<SelectItem>,
        placeholder: Option<String>,
        on_change: Option<E>,
    },
    Toggle { id: String, icon_id: String, pressed: bool, text: Option<String>, on_change: Option<E> },
    Vec3 { id: String, value: Option<[f64; 3]>, on_change: Option<E> },
    KeyValue { entries: Vec<KeyValueEntry> },
    Slider { id: String, value: f64, min: f64, max: f64, step: f64, on_change: Option<E> },
    NumberStepper {
        id: String,
        value: f64,
        step: f64,
        uniform: bool,
        on_absolute: Option<E>,
        on_delta: Option<E>,
    },
    Ring { id: String, t: f64, disabled: bool, on_change: Option<E> },
    IconSelect { id: String, value: String, uniform: bool, classifier_kind: String, on_change: Option<E> },
}

#[derive(Clone, Debug)]
pub enum WidgetNode<E> {
    Stack {
        direction: String,
        gap: Option<String>,
        padding: Option<String>,
        children: Vec<WidgetNode<E>>,
    },
    Text { value: String, emphasize: bool },
    Separator,
    Button { id: Option<String>, icon_id: Option<String>, label: String, event: Option<E> },
    Input {
        id: String,
        input_kind: String,
        value: String,
        placeholder: Option<String>,
        commit: Option<String>,
        on_change: Option<E>,
    },
    Select {
        id: String,
        value: String,
        items: Vec<SelectItem>,
        placeholder: Option<String>,
        on_change: Option<E>,
    },
    Toggle { id: String, icon_id: String, pressed: bool, text: Option<String>, on_change: Option<E> },
    Vec3 { id: String, value: Option<[f64; 3]>, on_change: Option<E> },
    KeyValue { entries: Vec<KeyValueEntry> },
    Slider { id: String, value: f64, min: f64, max: f64, step: f64, on_change: Option<E> },
    NumberStepper {
        id: String,
        value: f64,
        step: f64,
        uniform: bool,
        on_absolute: Option<E>,
        on_delta: Option<E>,
    },
    Ring { id: String, t: f64, disabled: bool, on_change: Option<E> },
    IconSelect { id: String, value: String, uniform: bool, classifier_kind: String, on_change: Option<E> },
    Field { id: String, label: String, child: ControlNode<E> },
    Section { id: String, label: Option<String>, default_open: bool, children: Vec<WidgetNode<E>> },
    Tree {
        sections: Vec<TreeSection<E>>,
        selected_ids: Vec<String>,
        highlighted_ids: Vec<String>,
        selection_change: Option<E>,
    },
}

const PANEL_HEADER: f32 = 24.0;
const TREE_ROW_HEIGHT: f32 = 24.0;
const TREE_INDENT_PER_LEVEL: f32 = 10.0;
const TREE_TOGGLE_WIDTH: f32 = 14.0;
const TREE_ICON_SIZE: f32 = 14.0;
const TREE_SECTION_GAP: f32 = 8.0;

pub fn measure_widget<E>(atlas: &mut FontAtlas, theme: &Theme, node: &WidgetNode<E>) -> (f32, f32) {
    match node {
        WidgetNode::Stack { direction, gap, padding, children } => {
            let gap = gap_for_token(theme, gap.as_deref());
            let padding = padding_for_token(theme, padding.as_deref()) * 2.0;
            let vertical = direction != "horizontal";
            let mut total_main = 0.0f32;
            let mut max_cross = 0.0f32;
            for (index, child) in children.iter().enumerate() {
                let (w, h) = measure_widget(atlas, theme, child);
                if vertical {
                    total_main += h;
                    max_cross = max_cross.max(w);
                    if index + 1 < children.len() {
                        total_main += gap;
                    }
                } else {
                    total_main += w;
                    max_cross = max_cross.max(h);
                    if index + 1 < children.len() {
                        total_main += gap;
                    }
                }
            }
            if vertical {
                (max_cross + padding, total_main + padding)
            } else {
                (total_main + padding, max_cross + padding)
            }
        }
        WidgetNode::Text { value, emphasize } => {
            let size = if *emphasize { theme.font_size_emphasized } else { theme.font_size_body };
            let (w, _) = atlas.measure_text(value, size);
            let lines = wrap_text(atlas, value, w.max(120.0), size);
            (w.max(120.0), lines.len() as f32 * size * 1.35)
        }
        WidgetNode::Separator => (theme.control_height.max(1.0), 1.0 + theme.gap_standard),
        WidgetNode::Button { .. } | WidgetNode::Input { .. } | WidgetNode::Select { .. }
        | WidgetNode::Toggle { .. } | WidgetNode::Slider { .. } | WidgetNode::NumberStepper { .. }
        | WidgetNode::IconSelect { .. } => (theme.control_height, theme.control_height),
        WidgetNode::Vec3 { .. } => (theme.control_height, theme.control_height * 3.0 + theme.gap_standard * 2.0),
        WidgetNode::KeyValue { entries } => {
            let label_w = entries
                .iter()
                .map(|e| atlas.measure_text(&e.label, theme.font_size_small).0)
                .fold(0.0f32, f32::max);
            (label_w + theme.gap_standard * 2.0 + 80.0, entries.len() as f32 * theme.control_height)
        }
        WidgetNode::Ring { .. } => (80.0, 80.0),
        WidgetNode::Field { label, child, .. } => {
            let label_h = theme.font_size_small;
            let gap = gap_for_token(theme, Some("standard"));
            let (cw, ch) = measure_control(atlas, theme, child);
            (cw.max(atlas.measure_text(label, theme.font_size_small).0), label_h + gap + ch)
        }
        WidgetNode::Section { children, label, .. } => {
            let mut height = PANEL_HEADER;
            let mut max_w = 0.0f32;
            if label.is_some() {
                max_w = max_w.max(160.0);
            }
            for child in children {
                let (w, h) = measure_widget(atlas, theme, child);
                max_w = max_w.max(w);
                height += h + theme.gap_standard;
            }
            (max_w.max(120.0), height)
        }
        WidgetNode::Tree { sections, .. } => (measure_tree_sections_width(sections, atlas, theme), measure_tree_sections(sections)),
    }
}

fn measure_control<E>(atlas: &mut FontAtlas, theme: &Theme, control: &ControlNode<E>) -> (f32, f32) {
    match control {
        ControlNode::Button { .. } | ControlNode::Input { .. } | ControlNode::Select { .. }
        | ControlNode::Toggle { .. } | ControlNode::Slider { .. } | ControlNode::NumberStepper { .. }
        | ControlNode::IconSelect { .. } => (theme.control_height, theme.control_height),
        ControlNode::Vec3 { .. } => (theme.control_height, theme.control_height * 3.0 + theme.gap_standard * 2.0),
        ControlNode::KeyValue { entries } => {
            let label_w = entries
                .iter()
                .map(|e| atlas.measure_text(&e.label, theme.font_size_small).0)
                .fold(0.0f32, f32::max);
            (label_w + theme.gap_standard * 2.0 + 80.0, entries.len() as f32 * theme.control_height)
        }
        ControlNode::Ring { .. } => (80.0, 80.0),
    }
}

pub fn render_widget<E: Clone>(
    node: &WidgetNode<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    match node {
        WidgetNode::Stack { direction, gap, padding, children } => {
            let gap = gap_for_token(ctx.theme, gap.as_deref());
            let padding = padding_for_token(ctx.theme, padding.as_deref());
            let vertical = direction != "horizontal";
            let sizes: Vec<f32> = children
                .iter()
                .map(|child| {
                    let (w, h) = measure_widget(ctx.atlas, ctx.theme, child);
                    if vertical { h } else { w }
                })
                .collect();
            let rects = if vertical {
                layout_vertical(bounds, gap, padding, &sizes)
            } else {
                layout_horizontal(bounds, gap, padding, &sizes)
            };
            for (child, rect) in children.iter().zip(rects.iter()) {
                render_widget(child, *rect, ctx);
            }
        }
        WidgetNode::Text { value, emphasize } => {
            let size = if *emphasize { ctx.theme.font_size_emphasized } else { ctx.theme.font_size_body };
            let color = if *emphasize { ctx.theme.text } else { ctx.theme.text_muted };
            draw_text_wrapped(ctx, value, bounds.x, bounds.y, bounds.w.max(1.0), size, color);
        }
        WidgetNode::Separator => {
            let y = bounds.y + bounds.h * 0.5;
            ctx.draw.push_line(bounds.x, y, bounds.x + bounds.w, y, ctx.theme.separator, 1.0);
        }
        WidgetNode::Button { id, icon_id, label, event } => {
            render_button(id.clone(), icon_id.as_deref(), label, event.clone(), bounds, ctx)
        }
        WidgetNode::Input { id, value, placeholder, commit, on_change, .. } => {
            register_input_meta(ctx, id, value, commit.clone(), on_change.clone());
            render_input(id, value, placeholder.as_deref(), bounds, ctx);
        }
        WidgetNode::Select { id, value, items, placeholder, on_change } => {
            register_select_meta(ctx, id, on_change.clone());
            render_select(id, value, items, placeholder.as_deref(), bounds, ctx);
        }
        WidgetNode::Toggle { id, icon_id, pressed, text, on_change } => {
            register_toggle_meta(ctx, id, *pressed, on_change.clone());
            render_toggle(id, icon_id, *pressed, text.as_deref(), bounds, ctx);
        }
        WidgetNode::Vec3 { id, value, on_change } => render_vec3(id, *value, on_change.clone(), bounds, ctx),
        WidgetNode::KeyValue { entries } => render_key_value(entries, bounds, ctx),
        WidgetNode::Slider { id, value, min, max, step, on_change } => {
            render_slider(id, *value, *min, *max, *step, on_change.clone(), bounds, ctx)
        }
        WidgetNode::NumberStepper { id, value, step, uniform, on_absolute, on_delta } => {
            render_number_stepper(id, *value, *step, *uniform, on_absolute.clone(), on_delta.clone(), bounds, ctx)
        }
        WidgetNode::Ring { id, t, disabled, on_change } => {
            render_ring(id, *t, *disabled, on_change.clone(), bounds, ctx)
        }
        WidgetNode::IconSelect { id, value, uniform, classifier_kind, on_change } => {
            render_icon_select(id, value, *uniform, classifier_kind, on_change.clone(), bounds, ctx)
        }
        WidgetNode::Field { label, child, .. } => {
            let label_h = ctx.theme.font_size_small;
            let gap = gap_for_token(ctx.theme, Some("standard"));
            draw_text(ctx, label, bounds.x, bounds.y + label_h, ctx.theme.font_size_small, ctx.theme.text_muted);
            let child_bounds = Rect::new(bounds.x, bounds.y + label_h + gap, bounds.w, bounds.h - label_h - gap);
            render_control(child, child_bounds, ctx);
        }
        WidgetNode::Section { label, children, id, default_open } => {
            let section_key = format!("section.{id}");
            if !ctx.collapsed_sections.contains_key(&section_key) {
                ctx.collapsed_sections.insert(section_key.clone(), !default_open);
            }
            let collapsed = tree_row_collapsed(ctx.collapsed_sections, &section_key, *default_open);
            if label.is_some() {
                let header = Rect::new(bounds.x, bounds.y, bounds.w, PANEL_HEADER);
                let chevron_rect = Rect::new(bounds.x, bounds.y, TREE_TOGGLE_WIDTH, PANEL_HEADER);
                let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
                tree_draw_chevron(ctx, chevron, chevron_rect);
                if let Some(label) = label {
                    draw_text(
                        ctx,
                        label,
                        bounds.x + TREE_TOGGLE_WIDTH + ctx.theme.gap_standard,
                        bounds.y + (PANEL_HEADER + ctx.theme.font_size_body) * 0.5 - 2.0,
                        ctx.theme.font_size_body,
                        ctx.theme.text,
                    );
                }
                ctx.input.register_hit(HitTarget {
                    rect: header,
                    event: None,
                    control_id: Some(format!("section.chevron.{id}")),
                    kind: HitKind::Generic,
                    drag_axis: None,
                    drag_data: None,
                });
            }
            if !collapsed {
                let mut y = bounds.y + PANEL_HEADER;
                for child in children {
                    let (_, h) = measure_widget(ctx.atlas, ctx.theme, child);
                    let child_bounds = Rect::new(bounds.x, y, bounds.w, h);
                    render_widget(child, child_bounds, ctx);
                    y += h + ctx.theme.gap_standard;
                }
            }
        }
        WidgetNode::Tree {
            sections,
            selected_ids,
            highlighted_ids,
            selection_change,
        } => {
            if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
                maps.tree_selection_change = selection_change.clone();
            }
            let scroll_id = format!("tree:{:.0}:{:.0}", bounds.x, bounds.y);
            let content_h = measure_tree_sections_state(sections, ctx.collapsed_sections);
            render_scroll_region(&scroll_id, bounds, content_h.max(bounds.h), ctx, |content, ctx| {
                render_tree(sections, selected_ids, highlighted_ids, content, ctx);
            });
        }
    }
}

fn render_control<E: Clone>(control: &ControlNode<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    match control {
        ControlNode::Button { id, icon_id, label, event } => {
            render_button(id.clone(), icon_id.as_deref(), label, event.clone(), bounds, ctx)
        }
        ControlNode::Input { id, value, placeholder, commit, on_change, .. } => {
            register_input_meta(ctx, id, value, commit.clone(), on_change.clone());
            render_input(id, value, placeholder.as_deref(), bounds, ctx);
        }
        ControlNode::Select { id, value, items, placeholder, on_change } => {
            register_select_meta(ctx, id, on_change.clone());
            render_select(id, value, items, placeholder.as_deref(), bounds, ctx);
        }
        ControlNode::Toggle { id, icon_id, pressed, text, on_change } => {
            register_toggle_meta(ctx, id, *pressed, on_change.clone());
            render_toggle(id, icon_id, *pressed, text.as_deref(), bounds, ctx);
        }
        ControlNode::Vec3 { id, value, on_change } => render_vec3(id, *value, on_change.clone(), bounds, ctx),
        ControlNode::KeyValue { entries } => render_key_value(entries, bounds, ctx),
        ControlNode::Slider { id, value, min, max, step, on_change } => {
            render_slider(id, *value, *min, *max, *step, on_change.clone(), bounds, ctx)
        }
        ControlNode::NumberStepper { id, value, step, uniform, on_absolute, on_delta } => {
            render_number_stepper(id, *value, *step, *uniform, on_absolute.clone(), on_delta.clone(), bounds, ctx)
        }
        ControlNode::Ring { id, t, disabled, on_change } => render_ring(id, *t, *disabled, on_change.clone(), bounds, ctx),
        ControlNode::IconSelect { id, value, uniform, classifier_kind, on_change } => {
            render_icon_select(id, value, *uniform, classifier_kind, on_change.clone(), bounds, ctx)
        }
    }
}

fn register_input_meta<E: Clone>(
    ctx: &mut WidgetContext<'_, E>,
    id: &str,
    value: &str,
    commit: Option<String>,
    on_change: Option<E>,
) {
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        maps.input_metas.insert(
            id.to_string(),
            InputMeta {
                on_change,
                commit,
                value: value.to_string(),
            },
        );
    }
}

fn register_select_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, on_change: Option<E>) {
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        maps.select_metas.insert(id.to_string(), on_change);
    }
}

fn register_toggle_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, pressed: bool, on_change: Option<E>) {
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        maps.toggle_metas.insert(id.to_string(), (pressed, on_change));
    }
}

fn render_button<E: Clone>(
    id: Option<String>,
    icon_id: Option<&str>,
    label: &str,
    event: Option<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let control_id = id.clone().or_else(|| Some(label.to_string()));
    let hovered = ctx.input.hovered_id == control_id;
    let bg = item_bg(ctx.theme, false, hovered);
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let mut text_x = bounds.x + ctx.theme.padding_standard;
    let icon_key = icon_id.filter(|id| !id.is_empty()).unwrap_or(label);
    if let Some(icons) = ctx.icons {
        if icons.icon_uv(icon_key).is_some() {
            push_icon(
                ctx.draw,
                icons,
                icon_key,
                text_x,
                bounds.y + (bounds.h - ICON_TINY) * 0.5,
                ICON_TINY,
                item_text(ctx.theme, false, hovered),
            );
            text_x += ICON_TINY + ctx.theme.gap_standard;
        }
    }
    draw_text(
        ctx,
        label,
        text_x,
        bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
        ctx.theme.font_size_body,
        item_text(ctx.theme, false, hovered),
    );
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event,
        control_id,
        kind: HitKind::Button,
        drag_axis: None,
        drag_data: None,
    });
}

fn render_input<E: Clone>(id: &str, value: &str, placeholder: Option<&str>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let focused = ctx.input.focused_id.as_deref() == Some(id);
    let border = if focused {
        ctx.theme.border_emphasized
    } else {
        ctx.theme.border_normal
    };
    push_control_border(ctx.draw, bounds, ctx.theme, border, ctx.theme.input_bg);
    let (display, muted) = if focused {
        (ctx.input.text_buffer.clone(), false)
    } else if value.is_empty() {
        (placeholder.unwrap_or("").to_string(), true)
    } else {
        (value.to_string(), false)
    };
    draw_text(
        ctx,
        &display,
        bounds.x + 8.0,
        bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
        ctx.theme.font_size_body,
        if muted { ctx.theme.text_muted } else { ctx.theme.text },
    );
    if focused {
        let cursor_x = bounds.x + 8.0 + measure_text_width(ctx, &display[..ctx.input.cursor_pos.min(display.len())], ctx.theme.font_size_body);
        ctx.draw.push_solid([cursor_x, bounds.y + 6.0, 1.0, bounds.h - 12.0], ctx.theme.text);
    }
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event: None,
        control_id: Some(id.to_string()),
        kind: HitKind::Input,
        drag_axis: None,
        drag_data: None,
    });
}

fn render_select<E: Clone>(
    id: &str,
    value: &str,
    items: &[SelectItem],
    placeholder: Option<&str>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let open = *ctx.open_selects.get(id).unwrap_or(&false);
    let hovered = ctx.input.hovered_id.as_deref() == Some(id);
    let bg = if hovered {
        ctx.theme.button_hover
    } else {
        ctx.theme.input_bg
    };
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let label = items
        .iter()
        .find(|item| item.value == value)
        .map(|item| item.label.as_str())
        .unwrap_or(placeholder.unwrap_or("Select…"));
    draw_text(
        ctx,
        label,
        bounds.x + ctx.theme.padding_standard,
        bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
        ctx.theme.font_size_body,
        ctx.theme.text,
    );
    if let Some(icons) = ctx.icons {
        push_icon(
            ctx.draw,
            icons,
            "chevron-down",
            bounds.x + bounds.w - ctx.theme.padding_standard - ICON_TINY,
            bounds.y + (bounds.h - ICON_TINY) * 0.5,
            ICON_TINY,
            ctx.theme.text_element,
        );
    }
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event: None,
        control_id: Some(id.to_string()),
        kind: HitKind::Select,
        drag_axis: None,
        drag_data: None,
    });
    if open {
        render_select_menu(id, value, items, bounds, ctx);
    }
}

fn render_select_menu<E: Clone>(
    id: &str,
    value: &str,
    items: &[SelectItem],
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let item_h = ctx.theme.control_height;
    let menu_h = items.len() as f32 * item_h + 4.0;
    let menu = Rect::new(bounds.x, bounds.y + bounds.h + 2.0, bounds.w, menu_h);
    let mut render_rows = |draw: &mut DrawList| {
        draw.push_glass([menu.x, menu.y, menu.w, menu.h], ctx.theme.border_radius, GlassTier::Menu, ctx.theme);
        for (index, item) in items.iter().enumerate() {
            let row = Rect::new(menu.x + 2.0, menu.y + 2.0 + index as f32 * item_h, menu.w - 4.0, item_h);
            let row_hovered = ctx.input.hit_at(ctx.input.pointer_x, ctx.input.pointer_y)
                .and_then(|h| h.control_id.as_deref()) == Some(&format!("{id}.item.{}", item.value));
            if row_hovered || item.value == value {
                draw.push_rounded([row.x, row.y, row.w, row.h], ctx.theme.row_hover, ctx.theme.border_radius);
            }
            draw_text_on(draw, ctx.atlas, &item.label, row.x + 8.0, row.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
            ctx.input.register_hit(HitTarget {
                rect: row,
                event: None,
                control_id: Some(format!("{id}.item.{}", item.value)),
                kind: HitKind::DropdownItem,
                drag_axis: None,
                drag_data: None,
            });
        }
    };
    if let Some(overlay) = ctx.overlay.as_deref_mut() {
        render_rows(overlay);
    } else {
        render_rows(ctx.draw);
    }
}

fn render_toggle<E: Clone>(
    id: &str,
    icon_id: &str,
    pressed: bool,
    text: Option<&str>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let hovered = ctx.input.hovered_id.as_deref() == Some(id);
    let bg = item_bg(ctx.theme, pressed, hovered);
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let mut content_x = bounds.x + ctx.theme.padding_standard;
    if let Some(icons) = ctx.icons {
        if icons.icon_uv(icon_id).is_some() {
            push_icon(
                ctx.draw,
                icons,
                icon_id,
                content_x,
                bounds.y + (bounds.h - ICON_TINY) * 0.5,
                ICON_TINY,
                item_text(ctx.theme, pressed, hovered),
            );
            content_x += ICON_TINY + ctx.theme.gap_standard;
        }
    }
    if let Some(text) = text {
        draw_text(
            ctx,
            text,
            content_x,
            bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
            ctx.theme.font_size_body,
            item_text(ctx.theme, pressed, hovered),
        );
    }
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event: None,
        control_id: Some(id.to_string()),
        kind: HitKind::Toggle,
        drag_axis: None,
        drag_data: None,
    });
}

fn render_vec3<E: Clone>(id: &str, value: Option<[f64; 3]>, on_change: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let values = value.unwrap_or([0.0, 0.0, 0.0]);
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change.clone()) {
        maps.vec3_metas.insert(id.to_string(), Vec3Meta { on_change, value: values });
    }
    let gap = ctx.theme.gap_standard;
    let seg_w = (bounds.w - gap * 2.0) / 3.0;
    let labels = ["X", "Y", "Z"];
    for (index, axis) in labels.iter().enumerate() {
        let x = bounds.x + index as f32 * (seg_w + gap);
        let row = Rect::new(x, bounds.y, seg_w, bounds.h);
        let input_id = format!("{id}.{index}");
        let text = format!("{:.3}", values[index]);
        register_input_meta(ctx, &input_id, &text, None, None);
        render_input(&input_id, &text, Some(axis), row, ctx);
    }
}

fn render_key_value<E>(entries: &[KeyValueEntry], bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let label_w = entries
        .iter()
        .map(|e| measure_text_width(ctx, &e.label, ctx.theme.font_size_small))
        .fold(0.0f32, f32::max);
    let value_x = bounds.x + label_w + ctx.theme.gap_standard * 2.0;
    let row_h = ctx.theme.control_height;
    for (index, entry) in entries.iter().enumerate() {
        let y = bounds.y + index as f32 * row_h;
        draw_text(ctx, &entry.label, bounds.x, y + (row_h + ctx.theme.font_size_small) * 0.5 - 1.0, ctx.theme.font_size_small, ctx.theme.text_muted);
        draw_text(
            ctx,
            &entry.value,
            value_x,
            y + (row_h + ctx.theme.font_size_small) * 0.5 - 1.0,
            ctx.theme.font_size_small,
            ctx.theme.text,
        );
    }
}

fn quantize_step(value: f64, step: f64, min: f64) -> f64 {
    if step <= 0.0 {
        return value;
    }
    min + ((value - min) / step).round() * step
}

fn render_slider<E: Clone>(
    id: &str,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    on_change: Option<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let track_y = bounds.y + bounds.h * 0.5;
    ctx.draw.push_rounded([bounds.x, track_y - 2.0, bounds.w, 4.0], ctx.theme.separator, 2.0);
    let range = (max - min).max(f64::EPSILON);
    let mut t = ((value - min) / range).clamp(0.0, 1.0);
    if ctx.input.drag.active && ctx.input.drag.target_id.as_deref() == Some(id) {
        let dx = ctx.input.drag.current_x - ctx.input.drag.start_x;
        t = (t as f32 + dx / bounds.w.max(1.0)).clamp(0.0, 1.0) as f64;
    }
    let live = quantize_step(min + t * range, step, min).clamp(min, max);
    if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
        if let Some(on_change) = on_change.clone() {
            maps.slider_metas.insert(
                id.to_string(),
                SliderMeta {
                    on_change,
                    min,
                    max,
                    step,
                    value,
                    bounds_x: bounds.x,
                    bounds_w: bounds.w,
                },
            );
        }
        maps.slider_live_values.insert(id.to_string(), live);
    }
    let knob_x = bounds.x + bounds.w * ((live - min) / range).clamp(0.0, 1.0) as f32;
    ctx.draw.push_rounded([knob_x - 6.0, track_y - 6.0, 12.0, 12.0], ctx.theme.accent, 6.0);
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event: None,
        control_id: Some(id.to_string()),
        kind: HitKind::Slider,
        drag_axis: Some(DragAxis::Horizontal),
        drag_data: None,
    });
}

fn render_number_stepper<E: Clone>(
    id: &str,
    value: f64,
    step: f64,
    uniform: bool,
    on_absolute: Option<E>,
    on_delta: Option<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let seg = bounds.w / 3.0;
    let minus = Rect::new(bounds.x, bounds.y, seg, bounds.h);
    let center = Rect::new(bounds.x + seg, bounds.y, seg, bounds.h);
    let plus = Rect::new(bounds.x + seg * 2.0, bounds.y, seg, bounds.h);
    let hair = ctx.theme.stroke_hairline;
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, ctx.theme.input_bg);
    ctx.draw.push_solid([bounds.x + seg, bounds.y, hair, bounds.h], ctx.theme.border_normal);
    ctx.draw.push_solid([bounds.x + seg * 2.0, bounds.y, hair, bounds.h], ctx.theme.border_normal);
    let minus_hovered = ctx.input.hovered_id.as_deref() == Some(&format!("{id}.minus"));
    let plus_hovered = ctx.input.hovered_id.as_deref() == Some(&format!("{id}.plus"));
    if minus_hovered {
        ctx.draw.push_solid([minus.x, minus.y, minus.w, minus.h], ctx.theme.button_hover);
    }
    if plus_hovered {
        ctx.draw.push_solid([plus.x, plus.y, plus.w, plus.h], ctx.theme.button_hover);
    }
    draw_text(ctx, "−", minus.x + seg * 0.5 - 4.0, minus.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
    let text = if uniform {
        format!("{value:.3}")
    } else {
        format!("{value:.3}")
    };
    let input_id = format!("{id}.input");
    register_input_meta(ctx, &input_id, &text, None, on_absolute.clone());
    render_input(&input_id, &text, None, center, ctx);
    draw_text(ctx, "+", plus.x + seg * 0.5 - 4.0, plus.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
    if let (Some(maps), Some(on_absolute), Some(on_delta)) =
        (ctx.interaction_maps.as_deref_mut(), on_absolute.clone(), on_delta.clone())
    {
        maps.stepper_metas.insert(
            id.to_string(),
            StepperMeta {
                on_absolute,
                on_delta,
                step,
                value,
            },
        );
    }
    ctx.input.register_hit(HitTarget {
        rect: minus,
        event: None,
        control_id: Some(format!("{id}.minus")),
        kind: HitKind::Generic,
        drag_axis: None,
        drag_data: None,
    });
    ctx.input.register_hit(HitTarget {
        rect: plus,
        event: None,
        control_id: Some(format!("{id}.plus")),
        kind: HitKind::Generic,
        drag_axis: None,
        drag_data: None,
    });
}

fn render_ring<E: Clone>(
    id: &str,
    t: f64,
    disabled: bool,
    on_change: Option<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let cx = bounds.x + bounds.w * 0.5;
    let cy = bounds.y + bounds.h * 0.5;
    let radius = bounds.w.min(bounds.h) * 0.4;
    let segments = 48usize;
    let mut points = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let angle = std::f32::consts::TAU * i as f32 / segments as f32;
        points.push([cx + angle.cos() * radius, cy + angle.sin() * radius]);
    }
    for window in points.windows(2) {
        ctx.draw.push_line(
            window[0][0], window[0][1], window[1][0], window[1][1],
            ctx.theme.separator, 2.0,
        );
    }
    let mut knob_t = t;
    if !disabled && ctx.input.drag.active && ctx.input.drag.target_id.as_deref() == Some(id) {
        let dx = ctx.input.drag.current_x - cx;
        let dy = ctx.input.drag.current_y - cy;
        knob_t = (dy.atan2(dx) as f64 / std::f64::consts::TAU).rem_euclid(1.0);
    }
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change.clone()) {
        maps.ring_metas.insert(
            id.to_string(),
            RingMeta {
                on_change,
                disabled,
                center_x: cx,
                center_y: cy,
                radius,
            },
        );
        maps.ring_live_values.insert(id.to_string(), knob_t);
    }
    let knob_angle = std::f32::consts::TAU * knob_t as f32;
    let kx = cx + knob_angle.cos() * radius;
    let ky = cy + knob_angle.sin() * radius;
    let accent = if disabled { ctx.theme.text_muted } else { ctx.theme.accent };
    ctx.draw.push_rounded([kx - 6.0, ky - 6.0, 12.0, 12.0], accent, 6.0);
    if !disabled {
        ctx.input.register_hit(HitTarget {
            rect: bounds,
            event: None,
            control_id: Some(id.to_string()),
            kind: HitKind::Slider,
            drag_axis: Some(DragAxis::Ring),
            drag_data: None,
        });
    }
}

fn render_icon_select<E: Clone>(
    id: &str,
    value: &str,
    _uniform: bool,
    _classifier_kind: &str,
    on_change: Option<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    push_control_border(
        ctx.draw,
        bounds,
        ctx.theme,
        ctx.theme.border_normal,
        chrome_item_bg(ctx.theme, false, ctx.input.hovered_id.as_deref() == Some(id)),
    );
    let mut content_x = bounds.x + ctx.theme.padding_standard;
    if let Some(icons) = ctx.icons {
        if icons.icon_uv(value).is_some() {
            push_icon(
                ctx.draw,
                icons,
                value,
                content_x,
                bounds.y + (bounds.h - ICON_TINY) * 0.5,
                ICON_TINY,
                ctx.theme.text_element,
            );
            content_x += ICON_TINY + ctx.theme.gap_standard;
        } else {
            draw_text(
                ctx,
                value,
                content_x,
                bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
                ctx.theme.font_size_body,
                ctx.theme.text,
            );
        }
    } else {
        draw_text(
            ctx,
            value,
            content_x,
            bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
            ctx.theme.font_size_body,
            ctx.theme.text,
        );
    }
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event: on_change,
        control_id: Some(id.to_string()),
        kind: HitKind::Generic,
        drag_axis: None,
        drag_data: None,
    });
}

fn measure_tree_sections_width<E>(sections: &[TreeSection<E>], atlas: &mut FontAtlas, theme: &Theme) -> f32 {
    let collapsed = HashMap::new();
    measure_tree_sections_width_state(sections, atlas, theme, &collapsed, 0)
}

fn measure_tree_sections_width_state<E>(
    sections: &[TreeSection<E>],
    atlas: &mut FontAtlas,
    theme: &Theme,
    collapsed: &HashMap<String, bool>,
    depth: u32,
) -> f32 {
    let mut max_w = 0.0f32;
    for section in sections {
        let section_key = format!("section.{}", section.id);
        let section_collapsed = collapsed.get(&section_key).copied().unwrap_or(!section.default_open);
        if let Some(label) = &section.label {
            let w = atlas.measure_text(label, theme.font_size_small).0
                + tree_gutter_width(0)
                + TREE_ICON_SIZE
                + theme.gap_standard * 2.0;
            max_w = max_w.max(w);
        }
        if !section_collapsed {
            for item in &section.items {
                max_w = max_w.max(measure_tree_item_width(item, atlas, theme, collapsed, depth));
            }
        }
    }
    max_w.max(120.0)
}

fn measure_tree_item_width<E>(
    item: &TreeItem<E>,
    atlas: &mut FontAtlas,
    theme: &Theme,
    collapsed: &HashMap<String, bool>,
    depth: u32,
) -> f32 {
    if item.is_hidden {
        return 0.0;
    }
    let mut w = tree_gutter_width(depth)
        + TREE_ICON_SIZE
        + theme.gap_standard
        + atlas.measure_text(&item.label, theme.font_size_body).0
        + theme.gap_standard;
    if let Some(description) = &item.description {
        w += atlas.measure_text(description, theme.font_size_small).0 + theme.gap_standard;
    }
    for action in &item.actions {
        w += TREE_ICON_SIZE + theme.padding_standard;
        if let Some(label) = &action.label {
            w += atlas.measure_text(label, theme.font_size_small).0 + theme.gap_standard;
        }
    }
    if item.control.is_some() {
        w += 120.0 + theme.gap_standard;
    }
    let key = format!("tree.{}", item.id);
    let item_collapsed = collapsed.get(&key).copied().unwrap_or(!item.default_open);
    if !item_collapsed {
        for child in &item.children {
            w = w.max(measure_tree_item_width(child, atlas, theme, collapsed, depth + 1));
        }
    }
    w
}

fn measure_tree_sections<E>(sections: &[TreeSection<E>]) -> f32 {
    let collapsed = HashMap::new();
    measure_tree_sections_state(sections, &collapsed)
}

fn measure_tree_sections_state<E>(sections: &[TreeSection<E>], collapsed: &HashMap<String, bool>) -> f32 {
    let mut height = 0.0;
    for section in sections {
        height += TREE_ROW_HEIGHT;
        let section_key = format!("section.{}", section.id);
        let section_collapsed = collapsed.get(&section_key).copied().unwrap_or(!section.default_open);
        if !section_collapsed {
            for item in &section.items {
                height += measure_tree_item_height(item, collapsed, 0);
            }
            height += TREE_SECTION_GAP;
        }
    }
    height
}

fn measure_tree_item_height<E>(item: &TreeItem<E>, collapsed: &HashMap<String, bool>, depth: u32) -> f32 {
    if item.is_hidden {
        return 0.0;
    }
    let mut height = TREE_ROW_HEIGHT;
    let key = format!("tree.{}", item.id);
    let item_collapsed = collapsed.get(&key).copied().unwrap_or(!item.default_open);
    if !item_collapsed {
        for child in &item.children {
            height += measure_tree_item_height(child, collapsed, depth + 1);
        }
    }
    height
}

fn tree_gutter_width(depth: u32) -> f32 {
    depth as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH
}

fn tree_icon_id<E>(item: &TreeItem<E>, expandable: bool) -> &str {
    item.icon_id
        .as_deref()
        .unwrap_or(if expandable { "folder" } else { "file-text" })
}

fn tree_row_collapsed(collapsed: &HashMap<String, bool>, key: &str, default_open: bool) -> bool {
    collapsed.get(key).copied().unwrap_or(!default_open)
}

fn render_tree<E: Clone>(
    sections: &[TreeSection<E>],
    selected_ids: &[String],
    highlighted_ids: &[String],
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let mut y = bounds.y;
    for section in sections {
    let section_key = format!("section.{}", section.id);
    if !ctx.collapsed_sections.contains_key(&section_key) {
        ctx.collapsed_sections.insert(section_key.clone(), !section.default_open);
    }
    let section_collapsed = tree_row_collapsed(ctx.collapsed_sections, &section_key, section.default_open);
        render_tree_section_header(section, bounds, y, section_collapsed, ctx);
        y += TREE_ROW_HEIGHT;
        if !section_collapsed {
            for item in &section.items {
                y += render_tree_item(
                    item,
                    Rect::new(bounds.x, y, bounds.w, TREE_ROW_HEIGHT),
                    ctx,
                    0,
                    selected_ids,
                    highlighted_ids,
                    &[],
                );
            }
            y += TREE_SECTION_GAP;
        }
    }
}

fn render_tree_section_header<E: Clone>(
    section: &TreeSection<E>,
    bounds: Rect,
    y: f32,
    collapsed: bool,
    ctx: &mut WidgetContext<'_, E>,
) {
    let row = Rect::new(bounds.x, y, bounds.w, TREE_ROW_HEIGHT);
    let gutter_w = TREE_TOGGLE_WIDTH;
    let gutter = Rect::new(row.x, row.y, gutter_w, row.h);
    let content = Rect::new(row.x + gutter_w, row.y, row.w - gutter_w, row.h);
    let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
    tree_draw_chevron(ctx, chevron, gutter);
    ctx.input.register_hit(HitTarget {
        rect: gutter,
        event: None,
        control_id: Some(format!("section.chevron.{}", section.id)),
        kind: HitKind::TreeItem,
        drag_axis: None,
        drag_data: None,
    });
    if let Some(label) = &section.label {
        let text_color = if collapsed { ctx.theme.text_muted } else { ctx.theme.text_element };
        let label_x = content.x + ctx.theme.gap_standard;
        if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv("folder")) {
            draw_icon(ctx, uv, label_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
        }
        draw_text(
            ctx,
            label,
            label_x + TREE_ICON_SIZE + ctx.theme.gap_standard,
            content.y + (content.h + ctx.theme.font_size_small) * 0.5 - 1.0,
            ctx.theme.font_size_small,
            text_color,
        );
    }
}

fn render_tree_item<E: Clone>(
    item: &TreeItem<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
    depth: u32,
    selected_ids: &[String],
    highlighted_ids: &[String],
    is_last_at_level: &[bool],
) -> f32 {
    if item.is_hidden {
        return 0.0;
    }
    let key = format!("tree.{}", item.id);
    if !ctx.collapsed_sections.contains_key(&key) {
        ctx.collapsed_sections.insert(key.clone(), !item.default_open);
    }
    let collapsed = tree_row_collapsed(ctx.collapsed_sections, &key, item.default_open);
    let expandable = !item.children.is_empty();
    let gutter_w = tree_gutter_width(depth);
    let row = Rect::new(bounds.x, bounds.y, bounds.w, TREE_ROW_HEIGHT);
    let gutter = Rect::new(row.x, row.y, gutter_w, row.h);
    let content = Rect::new(row.x + gutter_w, row.y, row.w - gutter_w, row.h);
    let hovered = ctx
        .input
        .hovered_id
        .as_deref()
        .is_some_and(|id| id.strip_prefix("tree.label.").is_some_and(|v| v == item.id));
    let selected = item.selected || selected_ids.iter().any(|id| id == &item.id);
    let highlighted = item.highlighted || highlighted_ids.iter().any(|id| id == &item.id);
    tree_draw_guides(ctx, gutter, depth, is_last_at_level);
    if expandable {
        let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
        let chevron_rect = Rect::new(
            gutter.x + depth as f32 * TREE_INDENT_PER_LEVEL,
            gutter.y,
            TREE_TOGGLE_WIDTH,
            gutter.h,
        );
        tree_draw_chevron(ctx, chevron, chevron_rect);
        ctx.input.register_hit(HitTarget {
            rect: chevron_rect,
            event: None,
            control_id: Some(format!("tree.chevron.{}", item.id)),
            kind: HitKind::TreeItem,
            drag_axis: None,
            drag_data: None,
        });
    }
    if selected {
        ctx.draw.push_rounded([content.x, content.y, content.w, content.h], ctx.theme.selected, ctx.theme.border_radius);
    } else if highlighted || hovered {
        ctx.draw.push_rounded([content.x, content.y, content.w, content.h], ctx.theme.row_hover, ctx.theme.border_radius);
    }
    let mut label_x = content.x + ctx.theme.gap_standard;
    let icon_id = tree_icon_id(item, expandable);
    let text_color = if selected || highlighted {
        ctx.theme.active_foreground
    } else if hovered {
        ctx.theme.border_emphasized
    } else if item.is_hidden {
        ctx.theme.text_muted
    } else {
        ctx.theme.text_element
    };
    if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(icon_id)) {
        draw_icon(ctx, uv, label_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
        label_x += TREE_ICON_SIZE + ctx.theme.gap_standard;
    }
    draw_text(
        ctx,
        &item.label,
        label_x,
        content.y + (content.h + ctx.theme.font_size_body) * 0.5 - 2.0,
        ctx.theme.font_size_body,
        text_color,
    );
    if let Some(description) = &item.description {
        let label_w = measure_text_width(ctx, &item.label, ctx.theme.font_size_body);
        draw_text(
            ctx,
            description,
            label_x + label_w + ctx.theme.gap_standard,
            content.y + (content.h + ctx.theme.font_size_small) * 0.5 - 1.0,
            ctx.theme.font_size_small,
            ctx.theme.text_muted,
        );
    }
    let mut actions_x = content.x + content.w - ctx.theme.gap_standard;
    for (index, action) in item.actions.iter().enumerate().rev() {
        if action.reveal_on_hover && !hovered {
            continue;
        }
        let label_w = action
            .label
            .as_ref()
            .map(|label| measure_text_width(ctx, label, ctx.theme.font_size_small) + ctx.theme.gap_standard)
            .unwrap_or(0.0);
        let action_w = TREE_ICON_SIZE + ctx.theme.padding_standard + label_w;
        actions_x -= action_w;
        let action_rect = Rect::new(actions_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5 - 2.0, action_w, TREE_ICON_SIZE + 4.0);
        if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(&action.icon_id)) {
            let action_color = if hovered {
                ctx.theme.border_emphasized
            } else {
                ctx.theme.text_element
            };
            draw_icon(ctx, uv, action_rect.x + 2.0, action_rect.y + 2.0, TREE_ICON_SIZE, action_color);
        }
        if hovered {
            if let Some(label) = &action.label {
                draw_text(
                    ctx,
                    label,
                    action_rect.x + TREE_ICON_SIZE + 4.0,
                    action_rect.y + (TREE_ICON_SIZE + ctx.theme.font_size_small) * 0.5,
                    ctx.theme.font_size_small,
                    ctx.theme.text_muted,
                );
            }
        }
        ctx.input.register_hit(HitTarget {
            rect: action_rect,
            event: Some(action.event.clone()),
            control_id: Some(format!("tree.action.{}.{}", item.id, index)),
            kind: HitKind::Button,
            drag_axis: None,
            drag_data: None,
        });
    }
    if let Some(hover) = &item.hover_event {
        if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
            maps.tree_hover_commands.insert(item.id.clone(), hover.clone());
        }
    }
    if let Some(unhover) = &item.unhover_event {
        if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
            maps.tree_unhover_commands.insert(item.id.clone(), unhover.clone());
        }
    }
    if let Some(control) = &item.control {
        let control_w = 120.0;
        let control_rect = Rect::new(
            content.x + content.w - control_w - ctx.theme.gap_standard,
            content.y + (content.h - ctx.theme.control_height) * 0.5,
            control_w,
            ctx.theme.control_height,
        );
        render_widget(control, control_rect, ctx);
    }
    let label_rect = Rect::new(label_x, content.y, content.x + content.w - label_x - ctx.theme.gap_standard, content.h);
    ctx.input.register_hit(HitTarget {
        rect: label_rect,
        event: item.event.clone(),
        control_id: Some(format!("tree.label.{}", item.id)),
        kind: HitKind::TreeItem,
        drag_axis: if item.draggable { Some(DragAxis::Both) } else { None },
        drag_data: if item.draggable && !item.drag_data.is_empty() {
            Some(item.drag_data.clone())
        } else {
            None
        },
    });
    let mut height = TREE_ROW_HEIGHT;
    if !collapsed {
        for (index, child) in item.children.iter().enumerate() {
            let mut child_is_last = is_last_at_level.to_vec();
            child_is_last.push(index + 1 == item.children.len());
            let child_bounds = Rect::new(bounds.x, bounds.y + height, bounds.w, TREE_ROW_HEIGHT);
            height += render_tree_item(
                child,
                child_bounds,
                ctx,
                depth + 1,
                selected_ids,
                highlighted_ids,
                &child_is_last,
            );
        }
    }
    height
}

fn tree_draw_chevron<E>(ctx: &mut WidgetContext<'_, E>, icon_id: &str, rect: Rect) {
    if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(icon_id)) {
        draw_icon(
            ctx,
            uv,
            rect.x + (rect.w - TREE_ICON_SIZE) * 0.5,
            rect.y + (rect.h - TREE_ICON_SIZE) * 0.5,
            TREE_ICON_SIZE,
            ctx.theme.text_muted,
        );
    }
}

fn tree_draw_guides<E>(ctx: &mut WidgetContext<'_, E>, gutter: Rect, depth: u32, is_last_at_level: &[bool]) {
    let hair = ctx.theme.stroke_hairline.max(1.0);
    let guide_color = ctx.theme.border_normal;
    for level in 0..depth {
        if is_last_at_level.get(level as usize).copied().unwrap_or(false) {
            continue;
        }
        let x = gutter.x + level as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        ctx.draw.push_solid([x, gutter.y, hair, gutter.h], guide_color);
    }
    if depth > 0 {
        let x = gutter.x + (depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        let mid_y = gutter.y + gutter.h * 0.5;
        ctx.draw.push_solid([x, gutter.y, hair, mid_y - gutter.y], guide_color);
        ctx.draw.push_solid([x, mid_y, TREE_INDENT_PER_LEVEL * 0.5, hair], guide_color);
    }
}

pub fn render_scroll_region<E: Clone, F: FnOnce(Rect, &mut WidgetContext<'_, E>)>(
    scroll_id: &str,
    bounds: Rect,
    content_height: f32,
    ctx: &mut WidgetContext<'_, E>,
    render_content: F,
) {
    let max_scroll = (content_height - bounds.h).max(0.0);
    let offset = ctx
        .scroll_offsets
        .entry(scroll_id.to_string())
        .or_insert(0.0);
    *offset = offset.clamp(0.0, max_scroll);
    let scroll = *offset;
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event: None,
        control_id: Some(scroll_id.to_string()),
        kind: HitKind::ScrollRegion,
        drag_axis: None,
        drag_data: None,
    });
    ctx.draw.push_scissor(bounds);
    let content_bounds = Rect::new(bounds.x, bounds.y - scroll, bounds.w, content_height);
    render_content(content_bounds, ctx);
    ctx.draw.pop_scissor();
}

pub fn draw_icon<E>(ctx: &mut WidgetContext<'_, E>, uv: [f32; 4], x: f32, y: f32, size: f32, color: Rgba) {
    ctx.draw.push_textured([x, y, size, size], uv, color);
}

fn measure_text_width<E>(ctx: &mut WidgetContext<'_, E>, text: &str, size: f32) -> f32 {
    let (w, _) = ctx.atlas.measure_text(text, size);
    w
}

pub fn draw_text_wrapped<E>(
    ctx: &mut WidgetContext<'_, E>,
    text: &str,
    x: f32,
    y: f32,
    max_width: f32,
    size: f32,
    color: Rgba,
) -> f32 {
    let lines = wrap_text(ctx.atlas, text, max_width, size);
    let line_h = size * 1.35;
    for (index, line) in lines.iter().enumerate() {
        draw_text(ctx, line, x, y + line_h * index as f32 + size, size, color);
    }
    lines.len() as f32 * line_h
}

pub fn wrap_text(atlas: &mut FontAtlas, text: &str, max_width: f32, size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let trial = if current.is_empty() {
            word.to_string()
        } else {
            format!("{current} {word}")
        };
        let (w, _) = atlas.measure_text(&trial, size);
        if w > max_width && !current.is_empty() {
            lines.push(current);
            current = word.to_string();
        } else {
            current = trial;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

pub fn draw_text_on(
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Rgba,
) {
    let scale = size / 16.0;
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = atlas.ensure_glyph(ch);
        let gw = glyph.width as f32 * scale;
        let gh = glyph.height as f32 * scale;
        let gx = cursor_x + glyph.bearing_x * scale;
        let gy = y - gh - glyph.bearing_y * scale;
        let uv_rect = [
            glyph.atlas_x as f32 / atlas_w,
            glyph.atlas_y as f32 / atlas_h,
            (glyph.atlas_x + glyph.width) as f32 / atlas_w,
            (glyph.atlas_y + glyph.height) as f32 / atlas_h,
        ];
        draw.push_glyph([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance * scale;
    }
}

pub fn draw_text_overlay_on(
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Rgba,
) {
    let scale = size / 16.0;
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = atlas.ensure_glyph(ch);
        let gw = glyph.width as f32 * scale;
        let gh = glyph.height as f32 * scale;
        let gx = cursor_x + glyph.bearing_x * scale;
        let gy = y - gh - glyph.bearing_y * scale;
        let uv_rect = [
            glyph.atlas_x as f32 / atlas_w,
            glyph.atlas_y as f32 / atlas_h,
            (glyph.atlas_x + glyph.width) as f32 / atlas_w,
            (glyph.atlas_y + glyph.height) as f32 / atlas_h,
        ];
        draw.push_glyph_overlay([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance * scale;
    }
}

pub fn draw_text<E>(ctx: &mut WidgetContext<'_, E>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let scale = size / 16.0;
    let atlas_w = ctx.atlas.width as f32;
    let atlas_h = ctx.atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = ctx.atlas.ensure_glyph(ch);
        let gw = glyph.width as f32 * scale;
        let gh = glyph.height as f32 * scale;
        let gx = cursor_x + glyph.bearing_x * scale;
        let gy = y - gh - glyph.bearing_y * scale;
        let uv_rect = [
            glyph.atlas_x as f32 / atlas_w,
            glyph.atlas_y as f32 / atlas_h,
            (glyph.atlas_x + glyph.width) as f32 / atlas_w,
            (glyph.atlas_y + glyph.height) as f32 / atlas_h,
        ];
        ctx.draw.push_glyph([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance * scale;
    }
}

pub fn draw_text_overlay<E>(ctx: &mut WidgetContext<'_, E>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    draw_text_overlay_on(ctx.draw, ctx.atlas, text, x, y, size, color);
}
// #endregion widgets
}

#[cfg(feature = "engine")]
pub mod host {
// #region host
//! 🪟 winit window event bridge into pointer callbacks.

use crate::input::{KeyAction, PointerCallbacks, PointerModifiers};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{Key, NamedKey};

pub fn pointer_coords(_window: &winit::window::Window, position: winit::dpi::PhysicalPosition<f64>) -> (f32, f32) {
    (position.x as f32, position.y as f32)
}

pub fn modifiers_from_winit(modifiers: winit::keyboard::ModifiersState) -> PointerModifiers {
    PointerModifiers {
        shift: modifiers.shift_key(),
        ctrl: modifiers.control_key(),
        alt: modifiers.alt_key(),
        meta: modifiers.super_key(),
    }
}

pub struct WindowInputState {
    pub pointer_x: f32,
    pub pointer_y: f32,
    pub pointer_down: bool,
    pub pointer_button: i16,
    pub modifiers: PointerModifiers,
}

impl Default for WindowInputState {
    fn default() -> Self {
        Self {
            pointer_x: 0.0,
            pointer_y: 0.0,
            pointer_down: false,
            pointer_button: 0,
            modifiers: PointerModifiers::default(),
        }
    }
}

pub fn dispatch_window_event(
    window: &winit::window::Window,
    event: &WindowEvent,
    input: &mut WindowInputState,
    callbacks: &PointerCallbacks,
) -> bool {
    match event {
        WindowEvent::ModifiersChanged(modifiers) => {
            input.modifiers = modifiers_from_winit(modifiers.state());
            true
        }
        WindowEvent::CursorMoved { position, .. } => {
            let (x, y) = pointer_coords(window, *position);
            input.pointer_x = x;
            input.pointer_y = y;
            (callbacks.on_move)(
                x,
                y,
                input.pointer_down,
                input.pointer_button,
                input.modifiers.clone(),
            );
            true
        }
        WindowEvent::MouseInput { state, button, .. } => {
            let down = *state == ElementState::Pressed;
            let btn = mouse_button_to_i16(*button);
            if down {
                input.pointer_down = true;
                input.pointer_button = btn;
            } else if input.pointer_down {
                input.pointer_down = false;
            }
            (callbacks.on_button)(
                input.pointer_x,
                input.pointer_y,
                down,
                btn,
                input.modifiers.clone(),
            );
            true
        }
        WindowEvent::MouseWheel { delta, .. } => {
            let delta_y = match delta {
                MouseScrollDelta::LineDelta(_, y) => *y * 40.0,
                MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
            };
            (callbacks.on_wheel)(
                delta_y,
                input.pointer_x,
                input.pointer_y,
                input.modifiers.clone(),
            );
            true
        }
        WindowEvent::KeyboardInput { event, .. } => {
            if let Key::Named(NamedKey::Space) = &event.logical_key {
                (callbacks.on_key)(
                    KeyAction::Space(event.state == ElementState::Pressed),
                    input.modifiers.clone(),
                );
                return true;
            }
            if event.state != ElementState::Pressed {
                return true;
            }
            let action = key_action_from_event(event);
            if let Some(action) = action {
                (callbacks.on_key)(action, input.modifiers.clone());
            }
            true
        }
        _ => false,
    }
}

fn mouse_button_to_i16(button: MouseButton) -> i16 {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 2,
        MouseButton::Middle => 1,
        MouseButton::Back => 3,
        MouseButton::Forward => 4,
        MouseButton::Other(id) => id as i16,
    }
}

fn key_action_from_event(event: &KeyEvent) -> Option<KeyAction> {
    match &event.logical_key {
        Key::Named(NamedKey::Backspace) => Some(KeyAction::Backspace),
        Key::Named(NamedKey::Delete) => Some(KeyAction::Delete),
        Key::Named(NamedKey::Enter) => Some(KeyAction::Enter),
        Key::Named(NamedKey::Escape) => Some(KeyAction::Escape),
        Key::Named(NamedKey::ArrowLeft) => Some(KeyAction::ArrowLeft),
        Key::Named(NamedKey::ArrowRight) => Some(KeyAction::ArrowRight),
        Key::Named(NamedKey::ArrowUp) => Some(KeyAction::ArrowUp),
        Key::Named(NamedKey::ArrowDown) => Some(KeyAction::ArrowDown),
        Key::Named(NamedKey::Tab) => Some(KeyAction::Tab),
        Key::Character(ch) if ch.chars().count() == 1 => {
            Some(KeyAction::Char(ch.to_string()))
        }
        _ => None,
    }
}
// #endregion host
}


// #region re-exports
// 🧩 Always available: declarative component types + engine-agnostic primitives (default features).
pub use geometry::Rect;
pub use theme::{GlassTier, Rgba, Theme};

// 🖥️ Retained-mode engine surface (feature = "engine" only).
#[cfg(feature = "engine")]
pub use cursor::{resolve_semio_cursor, apply_window_cursor, CursorDragState, SemioCursor};
#[cfg(all(feature = "engine", target_arch = "wasm32"))]
pub use cursor::apply_canvas_cursor;
#[cfg(feature = "engine")]
pub use draw::{mesh_content_version, DrawList, IconAtlas, MeshGpuStore, RasterTextureStore, ear_clip_polygon, paint_selection_marquee};
#[cfg(feature = "engine")]
pub use gpu::GpuContext;
#[cfg(feature = "engine")]
pub use gpu::schedule_frame;
#[cfg(feature = "engine")]
pub use host::{dispatch_window_event, modifiers_from_winit, pointer_coords, WindowInputState};
#[cfg(feature = "engine")]
pub use input::{DragAxis, DragState, HitKind, HitTarget, InputState, KeyAction, PointerCallbacks, PointerModifiers, TreeDragState, TreeDropPosition};
#[cfg(feature = "engine")]
pub use layout::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
#[cfg(feature = "engine")]
pub use kernel_3d_scene::{
    aabb_intersects_frustum, axis_rotate_angle, Camera3d, frustum_planes, gumball_axis_drag_plane_normal,
    gumball_extent, gumball_eye, gumball_project_ray_onto_axis, Instance3d, LineDraw3d, LineVertex3d, Mat4,
    Mesh3d, OrbitController, quat_from_basis, ray_plane_point, ray_segment_distance, rotate_vector, SceneDraw3d,
    ScenePass3d, TexturedDraw3d, TexturedInstance3d, Vec3, vec3_from_f64, point_in_polygon, project_point,
    ray_aabb_slab, ray_pick_instance, rect_contains, screen_select_instances, transform_aabb,
    marquee_is_crossing_from_path,
};
#[cfg(feature = "engine")]
pub use text::{fetch_font_bytes, FontAtlas};
#[cfg(feature = "engine")]
pub use chrome::{
    chrome_item_bg, chrome_item_text, item_bg, item_text, measure_action_item, push_chrome_border,
    push_chrome_group_border, push_control_border, push_icon, push_window_cap_border, ICON_TINY,
};
#[cfg(feature = "engine")]
pub use widgets::{
    draw_icon, draw_text, draw_text_overlay, draw_text_wrapped, measure_widget, render_scroll_region, render_widget,
    wrap_text, ControlNode, InputMeta, KeyValueEntry, RingMeta, SelectItem, SliderMeta, StepperMeta,
    TreeItem, TreeItemAction, TreeSection, Vec3Meta, WidgetContext, WidgetInteractionMaps, WidgetNode,
};
// #endregion re-exports
