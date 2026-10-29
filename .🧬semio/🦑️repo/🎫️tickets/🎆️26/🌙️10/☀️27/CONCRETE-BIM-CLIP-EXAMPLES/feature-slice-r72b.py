"""🧪 Resume the actual registered feature hook by explicit unchanged passed test identities."""
import ast,hashlib,importlib,inspect,json,runpy,time,unittest
from pathlib import Path

root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
ticket=Path(__file__).parent;generated=ticket/'🗑️generated';source=root/'🧱️model/🔣️.json'
prefix=json.loads((ticket/'feature-prefix-r62.json').read_text());passed=set(prefix['passed_canonical_ids'])
raw_before=hashlib.sha256(source.read_bytes()).hexdigest()
transition=json.loads((generated/'final-transition-r68.json').read_text())
verified=json.loads((generated/'transition-verified-r68.json').read_text())
assert raw_before==globals().get('expected_source_hash')==transition['source_after']==verified['source']
assert transition['source_before']==prefix['source_sha256']
assert transition['nonbibliographic_fingerprint_before']==transition['nonbibliographic_fingerprint_after']==verified['normalized_source_sha256']
assert verified['current_independently_verified'] and transition['full_references_values_preserved']
assert len(set(transition['changed_pointers']))==transition['bibliographic_fields']==11
assert all('/psets/' in pointer and pointer.endswith('/source') for pointer in transition['changed_pointers'])
assert hashlib.sha256((root/'🧱️model/🔌️exchange/🐍️.py').read_bytes()).hexdigest()==transition['producer_after']

def manifest():
    return {str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(root.rglob('*')) if p.is_file() and p.suffix in {'.py','.mjs','.json'}}

def canonical(test):
    fn=getattr(test,'_testFunc',None)
    if fn:
        bound=next((value for value in fn.__defaults__ or () if inspect.ismodule(value)),None)
        return bound.__name__+'.run' if bound and fn.__name__=='<lambda>' else fn.__module__+'.'+fn.__qualname__
    return test.id()

def flatten(suite):
    for item in suite:
        if isinstance(item,unittest.TestSuite):yield from flatten(item)
        else:yield item

def dependency_ast(relative):
    result={}
    for node in ast.parse((root/relative).read_text()).body:
        if isinstance(node,ast.FunctionDef):result[node.name]=hashlib.sha256(ast.dump(node,include_attributes=False).encode()).hexdigest()
        elif isinstance(node,ast.ClassDef):
            for method in node.body:
                if isinstance(method,ast.FunctionDef) and method.name!='test_physical_oracle_matches_native_height_intersections':result[node.name+'.'+method.name]=hashlib.sha256(ast.dump(method,include_attributes=False).encode()).hexdigest()
    return result

for relative,before in prefix['dependency_ast'].items():
    after=dependency_ast(relative)
    assert all(after.get(key)==value for key,value in before.items()),relative
