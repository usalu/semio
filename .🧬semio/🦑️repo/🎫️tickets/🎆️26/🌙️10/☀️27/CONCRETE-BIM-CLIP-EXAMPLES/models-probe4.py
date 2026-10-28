import importlib, json, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
t = M.translate_structure()
print(json.dumps(t["members"][25:27], ensure_ascii=False))
print(json.dumps(t["mapping"], ensure_ascii=False)[:800])
print([n for n in t["nodes"] if n["role"] != "slab"][:2])
print(json.dumps(t["supports"][0], ensure_ascii=False))
print(json.dumps(t["point_loads"][:1]))
print(t["loads"].keys())
print([a["id"] for a in t["assumptions"]])
for a in t["assumptions"][4:]:
    print(a)
print(M.load_model()["rules"] if "rules" in M.load_model() else "")
