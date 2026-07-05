//! 🖱️ Business-logic-free wgpu UI toolkit for browser WASM renderers.

pub mod draw;
pub mod geometry;
pub mod gpu;
pub mod input;
pub mod layout;
pub mod scene3d;
pub mod shaders;
pub mod text;
pub mod theme;
pub mod widgets;

pub use draw::{mesh_content_version, DrawList, MeshGpuStore, ear_clip_polygon};
pub use geometry::Rect;
pub use gpu::GpuContext;
#[cfg(target_arch = "wasm32")]
pub use gpu::schedule_frame;
pub use input::{DragState, HitKind, HitTarget, InputState, PointerModifiers};
#[cfg(target_arch = "wasm32")]
pub use input::{PointerCallbacks, attach_dom_listeners};
pub use layout::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
pub use scene3d::{
    aabb_intersects_frustum, Camera3d, frustum_planes, Instance3d, Mat4, Mesh3d, OrbitController,
    SceneDraw3d, ScenePass3d, Vec3, point_in_polygon, project_point, ray_pick_instance,
    rect_contains, screen_select_instances, transform_aabb,
};
pub use text::{fetch_font_bytes, FontAtlas};
pub use theme::{Rgba, Theme};
pub use widgets::{
    draw_text, measure_widget, render_widget, ControlNode, KeyValueEntry, SelectItem, TreeItem,
    TreeSection, WidgetContext, WidgetNode,
};
