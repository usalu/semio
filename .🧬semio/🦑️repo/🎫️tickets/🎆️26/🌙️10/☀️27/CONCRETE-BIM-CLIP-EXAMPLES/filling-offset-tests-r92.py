"""🪟 Run portable occurrence offset and frame head contract."""
import sys,importlib,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪟️offset");t=time.monotonic();m.test_occurrence_normal_offset_and_head_recess_preserve_rough_aperture();print("[DEBUG] filling offset PASS "+str(time.monotonic()-t),flush=True)
