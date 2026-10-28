import importlib, json, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
for n in ("architecture", "structure", "services"):
    p = M.partial_model(n)
    print("[DEBUG]", n, {k: v for k, v in p.items() if k not in ("element_tags", "element_guids")}, sum(p["counts_by_class"].values()), len(p["element_tags"]))
g = M.building_geometry()
print("[DEBUG] geometry keys", list(g.keys()))
for k, v in g.items():
    if isinstance(v, list):
        print("[DEBUG]", k, len(v), json.dumps(v[0], ensure_ascii=False)[:400] if v else "")
    elif isinstance(v, dict):
        print("[DEBUG]", k, json.dumps(v, ensure_ascii=False)[:400])
t = M.translate_structure()
print("[DEBUG] counts", t["counts"], t["mesh"].keys() if hasattr(t["mesh"], "keys") else t["mesh"])
print("[DEBUG] slab", t["slab"], t["mid_surface"], t["loads"])
print("[DEBUG] loss_groups", json.dumps(t["loss_groups"], ensure_ascii=False)[:2000])
print("[DEBUG] losses", len(t["losses"]), json.dumps(t["losses"][:2], ensure_ascii=False))
print("[DEBUG] mapping", json.dumps(t["mapping"], ensure_ascii=False)[:1500])
print("[DEBUG] assumptions", json.dumps(t["assumptions"], ensure_ascii=False)[:1500])
print("[DEBUG] supports", json.dumps(t["supports"][0], ensure_ascii=False))
print("[DEBUG] members", json.dumps(t["members"][0], ensure_ascii=False), json.dumps(t["members"][25], ensure_ascii=False))
print("[DEBUG] shell", json.dumps(t["shells"][0], ensure_ascii=False))
print("[DEBUG] clash", json.dumps(M.clash_check(), ensure_ascii=False))
tr = M.tributary()
print("[DEBUG] trib", tr["area_load"], tr["load_breakdown"], tr["total_reaction"])
m = M.load_model()
print("[DEBUG] models", json.dumps(m["models"], ensure_ascii=False)[:2500])
print("[DEBUG] notes", [n["id"] for n in m["notes"]])
for tag in ("SL-1", "RM-1", "RM-2", "CW-1", "W-S"):
    e = M.element(tag)
    print("[DEBUG]", tag, e["class"], e["guid"], e["name"], json.dumps(e["geometry"], ensure_ascii=False)[:300])
print("[DEBUG] q SL-1", M.quantities("SL-1"))
print("[DEBUG] spaces", [(s["tag"], s.get("reference_area")) for s in m["spaces"]])
print("[DEBUG] rules", m["rules"])
print("[DEBUG] relations", [(r["tag"], r["class"]) for r in m["relations"]][:60])
