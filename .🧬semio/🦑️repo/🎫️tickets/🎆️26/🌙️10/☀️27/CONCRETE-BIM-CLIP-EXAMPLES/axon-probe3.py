import importlib, sys
from pathlib import Path
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
g = m.building_geometry()
for s in g["spaces"]:
    print(s["id"], s["predefined_type"], s["storey"])
mod = m.load_model()
e = m.element("SL-R")
print([ (l["id"], l["thickness"]["value"] if isinstance(l["thickness"], dict) else l["thickness"]) for l in e["material"]["layers"]])
print(sorted(set(k for s in g["slabs"] for k in s)))
print([ (s["tag"], s["name"]) for s in g["spaces"]][:1])
import time
t=time.time(); 
for _ in range(10): m.building_geometry()
print("geom ms", (time.time()-t)*100)
print(g["slabs"][1].get("zones"), g["slabs"][1].get("total_thickness"))
