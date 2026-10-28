import importlib, sys, json
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
a = M.translate_structure()
print(sorted(a), len(a["nodes"]), len(a["shells"]), len(a["members"]), len(a["supports"]))
print(a["nodes"][0], a["nodes"][-1])
print(a["members"][0]); print(a["members"][-1]); print(a["supports"][0]); print(a["shells"][0])
print(a["loads"]["area_load"], list(a["loads"]), a["point_loads"][:1], a["source_revision"], a["mesh"])
t = M.tributary()
print(sorted(t)); p = t["pilotis"]
print(p[0]); print(p[7]["cell"], p[7]["pieces"])
print(t["cantilever"], t["cuts"])
print([ (q["tag"], q["kind"], round(q["area"],1), round(q["reaction"])) for q in p])
print(hasattr(M, "mesh"), hasattr(M, "step_excerpt"))
g = M.building_geometry()
print(g["slabs"][0]["outline"], g["slabs"][0]["voids"], g["storeys"])
