import importlib.util, sys
from pathlib import Path
ROOT = Path(r"C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🐍️.py")
spec = importlib.util.spec_from_file_location("ghm", ROOT); m = importlib.util.module_from_spec(spec); sys.modules["ghm"] = m; spec.loader.exec_module(m)
base = m.canvas("anatomy")
def show(tag, d):
    s = m.evaluate(d)
    print(tag, {o: (s.state(o), [x["en"] for x in s.msgs(o)]) for o in d.objects if s.state(o) != "ok" or o in ("cb",)})
S = lambda i, **k: {"id": i, "component": "Number Slider", "at": [0, 0], "settings": k}
objs = [S("w", label="Width", min=1, max=5, value=2.5, rounding="float", decimals=1), S("bl", label="L", min=2, max=6, value=4.0, rounding="float", decimals=1),
        {"id": "tl", "component": "Panel", "at": [0,0], "settings": {"lines": ["tall"]}}, {"id": "cb", "component": "Center Box", "at": [0,0], "settings": {}}, {"id": "p", "component": "Panel", "at":[0,0], "settings": {"lines": [""]}},
        {"id": "se", "component": "Series", "at": [0,0], "settings": {}}, {"id": "sp", "component": "Panel", "at":[0,0], "settings": {"lines": [""]}}]
for tag, w in (("textZ", ["w.Number>cb.X", "bl.Number>cb.Y", "tl.Stream>cb.Z", "cb.Box>p.Stream"]), ("noZ", ["w.Number>cb.X", "bl.Number>cb.Y", "cb.Box>p.Stream"]),
               ("textCount", ["w.Number>cb.X", "tl.Stream>se.Count", "se.Series>sp.Stream", "cb.Box>p.Stream"])):
    show(tag, m.define({"objects": objs, "wires": w}))
s = m.evaluate(base)
for o in ("wp","vp","sp","tp"):
    print(o, m.panel_rows(s.tree(o)))
print(m.panel_rows(m.evaluate(base, {"cnt": 9}).tree("sp")))
print(m.aggregate_tree().paths()[:4])
print("----")
objs[2]["settings"]["lines"] = ["2.5", "tall"]
d = m.define({"objects": objs, "wires": ["w.Number>cb.X", "bl.Number>cb.Y", "tl.Stream>cb.Z", "cb.Box>p.Stream"]})
s = m.evaluate(d); show("mixed", d); print(m.panel_rows(s.tree("p")), [ (x['en'],x['de']) for x in s.msgs('cb')])
