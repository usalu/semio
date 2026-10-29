import importlib
import sys
import time
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
gh = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
d = gh.canvas("capsule")
t0 = time.time()
s = gh.evaluate(d, {"w": 2.8})
print("[DEBUG] evaluate", time.time() - t0)
print(s.tree("cut").items()[0].root_box())
print(nk.towers()[1])
for sl in nk.slots():
    if sl.tower == 1 and sl.storey < 2 and sl.item == 0:
        print(sl.path, sl.type, sl.rotation, sl.plane.origin, sl.plane.x, sl.facade_plane.origin)
wrap = gh.define({"objects": [
    {"id": "types", "component": "Brep Param", "settings": {"label": "Types", "data": [{"capsule": k} for k in "ABCD"]}},
    {"id": "idx", "component": "Panel", "settings": {"lines": ["0", "1", "2", "3", "4", "5", "6", "7"]}},
    {"id": "wr", "component": "Boolean Toggle", "settings": {"value": True}},
    {"id": "li", "component": "List Item", "settings": {}}],
    "wires": ["types.Brep>li.List", "idx.Stream>li.Index", "wr.Boolean>li.Wrap"]})
for wr in (True, False):
    sol = gh.evaluate(wrap, {"wr": wr})
    print(wr, sol.state("li"), [m for m in sol.msgs("li")], [i.tag if i is not None else None for i in sol.tree("li").items()])
