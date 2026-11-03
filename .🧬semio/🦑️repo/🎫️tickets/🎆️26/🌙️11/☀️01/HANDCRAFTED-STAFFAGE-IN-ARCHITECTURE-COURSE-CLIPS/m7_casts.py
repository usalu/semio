"""🧍 M7: build the course-clip casts of the non-energy series and print their sizes."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
IDS = ["bim-consistency-reviewer", "bim-consistency-decider", "excel-program-occupant", "optimization-room-occupant", "optimization-room-radiator",
       "optimization-extrema-radiator", "optimization-experiments-occupant", "optimization-experiments-window"]
print("[DEBUG] ensure", art.ensure_casts(IDS) if hasattr(art, "ensure_casts") else "no ensure")
for cast in IDS:
    g = art.staffage(cast, unit=1.0)
    print("[DEBUG]", cast, "height_m", round(art.staffage_height(cast), 3), "w,h units", round(g.width, 3), round(g.height, 3), "centre", g.get_center().round(2))
