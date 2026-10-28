import importlib
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m = model.load_model()
print("types0", json.dumps(m["types"][0], ensure_ascii=False)[:600])
print("storeys0", json.dumps(m["storeys"][0], ensure_ascii=False))
print("site", json.dumps(m["site"]["name"], ensure_ascii=False), json.dumps(m["building"]["name"], ensure_ascii=False))
for key in ("TY-PILOTI", "SL-1", "E3"):
    try:
        e = model.element(key, m)
        print("element", key, e["class"], json.dumps(e["name"], ensure_ascii=False))
    except Exception as exc:
        print("element-fail", key, repr(exc))
print("rels", [(r["class"], [o["tag"] for o in r["others"]]) for r in model.relations_of("E3", m)])
ex = model.step_excerpt("E3", m) if hasattr(model, "step_excerpt") else None
print("step", bool(ex))
if ex:
    print("\n".join(ex["lines"]))
print("spaces", [(s["id"], s["storey"], len(s["outline"])) for s in model.building_geometry(m)["spaces"]])
print("flights", [(f["tag"], f["z0"], f["z1"]) for f in model.building_geometry(m)["ramp"]["flights"]])
print("car", json.dumps(m["car"], ensure_ascii=False)[:500])
