"""🏗️ Execute native construction export acceptance with retained outcomes."""
import importlib,unittest,json,time,sys,hashlib
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
stage=sys.argv[1];module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🧪️tests.📏️rounding');suite=unittest.defaultTestLoader.loadTestsFromModule(module);start=time.monotonic();result=unittest.TextTestRunner(verbosity=2).run(suite);out={'stage':stage,'run':result.testsRun,'pass':result.wasSuccessful(),'seconds':time.monotonic()-start,'errors':[(str(a),b)for a,b in result.errors+result.failures]};(Path(__file__).parent/'🗑️generated'/('rounding-'+stage+'-r92.json')).write_text(json.dumps(out,indent=2),encoding='utf8');raise SystemExit(not out['pass'])
