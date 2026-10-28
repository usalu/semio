"""🔎 Print the orientation study fields the extrema scene reads."""
import importlib
import sys
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
sys.stdout.reconfigure(encoding="utf-8")
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
study = model.orientation_study()
for key, value in study.items():
    if key not in {"samples", "savings"}:
        print("[DEBUG]", key, "=", value if len(str(value)) < 700 else str(value)[:700])
print("[DEBUG] samples", study["samples"][:3], len(study["samples"]), study["savings"][:2])
print("[DEBUG] room", model.load_study()["room"], list(model.load_study().keys()))
