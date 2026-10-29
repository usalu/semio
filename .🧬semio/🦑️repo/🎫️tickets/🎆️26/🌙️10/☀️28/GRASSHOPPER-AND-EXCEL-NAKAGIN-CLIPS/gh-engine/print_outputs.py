import importlib.util
import json
import sys
from pathlib import Path

ROOT = Path(r"C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🐍️.py")
spec = importlib.util.spec_from_file_location("ghm", ROOT)
m = importlib.util.module_from_spec(spec)
sys.modules["ghm"] = m
spec.loader.exec_module(m)
out = {}
cap = {"w": 2.5, "l": 4.0, "h": 2.5, "r": 0.25, "d": 1.3, "bu": 0.9, "bvl": 0.5, "bvh": 2.0, "side": 5.5}
s = m.evaluate(m.canvas("capsule"), cap)
for oid, name in (("fvp", None), ("bolt", None), ("rot", "Geometry"), ("cut", "Result"), ("cir", None), ("lp", None), ("ori", "Geometry")):
    out[oid] = m.plain(s.tree(oid, name))
tw = {"side": 5.5, "sh": 2.75, "n": 11, "ps": 2, "pitch": 2.75, "ow": 0.25, "ch": 2.75, "pw": 31.4, "pd": 20.7}
t = m.evaluate(m.canvas("tower"), tw)
for oid, name in (("tot", None), ("core", None), ("capital", None), ("ext", None), ("stl", None), ("stc", None)):
    out["tower." + oid] = m.plain(t.tree(oid, name))
out["tower.core.volume"] = t.value("core").volume
out["tower.capital.volume"] = t.value("capital").volume
Path(sys.argv[1]).write_text(json.dumps(out, ensure_ascii=False, indent=1), encoding="utf-8")
