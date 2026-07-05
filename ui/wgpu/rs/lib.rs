//! 🖱️ Business-logic-free wgpu UI toolkit for browser WASM renderers.

pub mod chrome;
pub mod cursor;
pub mod draw;
pub mod geometry;
pub mod gpu;
pub mod input;
pub mod layout;
pub mod shaders;
pub mod text;
pub mod theme;
pub mod widgets;

pub use cursor::{resolve_semio_cursor, CursorDragState, SemioCursor};
#[cfg(target_arch = "wasm32")]
pub use cursor::apply_canvas_cursor;
pub use draw::{mesh_content_version, DrawList, IconAtlas, MeshGpuStore, RasterTextureStore, ear_clip_polygon};
pub use geometry::Rect;
pub use gpu::GpuContext;
#[cfg(target_arch = "wasm32")]
pub use gpu::schedule_frame;
pub use input::{DragAxis, DragState, HitKind, HitTarget, InputState, KeyAction, PointerModifiers, TreeDragState, TreeDropPosition};
#[cfg(target_arch = "wasm32")]
pub use input::{PointerCallbacks, attach_dom_listeners};
pub use layout::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
pub use kernel_3d_scene::{
    aabb_intersects_frustum, axis_rotate_angle, Camera3d, frustum_planes, gumball_axis_drag_plane_normal,
    gumball_extent, gumball_eye, gumball_project_ray_onto_axis, Instance3d, LineDraw3d, LineVertex3d, Mat4,
    Mesh3d, OrbitController, quat_from_basis, ray_plane_point, ray_segment_distance, rotate_vector, SceneDraw3d,
    ScenePass3d, TexturedDraw3d, TexturedInstance3d, Vec3, vec3_from_f64, point_in_polygon, project_point,
    ray_aabb_slab, ray_pick_instance, rect_contains, screen_select_instances, transform_aabb,
};
pub use text::{fetch_font_bytes, FontAtlas};
pub use theme::{GlassTier, Rgba, Theme};
pub use chrome::{
    chrome_item_bg, chrome_item_text, item_bg, item_text, measure_action_item, push_chrome_border,
    push_chrome_group_border, push_control_border, push_icon, push_window_cap_border, ICON_TINY,
};
pub use widgets::{
    draw_icon, draw_text, draw_text_wrapped, measure_widget, render_scroll_region, render_widget,
    wrap_text, ControlNode, InputMeta, KeyValueEntry, RingMeta, SelectItem, SliderMeta, StepperMeta,
    TreeItem, TreeItemAction, TreeSection, Vec3Meta, WidgetContext, WidgetInteractionMaps, WidgetNode,
};
