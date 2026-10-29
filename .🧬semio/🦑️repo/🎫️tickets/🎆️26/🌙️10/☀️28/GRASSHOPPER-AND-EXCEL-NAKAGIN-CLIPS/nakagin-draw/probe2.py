"""🔬 Probe text cost."""
import importlib, os, sys, time
from pathlib import Path
TICKET = Path(__file__).resolve().parent.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
fonts.body_text("warm", font_size=16, color="#ffffff")
for i in range(5):
    t0 = time.perf_counter(); m = fonts.body_text(f"2.{i}7 m", font_size=16.2, color="#ffffff"); t1 = time.perf_counter(); m.copy(); t2 = time.perf_counter()
    print("[DEBUG] new", (t1 - t0) * 1000, "copy", (t2 - t1) * 1000, len(m.submobjects), type(m).__name__)
