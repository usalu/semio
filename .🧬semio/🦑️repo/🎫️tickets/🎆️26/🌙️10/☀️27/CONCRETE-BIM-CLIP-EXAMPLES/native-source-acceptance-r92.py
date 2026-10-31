"""🧪 Check current source acceptance against independent geometry without a final-freeze claim."""
import hashlib,importlib,json,sys,time,unittest
from pathlib import Path
ROOT=Path.cwd();sys.path.insert(0,str(ROOT));TICKET=Path(__file__).parent;OUT=TICKET/'🗑️generated/native-r92';OUT.mkdir(parents=True,exist_ok=True)
PREFIX='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
NAMES=['🏛️architecture','🏠ground'];SELECTED=[]
for name in NAMES:
    module=importlib.import_module(PREFIX+'🧱️model.🧪️tests.'+name)
    SELECTED.extend(getattr(module,key) for key in sorted(dir(module)) if key.startswith('test_'))
for name,function in [('🏛️remodel','test_chauffeur_programme_partitions_the_real_diagonal_studio'),('🚬️chimneys','test_chimney_and_real_hall_route_have_no_occupied_native_clash'),('🚶️hall','test_hall_body_route_retains_all_native_parts_and_actual_laundry_leaf')]:SELECTED.append(getattr(importlib.import_module(PREFIX+'🧱️model.🧪️tests.'+name),function))
BASE=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip';FILES=[BASE/'🧱️model/🔣️.json',BASE/'🧱️model/🐍️.py',BASE/'🧱️model/🧮️analysis/🐍️.py',Path(__file__)]+[BASE/'🧱️model/🧪️tests'/file for file in ['🏛️architecture.py','🏛️architecture.json','🏠ground.py','🏠ground.json','🏠ground.schema.json','🏛️remodel.py','🏛️remodel.json','🚬️chimneys.py','🚬️chimneys.json','🚗️parking.py','🚶️hall.py','🚶️hall.json','🚶️hall.schema.json']]
def hashes():
    """🔐 Capture the exact current source and acceptance producers."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in FILES}
class Result(unittest.TextTestResult):
    def startTest(self,test):
        if (TICKET/'native-r92-cancel').exists():raise KeyboardInterrupt('Native validation cancelled')
        row={'state':'running','test':test.id(),'time':time.time(),'inputs':hashes()}
        (OUT/'progress-source-acceptance.json').write_text(json.dumps(row,ensure_ascii=True),encoding='utf8');print('[DEBUG] Source acceptance '+test.id(),flush=True)
        super().startTest(test)
before=hashes();start=time.time();suite=unittest.TestSuite(unittest.FunctionTestCase(f) for f in SELECTED)
result=unittest.TextTestRunner(verbosity=2,resultclass=Result).run(suite);after=hashes()
report={'diagnostic':True,'tests_successful':result.wasSuccessful(),'stable':before==after,'run':result.testsRun,'seconds':time.time()-start,'selected':[f.__module__+'.'+f.__name__ for f in SELECTED],'inputs_before':before,'inputs_after':after,'failures':[(t.id(),e) for t,e in result.failures],'errors':[(t.id(),e) for t,e in result.errors]}
(OUT/'result-source-acceptance.json').write_text(json.dumps(report,indent=2),encoding='utf8')
raise SystemExit(0 if result.wasSuccessful() else 1)
