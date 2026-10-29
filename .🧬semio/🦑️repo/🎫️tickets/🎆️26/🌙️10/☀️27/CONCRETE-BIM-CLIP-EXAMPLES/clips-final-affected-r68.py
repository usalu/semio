"""🧾 Verify bibliographic and STEP annotation repairs on one fresh source without tributary replay."""
import hashlib,importlib,json,runpy,time,unittest
from copy import deepcopy
from pathlib import Path
import numpy as np
import jsonschema
CLIP=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
TICKET=Path(__file__).parent
OUTPUT=TICKET/'🗑️generated/clips-final-affected-r68.json'
started=time.monotonic()
source=CLIP/'🧱️model/🔣️.json'
raw_before=hashlib.sha256(source.read_bytes()).hexdigest()
assert raw_before==EXPECTED_SOURCE
code_before={str(p.relative_to(CLIP)):hashlib.sha256(p.read_bytes()).hexdigest() for p in CLIP.rglob('*.py') if not any('🧪' in part for part in p.relative_to(CLIP).parts)}
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
model=api.load_model()
provider_before=hashlib.sha256(json.dumps(model,sort_keys=True,separators=(',',':')).encode()).hexdigest()
OUTPUT.write_text(json.dumps({'debug':'[DEBUG] affected repair subset','phase':'native geometry setup, no analytical translation','raw':raw_before,'provider':provider_before}))
geometry=api.building_geometry(model)
namespace=runpy.run_path(str(CLIP/'🧪️tests/🐍️.py'))
items=namespace['registry'](model)
snapshot=deepcopy(model)
suite=unittest.TestSuite()
for name in ('BimModelStructureContract','BimInformationContract','BimRelationContract','BimPartialModelContract'):
    for method in unittest.defaultTestLoader.getTestCaseNames(namespace[name]):
        subject=namespace[name](method)
        subject.api,subject.model,subject.items,subject.geometry,subject.snapshot=api,model,items,geometry,snapshot
        suite.addTest(unittest.FunctionTestCase(getattr(subject,method)))
for name,method in (('BimQuantityContract','test_window_and_opening_quantities_agree'),('BimStepExchangeContract','test_spans_point_at_the_tokens_they_name'),('BimStepExchangeContract','test_property_and_value_spans_hold_the_pset_content')):
    subject=namespace[name](method)
    subject.api,subject.model,subject.items,subject.geometry,subject.snapshot=api,model,items,geometry,snapshot
    suite.addTest(unittest.FunctionTestCase(getattr(subject,method)))
span_namespace=runpy.run_path(str(CLIP/'🧱️model/🔌️exchange/🧪️tests/🧵spans.py'))
suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(span_namespace['NativeStepSpanContract']))
chapter_namespace=runpy.run_path(str(CLIP/'🧪️tests/🧩️chapter-source/🐍️.py'),init_globals={'CAPTURED_MODEL':model})
suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(chapter_namespace['BimChapterSourceContract']))
def test_exact_bibliographic_field_repair():
    """🔗 Restoring only the eleven reference strings recovers the captured whole-provider identity."""
    text=(TICKET/'metadata-source-ids-r60.md').read_text()
    plan=json.loads(text.split('~~~json',1)[1].split('~~~',1)[0])
    restored=deepcopy(model)
    for row in plan:
        keys=row['pointer'].strip('/').split('/')
        current,target=model,restored
        for key in keys[:-1]:
            index=int(key) if isinstance(current,list) else key
            current,target=current[index],target[index]
        assert current[keys[-1]]==row['after']
        target[keys[-1]]=row['before']
    old=hashlib.sha256(json.dumps(restored,sort_keys=True,separators=(',',':')).encode()).hexdigest()
    assert old=='858fc15cd806bbaedf07fab96843ad56b34b332e22195a73264c19d4677e2628',old
def test_empty_native_shared_owner_property_annotation():
    """🧵 The real regional slab serializes empty text and annotates its two quote characters."""
    excerpt=api.step_excerpt('SL-1',model)
    index=next(i for i,line in enumerate(excerpt['lines']) if "'SharedOwnerTags'" in line and "IFCTEXT('')" in line)
    values=[excerpt['lines'][index][s['start']:s['end']] for s in excerpt['spans'] if s['line']==index and s['role']=='value']
    assert values==["''"],values
def test_real_point_boundary_is_owned_builtin_data():
    """📍 Current pure points accept NumPy inputs while exporting only a builtin float tuple."""
    module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.📏️geometry')
    point=module.P(np.asarray([1.,2.],dtype=np.float64))
    assert type(point) is tuple and all(type(value) is float for value in point)
    assert point==(1.,2.,0.)
def test_final_source_matches_real_schema():
    """🧬 Validate the final native model with the actual JSON Schema implementation."""
    schema=json.loads((CLIP/'🧱️model/🧬️schema/🔣️.schema.json').read_text())
    errors=list(jsonschema.Draft202012Validator(schema).iter_errors(model))
    assert not errors,[(list(e.path),e.message) for e in errors]
suite.addTests(unittest.FunctionTestCase(fn) for fn in (test_exact_bibliographic_field_repair,test_empty_native_shared_owner_property_annotation,test_real_point_boundary_is_owned_builtin_data,test_final_source_matches_real_schema))
phases=[]
class Result(unittest.TestResult):
    """⏳ Retain immediate phases and complete affected-subset failure details."""
    def write(self,phase,test):
        OUTPUT.write_text(json.dumps({'debug':'[DEBUG] affected repair subset','phase':phase,'test':str(test),'completed':len(phases),'planned':planned,'failures':[(str(t),m) for t,m in self.failures],'errors':[(str(t),m) for t,m in self.errors]}))
    def startTest(self,test):
        super().startTest(test)
        self.tick=time.monotonic()
        self.write('started',test)
    def stopTest(self,test):
        phases.append((str(test),time.monotonic()-self.tick))
        self.write('completed',test)
        super().stopTest(test)
planned=suite.countTestCases()
result=Result()
suite.run(result)
assert model==snapshot,'A repaired-source derivation mutated its input'
raw_after=hashlib.sha256(source.read_bytes()).hexdigest()
provider_after=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
code_after={str(p.relative_to(CLIP)):hashlib.sha256(p.read_bytes()).hexdigest() for p in CLIP.rglob('*.py') if not any('🧪' in part for part in p.relative_to(CLIP).parts)}
summary={'planned':planned,'tests':result.testsRun,'failures':[(str(t),m) for t,m in result.failures],'errors':[(str(t),m) for t,m in result.errors],'seconds':time.monotonic()-started,'source_before':raw_before,'source_after':raw_after,'provider_before':provider_before,'provider_after':provider_after,'code_stable':code_before==code_after,'changed_code':[p for p in sorted(set(code_before)|set(code_after)) if code_before.get(p)!=code_after.get(p)],'native_frames':'unverified'}
OUTPUT.write_text(json.dumps(summary))
print('FINAL_AFFECTED_SUBSET',json.dumps(summary))
