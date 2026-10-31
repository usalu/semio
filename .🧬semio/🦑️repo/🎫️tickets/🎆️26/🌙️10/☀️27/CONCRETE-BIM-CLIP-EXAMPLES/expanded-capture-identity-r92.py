"""🧱 Execute exact inclined wall-interface acceptance with retained outcomes."""
import importlib,json,sys,time,unittest
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
stage=sys.argv[1];start=time.monotonic();module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧾️capture');result=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromModule(module));out={'stage':stage,'run':result.testsRun,'pass':result.wasSuccessful(),'seconds':time.monotonic()-start,'errors':[(str(a),b)for a,b in result.errors+result.failures]};(Path(__file__).parent/'🗑️generated'/('expanded-capture-identity-'+stage+'-r92.json')).write_text(json.dumps(out,indent=2),encoding='utf8');raise SystemExit(not out['pass'])
