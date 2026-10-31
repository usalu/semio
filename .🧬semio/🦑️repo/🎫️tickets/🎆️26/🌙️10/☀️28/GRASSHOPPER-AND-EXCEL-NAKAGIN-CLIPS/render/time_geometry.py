"""⏱️ Time the shared BIM building_geometry once, in process, to see what a BIM scene pays before its first frame."""
import cProfile, importlib, pstats, sys, time
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
if len(sys.argv) > 1:
    importlib.import_module(sys.argv[1]).install()
started = time.time()
profile = cProfile.Profile()
profile.enable()
value = model.building_geometry()
profile.disable()
print("[DEBUG] building_geometry seconds", round(time.time() - started), "walls", len(value.get("walls", ())), flush=True)
pstats.Stats(profile).sort_stats("cumulative").print_stats(25)
