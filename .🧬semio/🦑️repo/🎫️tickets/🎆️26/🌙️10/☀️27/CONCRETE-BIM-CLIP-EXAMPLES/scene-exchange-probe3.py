import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = m.load_model()
tr = m.translate_structure(model)
print("[DEBUG] groups", [(g["id"], g["count"]) for g in tr["loss_groups"]])
print("[DEBUG] losses props", sorted({(l["source_tag"][:2], l["property"], l.get("group")) for l in tr["losses"]}))
print("[DEBUG] wall layers", [(l["name"]["en"]) for l in m.element("W-S", model)["material"]["layers"]])
