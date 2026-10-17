"""🧷 rw2t step E: the moved laws leave the lib files that mounted them and are mounted by their new hosts."""
import sys
from ed import *
E_ = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SC = E_ + "/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu"
T_ = E_ + "/🎯️targets/🧊️wgpu"
def region(name, law, mod):
    return f'\n//#region {name}\n#[cfg(test)]\n#[path = "../../../🧪️tests/{law}/🦀️.rs"]\nmod {mod};\n//#endregion {name}\n'
cuts = [
 (SC + "/🖌️canvas2d/🦀️.rs", region("Canvas2dTests", "🔬️wgpu-canvas2d", "canvas2d_tests")),
 (SC + "/🖋️ink-canvas-render/🦀️.rs", region("RasterFrameCostTests", "🔬️wgpu-raster-frame-cost", "raster_frame_cost_tests")),
 (SC + "/📰️event-feed-and-timeline/🦀️.rs", region("EventFeedTests", "🔬️wgpu-event-feed", "event_feed_tests")),
 (SC + "/📰️event-feed-and-timeline/🦀️.rs", region("GraphTimelineTests", "🔬️wgpu-graph-timeline", "graph_timeline_tests")),
 (SC + "/🧩️board2d-and-vfs/🦀️.rs", region("VirtualFileSystemTests", "🔬️wgpu-virtual-file-system", "virtual_file_system_tests")),
 (SC + "/🧮️table-and-paint2d/🦀️.rs", region("TableTests", "🔬️wgpu-table", "table_tests")),
 (SC + "/🧱️block-list-and-diff/🦀️.rs", region("BlockListTests", "🔬️wgpu-block-list", "block_list_tests")),
]
for f, anchor in cuts:
    replace(f, [(anchor, "\n", 1)])

INK = T_ + "/🧶️scene-ink-tests/🦀️.rs"
ink_text = '''//! 🧶️ Companion tests of the scene crates: the Canvas2d, raster frame cost, event feed and graph timeline laws.
//!
//! 🧭️ These laws name the frame action fixtures (`collect_fixture_actions`), the scene render wire and the retained UI engine next to the scene state, so they sit above all of them; each law file imports its names from the owning crates.

#![cfg(test)]

#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs"]
mod canvas2d_tests;
#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-raster-frame-cost/🦀️.rs"]
mod raster_frame_cost_tests;
#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-event-feed/🦀️.rs"]
mod event_feed_tests;
#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs"]
mod graph_timeline_tests;
'''
RND = T_ + "/🪞️scene-render/🧪️tests/🦀️.rs"
rnd_text = '''//! 🪞️ Unit tests of scene-render: the table, block list and virtual file system laws that drive the scene pointer wire and the list transfer.
//!
//! 🧭️ The surface context menu and render entry laws stay mounted by the render entry; each law file imports its names from the owning crates.

#[path = "../../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-table/🦀️.rs"]
mod table_tests;
#[path = "../../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-block-list/🦀️.rs"]
mod block_list_tests;
#[path = "../../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-virtual-file-system/🦀️.rs"]
mod virtual_file_system_tests;
'''
replace(INK, [(read(INK), ink_text, 1)])
replace(RND, [(read(RND), rnd_text, 1)])
