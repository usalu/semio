import importlib
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m = M.load_model()
print("top keys", list(m))
for name in ("architecture", "structure", "services"):
    info = M.partial_model(name)
    print(name, info["author"], info["revision"], info["name"], info["counts_by_class"], len(info["element_guids"]))
a, s, v = (set(M.partial_model(n)["element_guids"]) for n in ("architecture", "structure", "services"))
print("shared a&s", len(a & s), [M.element(g)["tag"] for g in a & s], "a&v", len(a & v), "s&v", len(s & v))
print("translation", json.dumps(m.get("translation"), ensure_ascii=False)[:1500])
g = M.building_geometry()
print("geometry keys", list(g))
print("services", [(x["tag"], x["class"], x["kind"], x["points"]) for x in g["services"]])
print("spaces", [(x["id"], x["storey"], round(x["area"], 1)) for x in g["spaces"]])
print("slabs", [(x["tag"], x["kind"], x["top"], x["thickness"]) for x in g["slabs"]])
print("hasattr", [n for n in ("translate_structure", "clash_check", "tributary", "step_excerpt") if hasattr(M, n)])
e = M.element("W-S")
print("W-S", json.dumps({k: e[k] for k in e if k in ("material", "type", "psets", "attributes")}, ensure_ascii=False)[:1500])
print("type keys", list(m["types"]) if isinstance(m["types"], dict) else [t.get("id") for t in m["types"]])
print("building", m["building"])
print("link", json.dumps(m.get("links"), ensure_ascii=False)[:600])
print("loads", json.dumps(m.get("loads"), ensure_ascii=False)[:600])
print("wall quantities", M.quantities("W-S"))
print("elem keys", [k for k in m["elements"][0]])
