"""🧪 Run focused native projection integrations with source provenance and cancellation."""
import hashlib,importlib,json,sys,time,unittest
from pathlib import Path
ROOT=Path.cwd();TICKET=Path(__file__).parent
sys.path.insert(0,str(ROOT))
OUT=TICKET/'🗑️generated/native-r92';OUT.mkdir(parents=True,exist_ok=True)
PREFIX='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
SELECTED=[('🧱️model.🧪️tests.📐️representations','test_sections_and_elevations_use_actual_xz_solids'),('🧱️model.🧪️tests.📐️representations','test_sections_preserve_real_cavities'),('🧱️model.🧪️tests.🚬️chimneys','test_actual_production_catalogue_and_storey_projections_keep_spanning_chimney'),('🧱️model.🧪️tests.🚬️prisms','test_production_roof_profile_and_above_cut_wire_keep_material_scope')]
FILES=[ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip'/p for p in ['🧱️model/🔣️.json','🧱️model/🐍️.py','🧱️model/🧮️analysis/🐍️.py','🧱️model/🧪️tests/📐️representations.py','🧱️model/🧪️tests/📐️representations.json','🧱️model/🧪️tests/🚬️chimneys.py','🧱️model/🧪️tests/🚬️chimneys.json','🧱️model/🧪️tests/🚬️prisms.py','🧱️model/🧪️tests/🚬️prisms.json','🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_frustums.py','🎨️visuals/🏛️savoye/🧱️core/📐️geometry.py','🎨️visuals/🏛️savoye/📋️plan/🐍️.py','🎨️visuals/🏛️savoye/✂️section/🐍️.py','🎨️visuals/🏛️savoye/🧊️axon/🧩️primitives.py']]+[Path(__file__)]
def hashes():
    """🔐 Bind diagnostic evidence to the exact source and producer files."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in FILES}
class Result(unittest.TextTestResult):
    def startTest(self,test):
        if (TICKET/'native-r92-cancel').exists():raise KeyboardInterrupt('Native validation cancelled')
        row={'state':'running','test':test.id(),'time':time.time(),'inputs':hashes()}
        (OUT/'progress-focused-projections.json').write_text(json.dumps(row,ensure_ascii=True),encoding='utf8')
        print('[DEBUG] Native projection '+test.id(),flush=True)
        super().startTest(test)
before=hashes();start=time.time();suite=unittest.TestSuite()
for name,function in SELECTED:suite.addTest(unittest.FunctionTestCase(getattr(importlib.import_module(PREFIX+name),function)))
result=unittest.TextTestRunner(verbosity=2,resultclass=Result).run(suite)
after=hashes();report={'diagnostic':True,'tests_successful':result.wasSuccessful(),'stable':before==after,'run':result.testsRun,'seconds':time.time()-start,'inputs_before':before,'inputs_after':after,'failures':[(t.id(),e) for t,e in result.failures],'errors':[(t.id(),e) for t,e in result.errors]}
(OUT/'result-focused-projections.json').write_text(json.dumps(report,indent=2),encoding='utf8')
raise SystemExit(0 if result.wasSuccessful() else 1)
