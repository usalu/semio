"""🪟 Run actual source salon bay contract."""
import sys,importlib,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪟️salon");t=time.monotonic();m.test_original_bay_dimensions_outboard_plane_fixed_guard_and_fold_contact();print("[DEBUG] salon bay PASS "+str(time.monotonic()-t),flush=True)
