import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = m.load_model()
k = m.element("TY-PILOTI", model)
print("[DEBUG] type keys", sorted(k.keys()), "psets", list((k.get("psets") or {}).keys()))
b = m.element("B3", model)
print("[DEBUG] b3 psets", list(b["psets"].keys()), b.get("type"), k["tag"])
w = m.element("W-S", model)
print("[DEBUG] wall type", w.get("type"), list(m.element(w["type"], model).get("psets", {}).keys()) if w.get("type") else None, list(w["psets"].keys()))
