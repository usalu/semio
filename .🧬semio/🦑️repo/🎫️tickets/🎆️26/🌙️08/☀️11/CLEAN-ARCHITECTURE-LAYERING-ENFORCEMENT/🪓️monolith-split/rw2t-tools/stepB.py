"""🧩 rw2t step B: the two Canvas2d camera laws that name the Scenes standalone emulator (`handle_scene_*`) leave the canvas2d law for their own file, mounted next to the emulator."""
import re, sys
from ed import *
E_ = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
LAW = E_ + "/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs"
NEW = E_ + "/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d-camera/🦀️.rs"
ROOT = E_ + "/🎯️targets/🧊️wgpu/🧷️engine-attach-tests/🦀️.rs"
DRY = "--dry" in sys.argv

text = read(LAW)
def take_test(text, fn_name, doc_start):
    i = text.index(doc_start)
    assert text.count(doc_start) == 1
    j = text.index("fn " + fn_name + "(", i)
    end = text.index("\n}\n", j) + 3
    block = text[i:end]
    rest = text[:i] + text[end:].lstrip("\n")
    return block, rest

b1, text = take_test(text, "canvas2d_wheel_schedules_a_settled_camera_dispatch_without_firing_immediately", "/// 🕒️ A Canvas2d wheel tick mutates the viewport immediately")
b2, text = take_test(text, "canvas2d_pan_drag_schedules_a_settled_camera_dispatch", "/// 🕒️ A Canvas2d pan-drag (`SceneDragMode::PanViewport`)")
text = re.sub(r"\n{3,}", "\n\n", text)

def strip_heads(b):
    b = b.replace("crate::scenes::camera_fixture_lease(crate::scenes::camera_fixture_grant())", "camera_fixture_lease(camera_fixture_grant())")
    b = b.replace("crate::app_now_ms()", "app_now_ms()")
    return b
b1, b2 = strip_heads(b1), strip_heads(b2)
assert "crate::" not in b1 + b2, (b1 + b2)

scene_fn = re.search(r"fn canvas_scene\(.*?\n}\n", read(LAW), re.S).group(0)
fixture_fn = re.search(r"fn canvas_camera_gesture_fixture\(\) -> Value \{.*?\n}\n", read(LAW), re.S).group(0)

header = '''//! 🕒️ Canvas2d camera laws that drive the Scenes standalone emulator: the settled wheel-zoom and pan-drag dispatch.
//!
//! 🧭️ They name `handle_scene_wheel` and `handle_scene_pointer_move`, the test-only emulator of the Scenes standalone law, so they are mounted next to it by the Scenes companion.

use super::{handle_scene_pointer_move, handle_scene_wheel};
use semio_framework_os_renderer_wgpu_host_base::app_now_ms;
use semio_framework_os_renderer_wgpu_scene_state::{SceneDrag, SceneDragMode, camera_fixture_grant, camera_fixture_lease, clear_camera_fixture, mutate_scene_state, sweep_expired_scene_camera_dispatches};
use serde_json::Value;
use ui_wgpu::wgpu::{Rect, SurfaceKind, UiComponentSceneNode, UiPresence};

'''
newtext = header + scene_fn + "\n" + fixture_fn + "\n" + b1 + "\n" + b2
if DRY:
    print(newtext[:6000]); print(len(text)); sys.exit(0)
write_new(NEW, newtext)
backup(LAW)
assert read(LAW) != text
with open(LAW, "w", encoding="utf8") as h:
    h.write(text)
note(LAW)
print("edited", LAW[-60:])
