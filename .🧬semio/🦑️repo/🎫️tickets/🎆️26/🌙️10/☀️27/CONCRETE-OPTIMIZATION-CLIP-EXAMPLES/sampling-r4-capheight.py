import importlib
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(Path(__file__).resolve().parent / "generated-placeholder")
room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
for size in (13, 14, 15, 17, 18, 19, 20):
    for word in ("H", "Daylit", "100 %"):
        label = room.text(word, size=size)
        print("[DEBUG]", size, word, round(label.height * 90, 1), "px high", round(label.width * 90, 1), "px wide")
