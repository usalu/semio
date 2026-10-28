import importlib
import os
import sys
import tempfile
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_CACHE_DIR"] = tempfile.mkdtemp()
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
for word in ("kWh/(m²·a)", "(m²·a)", "m²"):
    proto = room._proto("0" + word)
    print(repr(word), len(proto.submobjects), [len(g.points) for g in proto.submobjects])
