"""🟢 Verify published physical fixtures, bilingual provenance and current source contracts."""
import hashlib,importlib,inspect,json,runpy,time,unittest
from pathlib import Path
root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');ticket=Path(__file__).parent;generated=ticket/'🗑️generated';source=root/'🧱️model/🔣️.json'
raw_before=hashlib.sha256(source.read_bytes()).hexdigest();assert raw_before=='1bad321155b30fcaed4d85f6ff15e0ea5102d93e71c0b367693d19cccf72bb34'
ns=runpy.run_path(str(root/'🧪️tests/🐍️.py'));api=ns['model_api']();provider_before=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
details=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛁️details');remodel=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️remodel')
selected=[unittest.FunctionTestCase(fn) for name,fn in inspect.getmembers(details,inspect.isfunction) if name.startswith('test_')]
selected+=[unittest.FunctionTestCase(remodel.test_guest_door_full_body_width_and_public_approach)]
for name in ('test_every_quantity_is_sourced_or_assumed_with_a_unit','test_every_element_is_complete_and_references_exist','test_every_localized_text_has_both_languages'):selected.append(ns['BimModelStructureContract'](name))
def schema_contract():
    import jsonschema
    schema=json.loads((root/'🧱️model/🧬️schema/🔣️.schema.json').read_text());jsonschema.Draft202012Validator(schema).validate(api.load_model())
selected.append(unittest.FunctionTestCase(schema_contract));assert len(selected)==11
phases=[]
class Result(unittest.TestResult):
    """⏱️ Persist each published-source verification boundary."""
    def save(self,phase,test):
        record={'debug':'[DEBUG] R77 published fixture proof','phase':phase,'test':test.id(),'completed':len(phases),'planned':len(selected),'failures':[(t.id(),m) for t,m in self.failures],'errors':[(t.id(),m) for t,m in self.errors],'phases':phases};(generated/'fixture-repair-r77-green-progress.json').write_text(json.dumps(record,ensure_ascii=False))
    def startTest(self,test):super().startTest(test);self.started=time.monotonic();self.save('started',test)
    def stopTest(self,test):phases.append({'id':test.id(),'seconds':time.monotonic()-self.started});self.save('completed',test);super().stopTest(test)
result=Result();started=time.monotonic();unittest.TestSuite(selected).run(result)
record={'run':result.testsRun,'failures':[(t.id(),m) for t,m in result.failures],'errors':[(t.id(),m) for t,m in result.errors],'seconds':time.monotonic()-started,'source_before':raw_before,'source_after':hashlib.sha256(source.read_bytes()).hexdigest(),'provider_before':provider_before,'provider_after':hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest(),'phases':phases}
(generated/'fixture-repair-r77-green.json').write_text(json.dumps(record,ensure_ascii=False));print(record)
