import cProfile, importlib, os, pstats, sys, time
from pathlib import Path
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/axon-en")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
axon = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️")
class S(visuals.ClipScene):
    pass
s = S()
t = time.time(); s.villa_axon((0, 0, 0), 0.17); print("[DEBUG] first", time.time() - t)
t = time.time(); s.villa_axon((0, 0, 0), 0.17); print("[DEBUG] second", time.time() - t)
t = time.time(); s.villa_axon((0, 0, 0), 0.17, explode=0.5); print("[DEBUG] explode", time.time() - t)
profiler = cProfile.Profile(); profiler.enable()
s.villa_axon((0, 0, 0), 0.17, highlight=("B3",))
profiler.disable()
pstats.Stats(profiler).sort_stats("cumulative").print_stats(18)
