import sys, os, re
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ri1-tools")
from ed import *

INT = EL + "/🗣️Interpreter/🎯️targets/🧊️wgpu"
IMG = INT + "/🖼️ui-image-loading/🦀️.rs"
VAL = INT + "/🧮️render-plan-validator/🦀️.rs"
TESTS = EL + "/🗣️Interpreter/🧪️tests"
OLDT = TESTS + "/🔬️wgpu-render-plan-validator/🦀️.rs"
SC = T + "/🎨️scene-canvas2d"

ui_dir = [d for d in os.listdir(INT) if d.endswith("ui-image-loading")][0]
newname = ui_dir.replace("ui-image-loading", "wgpu-ui-image-loading")
assert newname.startswith("🖼") 
NEWT = TESTS + "/" + newname + "/🦀️.rs"

t = read(OLDT)
i = t.index("//#region UiImageLoadingTests")
head, tail = t[:i], t[i:]
assert tail.rstrip().endswith("}") and "avatar_image_replacement_cannot_publish_the_previous_sources_pixels" in tail
tail = tail.replace("crate::scenes::PendingRasterUploadCursor", "PendingRasterUploadCursor").replace("crate::scenes::PendingRasterUploadStep", "PendingRasterUploadStep")
tail = tail.replace("crate::take_next_renderer_asset()", "take_next_renderer_asset()").replace("crate::return_renderer_asset(", "return_renderer_asset(").replace("crate::retire_cancelled_renderer_asset_step()", "retire_cancelled_renderer_asset_step()")
tail = tail.replace("crate::interpreter::framework_widget_context(", "framework_widget_context(")
header = '''//! 🖼️ Unit tests of the UI image loading of scene-canvas2d: inline SVG and data url admission, fetched bytes, object-contain fit and the inline image paint step.

use crate::ui_image_loading::{apply_ui_image_bytes, current_ui_image_key, object_contain_rect, render_ui_image_step, resolve_ui_image, INLINE_SVG_PARSE_CALLS, UI_IMAGE_SIZES};
use infinite_world::world::{WorldAssetMetadataId, WorldAssetRequestKind};
use semio_framework_os_renderer_wgpu_asset_authority::{retire_cancelled_renderer_asset_step, return_renderer_asset, take_next_renderer_asset};
use semio_framework_os_renderer_wgpu_contracts::framework_widget_context;
use semio_framework_os_renderer_wgpu_scene_state::{PendingRasterUploadCursor, PendingRasterUploadStep};
use ui_wgpu::wgpu::{ActionDescriptor, Rect};

'''
os.makedirs(os.path.dirname(NEWT), exist_ok=True)
write(NEWT, header + tail)
write(OLDT, head.rstrip("\n") + "\n")

sub1(VAL, '''#[cfg(test)]
use super::ui_image_loading::{
    apply_ui_image_bytes, current_ui_image_key, object_contain_rect, render_ui_image_step,
    resolve_ui_image, INLINE_SVG_PARSE_CALLS, UI_IMAGE_SIZES,
};
#[cfg(test)]
use infinite_world::world::{WorldAssetMetadataId, WorldAssetRequestKind};
use serde_json::Value;
use ui_wgpu::wgpu::UiComponentSceneNode;
#[cfg(test)]
use ui_wgpu::wgpu::{ActionDescriptor, Rect, UiNode, UiPresence};
''', '''use serde_json::Value;
use ui_wgpu::wgpu::UiComponentSceneNode;
#[cfg(test)]
use ui_wgpu::wgpu::{UiNode, UiPresence};
''')

t = read(IMG)
t = t.replace("use crate::scenes::{queue_canvas_image_upload_sized, queue_canvas_image_upload_with};", "use crate::{queue_canvas_image_upload_sized, queue_canvas_image_upload_with};")
assert t.count("crate::reserve_renderer_asset_request(") == 1
t = t.replace("crate::reserve_renderer_asset_request(", "semio_framework_os_renderer_wgpu_asset_authority::reserve_renderer_asset_request(")
for a in ("pub(super) ",):
    t = t.replace(a, "pub ")
t = t.replace("pub(crate) fn current_ui_image_key", "pub fn current_ui_image_key")
write(IMG, t)

# scene-canvas2d root and manifest
R2 = SC + "/🦀️.rs"
sub1(R2, "extern crate semio_framework_os_renderer_wgpu_canvas2d_paint as canvas2d_paint;\n", "extern crate infinite_canvas as infinite_world;\nextern crate semio_framework_os_renderer_wgpu_canvas2d_paint as canvas2d_paint;\n")
sub1(R2, '''#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🦀️.rs"]
mod canvas2d;
pub use canvas2d::*;
''', '''#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🦀️.rs"]
mod canvas2d;
pub use canvas2d::*;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🖼️ui-image-loading/🦀️.rs"]
pub mod ui_image_loading;
pub use ui_image_loading::{apply_ui_image_bytes, current_ui_image_key, take_ui_image_asset_fault};
''')
sub1(R2, '''#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;''', f'''#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../../🧱️elements/🗣️Interpreter/🧪️tests/{newname}/🦀️.rs"]
mod ui_image_loading_tests;''')
M = SC + "/📦️packages/🦀️rust/Cargo.toml"
sub1(M, 'base64 = "0.22.1"\n', 'base64 = "0.22.1"\ninfinite_canvas = { path = "../../../../../../../♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }\n')
sub1(R2, "//! The canvas 2d scene render (layers, gradients, shapes, scene nodes, framing, hit paint order) built on the scene state; the gumball overlay is painted through the canvas2d-host crate.", "//! The canvas 2d scene render (layers, gradients, shapes, scene nodes, framing, hit paint order) built on the scene state; the gumball overlay is painted through the canvas2d-host crate. The UI image loading (data url and fetched image admission, SVG rasterisation, the retained image paint step) of the interpreter lives here too, because the scene leaves resolve their image keys through it.")

# Interpreter parent
P = INT + "/🦀️.rs"
sub1(P, '''#[path = "🖼️ui-image-loading/🦀️.rs"]
mod ui_image_loading;
pub(crate) use ui_image_loading::current_ui_image_key;''', '''use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading;
pub(crate) use ui_image_loading::current_ui_image_key;''')
print(newname)
