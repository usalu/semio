import importlib, json, sys
from pathlib import Path
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m = model.load_model()
print("[DEBUG] chain", [(c["class"], c["adds"]) for c in model.class_chain("IfcColumn", m)])
print("[DEBUG] chain win", [(c["class"]) for c in model.class_chain("IfcWindow", m)])
print("[DEBUG] has tributary", hasattr(model, "tributary"))
t = model.tributary(m)
b3 = next(p for p in t["pilotis"] if p["tag"] == "B3")
print("[DEBUG] trib B3", {k: v for k, v in b3.items() if k != "cell"}, t["slab_area"], t["method"])
print("[DEBUG] elements", len(m["elements"]), len({e["guid"] for e in m["elements"]}))
from collections import Counter
print("[DEBUG] by class", Counter(e["class"] for e in m["elements"]))
print("[DEBUG] project", m["project"]["name"], m["site"]["name"], m["building"]["name"])
print("[DEBUG] building", {k: v for k, v in m["building"].items() if k != "box"})
print("[DEBUG] rels", [(r["tag"], r["class"], r["relating"] if "relating" in r else None) for r in m["relations"] if r["class"] in ("IfcRelAggregates","IfcRelContainedInSpatialStructure")])
print("[DEBUG] wall type", json.dumps(next(t for t in m["types"] if t["tag"]=="TY-WALL"), ensure_ascii=False)[:600])
print("[DEBUG] mat", json.dumps(m["materials"], ensure_ascii=False)[:1500])
print("[DEBUG] step", model.step_excerpt("B3", m)["lines"])
print("[DEBUG] ids", model.ids_check(m) if hasattr(model, "ids_check") else None)
