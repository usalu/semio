"""🧿 Run portable analytic placement contracts through normal bounded Python admission."""
import importlib,json,sys,time,unittest
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));B=Path(__file__).parent;start=time.monotonic()
m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️profiles.🧪️tests.🐍️")
r=unittest.TextTestRunner(verbosity=2).run(unittest.TestSuite(unittest.FunctionTestCase(v)for k,v in vars(m).items()if k.startswith("test_")and callable(v)))
o=B/"🗑️generated/native-r92";o.mkdir(parents=True,exist_ok=True);(o/((sys.argv[1]if len(sys.argv)>1 else "profile-axis-r95")+".json")).write_text(json.dumps({"success":r.wasSuccessful(),"tests":r.testsRun,"seconds":time.monotonic()-start,"failures":r.failures,"errors":r.errors},default=str,indent=2),encoding="utf-8");raise SystemExit(not r.wasSuccessful())
