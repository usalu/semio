"""🧰 rw2t step A2: the engine-attach-tests Scenes module imports the moved helpers from their owners."""
from ed import *
T_ = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
ROOT = T_ + "/🧷️engine-attach-tests/🦀️.rs"
replace(ROOT, [
 ("ink_effective_bounds, ink_hits_point, ink_item_id,", "ink_effective_bounds, ink_hits_point, ink_item_id, ink_items_at_point,", 1),
 ("use semio_framework_os_renderer_wgpu_scene_tables::{PAINT2D_NAVIGATOR_PADDING, Paint2dDocSyncJson, Paint2dFlatLayer};", "use semio_framework_os_renderer_wgpu_scene_tables::{PAINT2D_NAVIGATOR_PADDING, Paint2dDocSyncJson, Paint2dFlatLayer, paint2d_navigator_fit_viewport, paint2d_navigator_overlay_rect};", 1),
 ("use semio_framework_os_renderer_wgpu_scene_text_vfs::{double_click_action, hit_double_click_target};", "use semio_framework_os_renderer_wgpu_scene_text_vfs::{cursor_from_click, double_click_action, hit_double_click_target, line_col_at};", 1),
])
