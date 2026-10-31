"""⚡ Run native qualified appliance contract."""
import sys,importlib,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.⚙️apparatus");t=time.monotonic();getattr(m,sys.argv[1] if len(sys.argv)>1 else "test_native_unknown_apparatus_preserves_owned_volume_and_required_user_type")();print("[DEBUG] native appliance PASS "+str(time.monotonic()-t),flush=True)
