import importlib
import json
import sys
import time
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m = model.load_model()
g = model.building_geometry(m)
print("[DEBUG] grid", g["grid"]["x"], g["grid"]["y"], g["box"], g["storeys"])
c = model.clearance(m)
print("[DEBUG] clearance", json.dumps({k: v for k, v in c.items() if k != "gaps"}))
start = time.time()
for _ in range(5):
    model.clearance(m, spacing=4.6)
print("[DEBUG] clearance time", (time.time() - start) / 5)
p = model.car_path(m)
print("[DEBUG] path keys", {k: (v if k not in ("poses", "crossings") else len(v)) for k, v in p.items()})
print("[DEBUG] curtain", json.dumps(g["curtain_wall"])[:600])
print("[DEBUG] rels", [(r["class"], r["role"], r["relating"]["tag"], [o["tag"] for o in r["related"]]) for r in model.relations_of("E3", m)])
print("[DEBUG] piloti", [q for q in g["pilotis"] if q["tag"] in ("E3", "E4")])
print("[DEBUG] slabs", [(s["tag"], s["kind"], s["top"], s["thickness"], s["level"] if "level" in s else None) for s in g["slabs"]])
print("[DEBUG] q", model.quantities("E3", m))
print("[DEBUG] info", [model.information("E3", k, "en", m) for k in ("grid.spacing", "material", "Pset_ColumnCommon.LoadBearing", "Pset_ColumnCommon.IsExternal")])
ex = model.step_excerpt("E3", m)
print("[DEBUG] step", "\n".join(f"{i}: {l}" for i, l in enumerate(ex["lines"])))
print("[DEBUG] pset", json.dumps(model.element("E3", m)["psets"])[:900])
print("[DEBUG] spaces", [(s["id"], s["storey"], s["name"]["en"]) for s in g["spaces"]])
print("[DEBUG] ground walls", list(g.keys()))
