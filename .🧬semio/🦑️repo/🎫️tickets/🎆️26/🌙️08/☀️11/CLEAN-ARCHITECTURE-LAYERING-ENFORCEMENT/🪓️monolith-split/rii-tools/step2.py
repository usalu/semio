"""✏️ rii step 2a: the surface-context-menu law reads `pick_target_hits` / `selection_domains` directly (both `pub`), so engine-board loses its `testkit` feature and the `*_for_test` wrappers, and scene-render loses the dev edge to it. usage: step2.py [--dry]"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ed import *
DRY = "--dry" in sys.argv
E = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
T = E + "/🎯️targets/🧊️wgpu"
B = E + "/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu"
P = B + "/🖌️paint2d/🦀️.rs"
replace(P, [
    ("fn pick_target_hits(json: &str) -> Vec<ui_wgpu::wgpu::ContextMenuHit> {", "pub fn pick_target_hits(json: &str) -> Vec<ui_wgpu::wgpu::ContextMenuHit> {", 1),
    ("\nfn selection_domains(json: &str) -> (Vec<String>, Vec<String>, Vec<String>) {", "\npub fn selection_domains(json: &str) -> (Vec<String>, Vec<String>, Vec<String>) {", 1),
    ("/// 🧪️ `pick_target_hits`, for the surface-context-menu law (this module's own tests never construct\n"
     "/// a live engine host, and the JSON shape is the contract that drifts).\n"
     "#[cfg(any(test, feature = \"testkit\"))]\n"
     "pub fn pick_target_hits_for_test(json: &str) -> Vec<semio_framework_ui_layout::layout::ContextMenuHit> {\n"
     "    pick_target_hits(json)\n"
     "}\n\n"
     "/// 🧪️ `selection_domains`, for the surface-context-menu law.\n"
     "#[cfg(any(test, feature = \"testkit\"))]\n"
     "pub fn selection_domains_for_test(json: &str) -> (Vec<String>, Vec<String>, Vec<String>) {\n"
     "    selection_domains(json)\n"
     "}\n", "", 1),
], dry=DRY)
LAW = E + "/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-surface-context-menu/🦀️.rs"
text = read(LAW)
n1, n2 = text.count("pick_target_hits_for_test"), text.count("selection_domains_for_test")
print("law occurrences", n1, n2)
replace(LAW, [("pick_target_hits_for_test", "pick_target_hits", n1), ("selection_domains_for_test", "selection_domains", n2)], dry=DRY)
