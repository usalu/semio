import importlib, json, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
t = M.translate_structure()
for g in t["loss_groups"]:
    print(g["id"], g["count"], g["source_tags"][:3], "|", g["en"], "|", g["de"])
print(json.dumps(t["assumptions"], ensure_ascii=False))
print([ (s["tag"], s.get("diameter"), s.get("size")) for s in M.building_geometry()["services"]])
print([(s["tag"], s["name"], s["storey"], s["area"]) for s in M.building_geometry()["spaces"]])
print(M.partial_model("architecture")["element_tags"])
print(M.element("A1")["psets"])
print(M.information("C3", "LoadBearing", "en"))
print(M.element("DP-1")["geometry"], M.element("DP-1")["attributes"], M.element("DP-1")["name"])
print(M.element("RAD-1")["name"], M.element("BL-1")["name"], M.element("DR-1")["name"])
print(t["loads"]["area_load"], t["counts"])
print(M.quantity("SL-1","thickness"), M.clearance()["spacing"])
print([ (k, v) for k, v in M.load_model()["notes"].items()] if isinstance(M.load_model()["notes"], dict) else M.load_model()["notes"])
