"""🛝️ Run permanent occupied ramp-floor interface contract."""
import importlib,sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️interfaces")
m.test_floor_and_ramp_interfaces_have_no_duplicate_material();print("[DEBUG] Ramp floor interface contract PASS",flush=True)
