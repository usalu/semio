import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = m.load_model()
g = m.building_geometry(model)
print("[DEBUG] geometry keys", list(g))
print("[DEBUG] b3", next(p for p in g["pilotis"] if p["tag"] == "B3"))
print("[DEBUG] slab", {k: v for k, v in next(s for s in g["slabs"] if s["tag"] == "SL-1").items() if k != "outline"})
mesh = m.mesh("B3", 8, model)
print("[DEBUG] mesh", {k: v for k, v in mesh.items() if k not in ("vertices", "faces")})
print("[DEBUG] verts", mesh["vertices"][:4], mesh["faces"][:3])
tr = m.translate_structure(model)
print("[DEBUG] tr keys", list(tr))
print("[DEBUG] mid", tr["mid_surface"], tr["mesh"], tr["slab"], tr["counts"])
print("[DEBUG] shell0", tr["shells"][0])
print("[DEBUG] member0", tr["members"][0], tr["members"][-1])
print("[DEBUG] support0", tr["supports"][0])
print("[DEBUG] loads", tr["loads"])
print("[DEBUG] mapping", tr["mapping"])
print("[DEBUG] loss_groups", tr["loss_groups"][:3])
trib = m.tributary(model)
print("[DEBUG] trib keys", list(trib), trib["area_load"], trib["load_breakdown"], trib["cantilever"])
for p in trib["pilotis"]:
    if p["tag"] in ("B3", "A3", "A1", "D3", "C3"):
        print("[DEBUG]", {k: v for k, v in p.items() if k not in ("cell", "pieces", "zones")})
print("[DEBUG] ids", {k: v for k, v in m.ids_check(model).items() if k not in ("items",)})
for key in ("architecture", "structure", "services"):
    pm = m.partial_model(key, model)
    print("[DEBUG] partial", key, len(pm["element_tags"]), pm["counts_by_class"])
print("[DEBUG] quantities", m.quantities("B3", model), m.quantities("SL-1", model))
print("[DEBUG] element B3", {k: v for k, v in m.element("B3", model).items() if k in ("material", "psets", "geometry", "type")})
