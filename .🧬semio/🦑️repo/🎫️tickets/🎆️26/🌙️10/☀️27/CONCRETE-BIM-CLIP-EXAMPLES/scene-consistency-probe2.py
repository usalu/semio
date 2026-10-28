import importlib, json, sys
from pathlib import Path
T = Path(__file__).resolve().parent
sys.path.insert(0, str(next(p for p in T.parents if (p / "🧰️framework").is_dir())))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
base = m.load_model()
new = m.apply_event(m.demo_event("roof_soil"), base)
print(len(base["links"]), m.consistency_issues(new)[:3], m.consistency_issues(base))
t = m.tributary(new, "roof"); o = m.tributary(base, "roof")
print(t.keys())
print([(z["id"], z["area"], z["area_load"], len(z["pieces"]), z["outline"][:2] if z["outline"] else None) for z in t["zones"]])
for p in t["pilotis"]:
    if p["tag"][1]=="3": print(p["tag"], p["area"], o["pilotis"][t["pilotis"].index(p)]["reaction"], p["reaction"], {k:(v["area"], v["reaction"]) for k,v in p["zones"].items()})
print(m.link_values(new,"roof-load").keys().__len__())
