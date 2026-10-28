"""🔎 Probe: dump the real building geometry the orthographic drawings consume."""
import importlib
import json
import os
import sys
import time
from pathlib import Path

T = Path(__file__).resolve().parent
ROOT = next(p for p in T.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(T / "🗑️generated/fonts/probe")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
g = model.building_geometry()
for key, value in g.items():
    text = json.dumps(value, ensure_ascii=False)
    print("[DEBUG]", key, text[:900])
m = model.load_model()
print("[DEBUG] car", json.dumps(m["car"], ensure_ascii=False)[:600])
print("[DEBUG] keys", list(m))
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
started = time.time()
for k in range(40):
    fonts.body_text(f"{k * 0.37:.2f} m", font_size=16.2, color="#ffffff")
print("[DEBUG] 40 texts", time.time() - started)