assert hashlib.sha256((root/'🧱️model/🧪️tests/🚗️parking.json').read_bytes()).hexdigest()==prefix['parking_fixture_sha256']
files_before=manifest()
ns=runpy.run_path(str(root/'🧪️tests/🐍️.py'));api=ns['model_api']()
provider_before=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
all_tests=list(flatten(ns['load_tests'](unittest.defaultTestLoader,unittest.TestSuite(),None)))
all_ids=[canonical(test) for test in all_tests]
assert len(all_ids)==len(set(all_ids))==351
assert passed<=set(all_ids)
assignment=json.loads((generated/'feature-slice-ready-r72.json').read_text())
assigned=set(assignment['my_ids'])
assert len(assigned)==147
assert hashlib.sha256((root/'🧱️model/🧪️tests/⚡️partitions.py').read_bytes()).hexdigest()==assignment['partition_sha256']
selected=[test for test in all_tests if canonical(test) in assigned]
selected_ids=[canonical(test) for test in selected]
assert len(selected_ids)==147 and set(selected_ids)==assigned and len(passed)==10
completed48=json.loads((ticket/'feature-prefix-r71.json').read_text())
covered=set(completed48['original_ten_passed_ids'])|set(completed48['new_48_passed_ids'])
peer=set(assignment['r11_ids'])
assert len(covered)==58 and len(peer)==146
assert not (assigned&peer or assigned&covered or peer&covered)
assert assigned|peer|covered==set(all_ids)
inputs={'source_sha256':raw_before,'files_before':files_before,'provider_before':provider_before,'all_registered_ids':all_ids,'passed_prefix_ids':sorted(passed),'selected_ids':selected_ids,'prefix_dependencies_unchanged':True,'completed_prefix_ids':sorted(covered),'peer_selected_ids':assignment['r11_ids'],'assignment':'r72b exact147 canonical IDs; root-r72a owns146','source_transition':transition,'normalized_transition_verification':verified,'prefix_numeric_source_sha256':prefix['source_sha256'],'composition_scope':'ten c53 numerical prefix passes under verified bibliographic-only invariance, final snapshot remaining341'}
(generated/'features-r72b-inputs.json').write_text(json.dumps(inputs,ensure_ascii=False),encoding='utf8')
phases=[];progress=generated/'features-r72b-progress.json';failures_path=generated/'features-r72b-failures.json'

class Result(unittest.TestResult):
    """⏱️ Observe every selected canonical test and preserve failures immediately."""
    def save_failures(self):
        failures_path.write_text(json.dumps({'failures':[(canonical(t),m) for t,m in self.failures],'errors':[(canonical(t),m) for t,m in self.errors]},ensure_ascii=False),encoding='utf8')
    def addFailure(self,test,error):
        super().addFailure(test,error);self.save_failures()
    def addError(self,test,error):
        super().addError(test,error);self.save_failures()
    def startTest(self,test):
        super().startTest(test);self.started=time.monotonic()
        progress.write_text(json.dumps({'debug':'[DEBUG] explicit-ID feature tail','phase':'started','test':canonical(test),'completed':len(phases),'planned':len(selected),'prefix_passed':len(passed),'registered':len(all_ids),'failures':len(self.failures),'errors':len(self.errors)},ensure_ascii=False),encoding='utf8')
    def stopTest(self,test):
        phases.append({'id':canonical(test),'seconds':time.monotonic()-self.started})
        progress.write_text(json.dumps({'debug':'[DEBUG] explicit-ID feature tail','phase':'completed','test':canonical(test),'completed':len(phases),'planned':len(selected),'prefix_passed':len(passed),'registered':len(all_ids),'failures':len(self.failures),'errors':len(self.errors),'seconds':phases[-1]['seconds']},ensure_ascii=False),encoding='utf8')
        self.save_failures()
        if (generated/'features-r72b-cancel').exists():self.stop()
        super().stopTest(test)

result=Result();started=time.monotonic();unittest.TestSuite(selected).run(result)
raw_after=hashlib.sha256(source.read_bytes()).hexdigest();files_after=manifest()
provider_after=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
record={**inputs,'run':result.testsRun,'seconds':time.monotonic()-started,'failures':[(canonical(t),m) for t,m in result.failures],'errors':[(canonical(t),m) for t,m in result.errors],'raw_after':raw_after,'files_after':files_after,'changed_files':[p for p in sorted(set(files_before)|set(files_after)) if files_before.get(p)!=files_after.get(p)],'provider_after':provider_after,'phases':phases,'cancelled':result.shouldStop}
(generated/'features-r72b-result.json').write_text(json.dumps(record,ensure_ascii=False),encoding='utf8')
print({'registered':len(all_ids),'passed_prefix':len(passed),'selected':len(selected),'run':result.testsRun,'failures':len(result.failures),'errors':len(result.errors),'seconds':record['seconds'],'cancelled':result.shouldStop,'source_equal':raw_before==raw_after,'code_equal':files_before==files_after,'provider_equal':provider_before==provider_after})
