"""🏠 Run registered native roof flooring contracts."""
import sys,importlib,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏠roof-floors")
for name in [n for n in dir(m) if n.startswith("test_") and (len(sys.argv)<2 or sys.argv[1] in n)]:
 t=time.monotonic();getattr(m,name)();print("[DEBUG] "+name+" PASS "+str(time.monotonic()-t),flush=True)
