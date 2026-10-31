"""🔬 Sample the stack of building_geometry every N seconds: dump_geometry.py <seconds> <dumpfile>"""
import faulthandler, importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
faulthandler.dump_traceback_later(float(sys.argv[1]), repeat=True, file=open(sys.argv[2], "wb", buffering=0))
importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️").building_geometry()
