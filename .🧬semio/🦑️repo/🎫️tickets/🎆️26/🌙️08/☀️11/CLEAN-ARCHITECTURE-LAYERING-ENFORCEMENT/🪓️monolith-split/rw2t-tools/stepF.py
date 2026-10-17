"""🧪 rw2t step F: the engine-board context menu helpers the surface context menu law names are testkit items; table-and-paint2d imports the Viewport its navigator fixtures return."""
from mf import *
E_ = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
P2D = E_ + "/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🖌️paint2d/🦀️.rs"
G = '#[cfg(any(test, feature = "testkit"))]'
replace(P2D, [
 ('#[cfg(test)]\npub fn pick_target_hits_for_test(', G + '\npub fn pick_target_hits_for_test(', 1),
 ('#[cfg(test)]\npub fn selection_domains_for_test(', G + '\npub fn selection_domains_for_test(', 1),
])
BM = mpath("🎲️engine-board")
t = read(BM)
UL = 'semio-framework-ui-layout = { path = "../../../../../../../../../../🔨️modules/🖱️ui/📜️declaration/📐️layout/📦️packages/🦀️rust" }'
assert t.count(UL) == 1
t2 = t.replace("\n[dependencies]\n", "\n[features]\ntestkit = []\n\n[dependencies]\n", 1).replace("\n[dev-dependencies]\n" + UL + "\n", "\n", 1)
t2 = t2.replace("semio-framework-surface = {", UL + "\nsemio-framework-surface = {", 1)
replace_text(BM, t, t2.rstrip("\n") + "\n")
add_lines(mpath("🪞️scene-render"), "dev-dependencies", [FP + 'engine-board = { path = "../../../🎲️engine-board/📦️packages/🦀️rust", features = ["testkit"] }'])
CTX = E_ + "/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-surface-context-menu/🦀️.rs"
replace(CTX, [
 ("use semio_framework_os_renderer_wgpu_scene_tables::table_metrics;", "use semio_framework_os_renderer_wgpu_engine_board::paint2d::{pick_target_hits_for_test, selection_domains_for_test};\nuse semio_framework_os_renderer_wgpu_scene_tables::table_metrics;", 1),
 ("engine_canvas::selection_domains_for_test", "selection_domains_for_test", 4),
 ("engine_canvas::pick_target_hits_for_test", "pick_target_hits_for_test", 2),
])
TBL = E_ + "/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧮️table-and-paint2d/🦀️.rs"
replace(TBL, [
 ("use super::*;\n\n//#region Paint2d\n", 'use super::*;\n#[cfg(any(test, feature = "testkit"))]\nuse semio_framework_os_renderer_wgpu_scene_state::Viewport;\n\n//#region Paint2d\n', 1),
])
