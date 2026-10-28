"""🧪 Ticket smoke script: print sizes and examples of the model core."""

import importlib.util
import json
import sys
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[7]
sys.path.insert(0, str(ROOT))
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

m = api.load_model()
g = api.building_geometry(m)
print("[DEBUG] geometry keys", list(g))
for key, value in g.items():
    print("[DEBUG]", key, len(value) if hasattr(value, "__len__") else value)
print("[DEBUG] storeys", g["storeys"])
print("[DEBUG] wall S", json.dumps(g["walls"][0], ensure_ascii=False)[:400])
print("[DEBUG] slabs", [(s["tag"], s["kind"], round(api.quantities(s["tag"], m)["area"], 3), s["thickness"]) for s in g["slabs"]])
print("[DEBUG] spaces", [(s["id"], s["area"]) for s in g["spaces"]])
c = api.clearance(m)
print("[DEBUG] clearance", {k: v for k, v in c.items() if k != "gaps"})
print("[DEBUG] gaps", [(x["row"], x["between"], x.get("clear_left"), x.get("clear_right")) for x in c["gaps"]])
p = api.car_path(m)
print("[DEBUG] car_path", p["entry_pair"], p["gap_progress"], p["gap"], p["length"], p["min_clearance"], len(p["poses"]), p["poses"][0], p["poses"][-1])
print("[DEBUG] 5.0 spacing", {k: v for k, v in api.clearance(m, 5.0).items() if k in ("spacing", "clear_width", "margin", "fits")})
print("[DEBUG] piloti", api.quantities("B3", m))
print("[DEBUG] type", {k: v for k, v in api.quantities("TY-PILOTI", m).items() if k != "occurrences"})
print("[DEBUG] wall q", api.quantities("W-S", m))
print("[DEBUG] roof q", api.quantities("SL-R", m), api.quantities("SL-1", m))
print("[DEBUG] chain", [x["class"] for x in api.class_chain("IfcColumn", m)])
print("[DEBUG] guid", api.global_id("B3"), api.decompress_guid(api.global_id("B3")) < 1 << 128)
print("[DEBUG] info", api.information("B3", "diameter", "en", m))
print("[DEBUG] info", api.information("B3", "Pset_ColumnCommon.LoadBearing", "de", m))
print("[DEBUG] info", api.information("W-S", "IsExternal", "en", m))
print("[DEBUG] info", api.information("B3", "grid.spacing", "de", m))
print("[DEBUG] relations", [(r["class"], r["role"], [o["tag"] for o in r["others"]][:3]) for r in api.relations_of("W-S", m)])
print("[DEBUG] partial", {k: v for k, v in api.partial_model("structure", m).items() if k not in ("element_guids", "element_tags")})
print("[DEBUG] represent", api.represent("W-S", "plan", m)["derived"])
ev = api.demo_event("sill", m)
new = api.apply_event(ev, m)
print("[DEBUG] sill event", new["revision"], api.quantities("W-S", new)["glazing_share"], api.quantities("W-S", m)["glazing_share"])
new = api.apply_event(api.demo_event("roof_soil", m), m)
print("[DEBUG] roof element", [(l["id"], l["thickness"]["value"]) for l in api.element("SL-R", new)["material"]["layers"]], new["revision"])
print("[DEBUG] total spaces fit", {s["tag"]: api.quantities(s["tag"], m).get("fit") for s in m["spaces"]})
