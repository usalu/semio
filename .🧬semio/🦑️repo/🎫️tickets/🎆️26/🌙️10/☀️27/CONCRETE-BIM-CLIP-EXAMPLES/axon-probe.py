import importlib, json, sys, os
from pathlib import Path
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
g = m.building_geometry()
def short(o, n=3):
    return json.dumps(o)[:1500]
for k in ("box", "storeys", "grid"):
    print(k, short(g[k]))
print("pilotis", [(p["tag"], p["x"], p["y"], p["top"]) for p in g["pilotis"]][:6])
for s in g["slabs"]:
    print("slab", s["tag"], s["kind"], s["outline"], s["voids"], s["top"], s["thickness"], s.get("zones"), [ (l["id"], l["thickness"], l.get("zones")) for l in s.get("layers", [])])
for w in g["walls"]:
    print("wall", w["tag"], w["start"], w["end"], w["thickness"], w["base"], w["top"], w["openings"])
print("curtain", g["curtain_wall"]["path"][:3], g["curtain_wall"]["path"][-3:], g["curtain_wall"]["centre"], g["curtain_wall"]["radius"], g["curtain_wall"]["base"], g["curtain_wall"]["top"])
for r in g["ramp"]["ramps"]:
    print("ramp", r["tag"], r["width"], r["flights"], r["landings"])
print("voids", g["ramp"]["voids"])
for s in g["screens"]:
    print("screen", s["tag"], s["path"], s["base"], s["top"], s["thickness"])
for s in g["spaces"]:
    print("space", s["id"], s["storey"], s["outline"], s["area"], s["name"])
for s in g["services"]:
    print("service", s["tag"], s["class"], s["kind"], s.get("system"), s["points"], s.get("size"), s.get("diameter"))
