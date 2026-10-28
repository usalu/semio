import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = m.load_model()
t = m.tributary(model)
for p in t["pilotis"]:
    if p["tag"] in ("B3", "A1", "A3"):
        print("[DEBUG]", p["tag"], p.get("cell"), p.get("pieces"))
print("[DEBUG]", t["zones"], t["cuts"], t["total_reaction"], t["slab_area"])
g = m.building_geometry(model)
print("[DEBUG] cw", g["curtain_wall"])
print("[DEBUG] ramp", {k: v for k, v in g["ramp"].items() if k != "ramps"})
