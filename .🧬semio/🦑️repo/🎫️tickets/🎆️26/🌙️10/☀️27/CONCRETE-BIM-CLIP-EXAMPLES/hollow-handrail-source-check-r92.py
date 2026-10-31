"""🧪 Verify complete handrail candidates through the actual source geometry API before publication."""
import importlib,json,sys,unittest,hashlib,time
from pathlib import Path
from unittest.mock import patch
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
ticket=Path(__file__).parent;api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');tests=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪈️tubes');m=api.load_model();candidate=json.loads((ticket/'🗑️generated'/'hollow-handrail-candidates-r92.json').read_text(encoding='utf8'));start=time.monotonic()
for row in candidate['products']:
    obj=api._find(m,row['tag']);obj['geometry']['parts']=row['parts'];obj['geometry']['tube_construction']=row['construction']
with patch.object(api,'load_model',return_value=m):result=unittest.TextTestRunner(verbosity=2).run(unittest.TestSuite([tests.HollowTubes('test_source_handrails_keep_positive_metal_and_exposed_air_cores')]))
out={'pass':result.wasSuccessful(),'tests':result.testsRun,'seconds':time.monotonic()-start,'errors':[(str(a),b)for a,b in result.errors+result.failures]};(ticket/'🗑️generated'/'hollow-handrail-source-check-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');raise SystemExit(not out['pass'])
