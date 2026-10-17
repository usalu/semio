"""🧹 rw2t step G: blank lines left by the removed mount regions, rustfmt-stable module order of the new roots, the ink import order of the Scenes companion."""
from ed import *
E_ = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SC = E_ + "/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu"
T_ = E_ + "/🎯️targets/🧊️wgpu"
def tail_fix(f, end):
    t = read(f)
    assert t.endswith(end + "\n\n"), f
    replace(f, [(end + "\n\n", end + "\n", 1)] if t.count(end + "\n\n") == 1 else [])
for d, end in [("🖌️canvas2d", "//#endregion Canvas2d"), ("🧩️board2d-and-vfs", "//#endregion VirtualFileSystem"), ("🧮️table-and-paint2d", "//#endregion Table"), ("📰️event-feed-and-timeline", "//#endregion GraphTimeline")]:
    f = SC + "/" + d + "/🦀️.rs"
    t = read(f)
    assert t.endswith(end + "\n\n") and t.count(end + "\n\n") >= 1
    replace_text = None
    backup(f)
    open(f, "w", encoding="utf8").write(t[:-1])
    note(f)
    print("tail", d)
replace(SC + "/🖋️ink-canvas-render/🦀️.rs", [("//#endregion InkCanvasRender\n\n\n//#region InkCanvasTests", "//#endregion InkCanvasRender\n\n//#region InkCanvasTests", 1)])
replace(SC + "/📰️event-feed-and-timeline/🦀️.rs", [("//#endregion EventFeed\n\n\n//#region GraphTimeline", "//#endregion EventFeed\n\n//#region GraphTimeline", 1)])
replace(SC + "/🧱️block-list-and-diff/🦀️.rs", [("//#endregion BlockList\n\n\n//#region DiffView", "//#endregion BlockList\n\n//#region DiffView", 1)])
INK = T_ + "/🧶️scene-ink-tests/🦀️.rs"
ink_text = '''//! 🧶️ Companion tests of the scene crates: the Canvas2d, raster frame cost, event feed and graph timeline laws.
//!
//! 🧭️ These laws name the frame action fixtures (`collect_fixture_actions`), the scene render wire and the retained UI engine next to the scene state, so they sit above all of them; each law file imports its names from the owning crates.

#![cfg(test)]

#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs"]
mod canvas2d_tests;
#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-event-feed/🦀️.rs"]
mod event_feed_tests;
#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs"]
mod graph_timeline_tests;
#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-raster-frame-cost/🦀️.rs"]
mod raster_frame_cost_tests;
'''
replace(INK, [(read(INK), ink_text, 1)])
RND = T_ + "/🪞️scene-render/🧪️tests/🦀️.rs"
rnd_text = '''//! 🪞️ Unit tests of scene-render: the table, block list and virtual file system laws that drive the scene pointer wire and the list transfer.
//!
//! 🧭️ The surface context menu and render entry laws stay mounted by the render entry; each law file imports its names from the owning crates.

#[path = "../../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-block-list/🦀️.rs"]
mod block_list_tests;
#[path = "../../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-table/🦀️.rs"]
mod table_tests;
#[path = "../../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-virtual-file-system/🦀️.rs"]
mod virtual_file_system_tests;
'''
replace(RND, [(read(RND), rnd_text, 1)])
replace(T_ + "/🧷️engine-attach-tests/🦀️.rs", [("ink_item_id, ink_items_at_point, ink_item_kind, ink_item_locked, ink_item_num,", "ink_item_id, ink_item_kind, ink_item_locked, ink_item_num, ink_items_at_point,", 1)])
