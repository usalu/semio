"""🛝️ Run native floor and ramp support regression after physical ownership cuts."""
import importlib,sys,time,unittest,traceback,runpy
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
errors=[]
for module,names in [("🛝️arrivals",None),("🪵️floors",["test_original_ground_wearing_materials_support_layers_and_exposed_cement"])]:
 m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests."+module)
 for name in names or [n for n in dir(m) if n.startswith("test_")]:
  start=time.monotonic()
  try:getattr(m,name)();print("[DEBUG] "+name+" PASS "+str(time.monotonic()-start),flush=True)
  except Exception:traceback.print_exc();errors.append(name)

runpy.run_path(str(Path(__file__).with_name("ramp-structural-joints-r92.py")),run_name="__main__")
suite=unittest.defaultTestLoader.loadTestsFromName("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️ramps");result=unittest.TextTestRunner(verbosity=2).run(suite);sys.exit(bool(errors) or not result.wasSuccessful())
