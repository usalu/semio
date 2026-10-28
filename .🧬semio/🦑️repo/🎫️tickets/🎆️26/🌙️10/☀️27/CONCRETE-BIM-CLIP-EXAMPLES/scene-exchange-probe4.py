import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = m.load_model()
g = m.building_geometry(model)
for s in g["spaces"]:
    print("[DEBUG] space", {k: v for k, v in s.items() if k not in ("outline",)} )
print("[DEBUG] grid", g["grid"])
print("[DEBUG] links", [ (l["id"], l["kind"], l["object_guid"]) for l in model["links"]])
t = m.tributary(model)
b3 = next(p for p in t["pilotis"] if p["tag"] == "B3")
print("[DEBUG] cantilever", t["cantilever"], t["method"])
print("[DEBUG] names", model["elements"][0].keys())
print("[DEBUG] slab elem", {k: v for k, v in m.element("SL-1", model).items() if k in ("name","psets","geometry")})
