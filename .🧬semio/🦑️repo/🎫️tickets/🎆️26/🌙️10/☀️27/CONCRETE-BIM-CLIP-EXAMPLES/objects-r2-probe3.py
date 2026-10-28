import importlib, sys, json
from pathlib import Path
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
t = model.tributary()
print("[DEBUG]", t["area_load"], json.dumps(t["load_breakdown"], ensure_ascii=False)[:600])
b3 = next(p for p in t["pilotis"] if p["tag"] == "B3")
print("[DEBUG]", b3["area"], b3["reaction"], b3["area"] * t["area_load"])
