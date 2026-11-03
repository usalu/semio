"""🚘 r3-clipsc: time the cheap car queries of the BIM model without the building geometry."""
import importlib
import sys
import time
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
for name in ("clearance", "car_path"):
    started = time.time()
    value = getattr(model, name)(None)
    print("[DEBUG]", name, round(time.time() - started, 2), "s", str(value)[:300])
