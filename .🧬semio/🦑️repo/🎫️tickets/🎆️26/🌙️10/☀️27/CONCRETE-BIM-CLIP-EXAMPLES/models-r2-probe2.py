import importlib, json, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8")
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
t = M.translate_structure()
for g in t["loss_groups"]:
    print("[DEBUG] LG", g["id"], g["count"], g["source_tags"][:6], g["en"], "|", g["de"])
for a in t["assumptions"]:
    print("[DEBUG] AS", a["id"], a["en"], "|", a["de"])
print("[DEBUG] mesh", t["mesh"])
print("[DEBUG] nodes", t["nodes"][:3], t["nodes"][-2:])
print("[DEBUG] svc", [(s["tag"], s["kind"], s["points"]) for s in M.building_geometry()["services"]])
m = M.load_model()
print("[DEBUG] notes", [(n["id"], n["text"]["en"]) for n in m["notes"]])
print("[DEBUG] storey names", m["storeys"])
print("[DEBUG] ramp", json.dumps(M.building_geometry()["ramp"])[:1800])
print("[DEBUG] void", [s["voids"] for s in M.building_geometry()["slabs"]])
print("[DEBUG] spaces", [(s["id"], s["storey"], s["area"], s["name"]["en"]) for s in M.building_geometry()["spaces"]])
print("[DEBUG] translation", json.dumps(m["translation"], ensure_ascii=False)[:1800])
print("[DEBUG] sections", M.element("SL-1")["psets"])
