"""🪵 Run physical floor versus exclusive raised beam contract."""
import sys,importlib,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪵️raised-beams");t=time.monotonic();m.test_real_floor_layers_do_not_occupy_exclusive_raised_beam_material();print("[DEBUG] raisedbeam floors PASS "+str(time.monotonic()-t),flush=True)
