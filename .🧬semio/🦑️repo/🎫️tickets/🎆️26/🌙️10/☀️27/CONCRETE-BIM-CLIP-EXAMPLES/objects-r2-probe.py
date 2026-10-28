"""🔬 Print the model values the objects clip reads: python objects-r2-probe.py."""
import copy
import importlib
import json
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m = model.load_model()
print("[DEBUG] notes", json.dumps(m["notes"], ensure_ascii=False)[:3000])
print("[DEBUG] type", json.dumps(next(t for t in m["types"] if t["class"] == "IfcColumnType"), ensure_ascii=False)[:2500])
print("[DEBUG] B3", json.dumps(next(e for e in m["elements"] if e["tag"] == "B3"), ensure_ascii=False)[:2500])
print("[DEBUG] storeys", json.dumps(m["storeys"], ensure_ascii=False)[:1500])
print("[DEBUG] q B3", model.quantities("B3", m))
print("[DEBUG] q type", {k: v for k, v in model.quantities("TY-PILOTI", m).items() if k != "occurrences"})
print("[DEBUG] q SL-1", model.quantities("SL-1", m))
print("[DEBUG] q W-S", model.quantities("W-S", m))
print("[DEBUG] info", model.information("B3", "piloti.diameter", "en", m))
print("[DEBUG] info", model.information("B3", "height", "en", m))
print("[DEBUG] rel", [(r["class"], r["tag"], r["role"], r["relating"]["tag"], [x["tag"] for x in r["related"]]) for r in model.relations_of("W-S", m)])
print("[DEBUG] rel B3", [(r["class"], r["tag"], r["role"], r["relating"]["tag"], [x["tag"] for x in r["related"]]) for r in model.relations_of("B3", m)])
print("[DEBUG] rel OP-S", [(r["class"], r["tag"], r["role"], r["relating"]["tag"], [x["tag"] for x in r["related"]]) for r in model.relations_of("OP-S", m)])
print("[DEBUG] rel RW-S", [(r["class"], r["tag"], r["role"], r["relating"]["tag"], [x["tag"] for x in r["related"]]) for r in model.relations_of("RW-S", m)])
print("[DEBUG] rel SL-1", [(r["class"], r["tag"], r["role"], r["relating"]["tag"], [x["tag"] for x in r["related"]]) for r in model.relations_of("SL-1", m)][:5])
print("[DEBUG] body", json.dumps(model.represent("B3", "body", m), ensure_ascii=False)[:1200])
print("[DEBUG] axis", json.dumps(model.represent("B3", "axis", m), ensure_ascii=False)[:800])
print("[DEBUG] foot", json.dumps(model.represent("B3", "footprint", m), ensure_ascii=False)[:800])
g = model.building_geometry(m)
print("[DEBUG] geo keys", list(g.keys()))
print("[DEBUG] storeys", g["storeys"], "box", g["box"])
print("[DEBUG] slab", [{k: v for k, v in s.items() if k not in ("outline", "voids")} for s in g["slabs"]])
print("[DEBUG] pilotis0", g["pilotis"][0], len(g["pilotis"]))
print("[DEBUG] wall", {k: v for k, v in next(w for w in g["walls"] if w["tag"] == "W-S").items()})
print("[DEBUG] grid", g["grid"])
d = copy.deepcopy(m)
t = next(t for t in d["types"] if t["class"] == "IfcColumnType")
t["profile"]["diameter"]["value"] = 0.4
t0 = time.time()
q = model.quantities("TY-PILOTI", d)
print("[DEBUG] what-if 0.4", {k: v for k, v in q.items() if k != "occurrences"}, model.quantities("B3", d)["volume"], time.time() - t0)
print("[DEBUG] classes", list(m["classes"].keys()))
print("[DEBUG] tags walls", [(e["tag"], e["class"]) for e in m["elements"] if e["class"] in ("IfcWall", "IfcWindow", "IfcOpeningElement")])
print("[DEBUG] spaces", [(s["tag"], s["name"]["en"]) for s in m["spaces"]])
