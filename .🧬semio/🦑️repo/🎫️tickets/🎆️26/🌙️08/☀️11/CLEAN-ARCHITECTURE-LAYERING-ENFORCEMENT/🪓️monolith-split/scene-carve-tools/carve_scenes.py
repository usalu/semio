"""🎬 Carves scenes.rs and its siblings into the doc, world3d, node-graph, plane and views crates and re-homes the pack, modelling and surface edges (staged)."""
import os, re, sys
from lib import *
from model import *
from synth import *
from rsitems import split_items, regions, region_text
import carve_math

SCENES_HEADER = """use protocol::value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_ui_viewport::Viewport2d;
use serde::{Deserialize, Serialize};
"""

DOCS = {
    "doc": "📜️ The scene payload contract: the `SceneDoc` trait every product scene implements, the lane carriers and references, the pack wire macro and the shared value-codec primitives.",
    "world3d": "🌍️ The world-3d scene payload with its presentation and typed lanes, the modelling builders and the world snapshot lease.",
    "node-graph": "🪢️ The node-graph scene payload with its port, node, edge, operator and hover records.",
    "plane": "🖼️ The flat-surface scene payloads (canvas, paint, board, tiled map, ink canvas) with the canvas framing and snapshot lease.",
    "views": "📄️ The document-style scene payloads (text editor, table, block list, diff view, event feed, graph timeline, virtual file system, icon render) with the draft text splice.",
}
ROOT_EMOJI2 = {"doc": "📜️", "world3d": "🌍️", "node-graph": "🪢️", "plane": "🖼️", "views": "📄️"}


def cut_items(text, pred):
    items = split_items(text)
    lines = text.split("\n")
    removed = []
    drop = set()
    for it in items:
        if pred(it):
            removed.append(it)
            for i in range(it.start, it.end + 1):
                drop.add(i)
            j = it.end + 1
            while j < len(lines) and not lines[j].strip():
                drop.add(j)
                j += 1
                break
    kept = [l for i, l in enumerate(lines) if i not in drop]
    return "\n".join(kept), removed


def unqual(text):
    out = []
    for l in text.split("\n"):
        if l.lstrip().startswith("//"):
            out.append(l)
        else:
            out.append(re.sub(r"(?<![\w:])crate::(?:math::|pack::)?(?=\w)", "", l))
    return "\n".join(out)


def widen_fns(text, names):
    for n in names:
        assert len(re.findall(r"^fn " + n + r"\b", text, re.M)) == 1, n
        text = re.sub(r"^fn " + n + r"\b", "pub fn " + n, text, flags=re.M)
    return text


def scenes_bodies():
    S = base("🎬️scenes/🦀️.rs")
    rt = lambda n: region_text(S, n)
    sd = rt("🔖️SceneDoc")
    assert sd.count("macro_rules! scene_pack_wire") == 1
    sd = sd.replace("macro_rules! scene_pack_wire", "#[macro_export]\nmacro_rules! scene_pack_wire")
    sd = unqual(sd)
    vh = rt("🔖️ValueCodecHelpers")
    vh = widen_fns(vh, ["value_field", "value_required", "value_decode", "value_decode_option", "value_decode_default", "value_push", "value_push_option", "value_push_if_nonempty"])
    w3, rem1 = cut_items(rt("🔖️World3dScene"), lambda it: (it.kind == "struct" and it.name == "SceneLaneRef") or (it.kind == "impl" and "for SceneLaneRef" in it.name))
    assert len(rem1) == 3, [r.name for r in rem1]
    w3l, rem2 = cut_items(rt("🔖️World3dSceneLanes"), lambda it: it.kind == "fn" and it.name == "scene_lane_hash")
    assert len(rem2) == 1
    ink, rem3 = cut_items(rt("🔖️InkCanvasScene"), lambda it: it.kind == "fn" and it.name == "deserialize_non_empty_interaction_domain_text")
    assert len(rem3) == 1
    lane_items = [rem1[0], rem1[1], rem1[2], rem2[0], rem3[0]]
    parts = []
    for it in lane_items:
        t = it.text
        if it.kind == "fn":
            t = re.sub(r"^((?:\s*///[^\n]*\n|\s*//[^\n]*\n|\s*#\[[^\n]*\]\n)*)pub fn |^((?:\s*///[^\n]*\n|\s*//[^\n]*\n|\s*#\[[^\n]*\]\n)*)fn ", lambda m: (m.group(1) or m.group(2) or "") + "pub fn ", t, count=1)
        parts.append(t)
    lane = "//#region 🔖️SceneLane\n" + "\n\n".join(parts) + "\n//#endregion 🔖️SceneLane"
    doc_body = "\n\n".join([sd, vh, lane])
    bodies = {"doc": doc_body}
    modelling_src = base("📏️world3d-modelling/🦀️.rs")
    mod_new, rem4 = cut_items(modelling_src, lambda it: it.kind == "impl" and it.name.startswith("impl World3dScene {"))
    assert len(rem4) == 1
    builders = "//#region 🔖️ModellingBuilders\n" + rem4[0].text + "\n//#endregion 🔖️ModellingBuilders"
    bodies["world3d"] = unqual("\n\n".join([w3, w3l, builders]))
    ng = unqual("\n\n".join([rt("🔖️NodeGraphRecords"), rt("🔖️NodeGraphScene")]))
    bodies["node-graph"] = ng
    plane = unqual("\n\n".join([rt("🔖️Canvas2dScene"), rt("🔖️Paint2dScene"), rt("🔖️TiledMapScene"), rt("🔖️Board2dScene"), ink]))
    bodies["plane"] = plane
    bl = rt("🔖️BlockListScene")
    assert "pub use block_list::*;" in bl
    views = unqual("\n\n".join([rt("🔖️TextEditorScene"), rt("🔖️TableScene"), rt("🔖️IconRenderScene"), rt("🔖️VirtualFileSystemScene"), rt("🔖️GraphTimelineScene"), rt("🔖️DiffViewScene"), rt("🔖️EventFeedScene")]))
    return bodies, views, bl, mod_new, S


if __name__ == "__main__":
    bodies, views, bl, mod_new, S = scenes_bodies()
    for k, v in bodies.items(): print(k, len(v.split("\n")))
    print("views", len(views.split("\n")))
    print(bodies["doc"][:300])
