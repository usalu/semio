"""🧪 Run actual published wearing/approach contracts without candidate injection."""
import hashlib,importlib,json,sys,time,unittest
from pathlib import Path
B=Path(__file__).parent;sys.path.insert(0,str(Path.cwd()));started=time.perf_counter()
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.'
modules=[importlib.import_module(prefix+n) for n in ['🌿️gravel','🪵️coal-bay']]
suite=unittest.TestSuite(unittest.FunctionTestCase(getattr(m,n)) for m in modules for n in sorted(dir(m)) if n.startswith('test_') and callable(getattr(m,n)))
floors=importlib.import_module(prefix+'🪵️floors');suite.addTest(unittest.FunctionTestCase(floors.test_original_ground_wearing_materials_support_layers_and_exposed_cement))
A=floors.a;before=hashlib.sha256(A._source().encode('utf-8')).hexdigest();r=unittest.TextTestRunner(verbosity=2).run(suite);after=hashlib.sha256(A._source().encode('utf-8')).hexdigest()
report={'success':r.wasSuccessful() and before==after,'source_before':before,'source_after':after,'tests':r.testsRun,'failures':[(str(t),s) for t,s in r.failures],'errors':[(str(t),s) for t,s in r.errors],'seconds':time.perf_counter()-started,'scope':'actual published source; no candidate monkeypatch'}
(B/'🗑️generated/native-r92/published-floor-grade-r94.json').write_text(json.dumps(report,ensure_ascii=True,indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps(report),flush=True);sys.exit(not report['success'])
