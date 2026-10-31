"""🛝️ Run registered native ramp arrival contracts."""
import sys,importlib,json,time,traceback
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️arrivals")
for n in [n for n in dir(m) if n.startswith("test_")]:
 t=time.monotonic()
 try:getattr(m,n)();print("[DEBUG] "+json.dumps({"test":n,"passed":True,"seconds":time.monotonic()-t}),flush=True)
 except Exception:traceback.print_exc();sys.exit(1)
