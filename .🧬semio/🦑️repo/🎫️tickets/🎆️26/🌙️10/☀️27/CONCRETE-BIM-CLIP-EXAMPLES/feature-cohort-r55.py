"""🧪 Execute the actual registered feature cohort with bounded observable progress."""
import hashlib,importlib,json,runpy,time,unittest
from pathlib import Path
root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
ticket=Path(__file__).parent;generated=ticket/'🗑️generated'
source=root/'🧱️model/🔣️.json'
raw_before=hashlib.sha256(source.read_bytes()).hexdigest()
if globals().get('expected_source_hash'):assert raw_before==expected_source_hash
def code_hash():
    digest=hashlib.sha256()
    for path in sorted(p for p in root.rglob('*') if p.is_file() and p.suffix in {'.py','.mjs','.json'}):
        digest.update(str(path.relative_to(root)).encode());digest.update(path.read_bytes())
    return digest.hexdigest()
def code_manifest():
    return {str(path.relative_to(root)):hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(p for p in root.rglob('*') if p.is_file() and p.suffix in {'.py','.mjs','.json'})}
code_before=code_hash();files_before=code_manifest()
ns=runpy.run_path(str(root/'🧪️tests/🐍️.py'))
api=ns['model_api']();provider_before=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
suite=ns['load_tests'](unittest.defaultTestLoader,unittest.TestSuite(),None)
planned=suite.countTestCases();phases=[]
progress=generated/'features-r55-progress.json';failure_path=generated/'features-r55-failures.json'
class Result(unittest.TestResult):
    """⏱️ Preserve exact phases and failures while allowing cancellation between tests."""
    def startTest(self,test):
        super().startTest(test);self.started=time.monotonic()
        progress.write_text(json.dumps({'debug':'[DEBUG] registered feature progress','phase':'started','test':str(test),'completed':len(phases),'planned':planned,'failures':len(self.failures),'errors':len(self.errors)}),encoding='utf8')
    def stopTest(self,test):
        phases.append({'test':str(test),'seconds':time.monotonic()-self.started})
        progress.write_text(json.dumps({'debug':'[DEBUG] registered feature progress','phase':'completed','test':str(test),'completed':len(phases),'planned':planned,'failures':len(self.failures),'errors':len(self.errors),'seconds':phases[-1]['seconds']}),encoding='utf8')
        failure_path.write_text(json.dumps({'failures':[(str(t),m) for t,m in self.failures],'errors':[(str(t),m) for t,m in self.errors]}),encoding='utf8')
        if (generated/'features-r55-cancel').exists():self.stop()
        super().stopTest(test)
result=Result();started=time.monotonic();suite.run(result)
raw_after=hashlib.sha256(source.read_bytes()).hexdigest();code_after=code_hash();files_after=code_manifest()
provider_after=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
record={'planned':planned,'run':result.testsRun,'seconds':time.monotonic()-started,'failures':[(str(t),m) for t,m in result.failures],'errors':[(str(t),m) for t,m in result.errors],'raw_before':raw_before,'raw_after':raw_after,'code_before':code_before,'code_after':code_after,'provider_before':provider_before,'provider_after':provider_after,'phases':phases,'cancelled':result.shouldStop,'files_before':files_before,'files_after':files_after,'changed_files':[path for path in sorted(set(files_before)|set(files_after)) if files_before.get(path)!=files_after.get(path)]}
(generated/'features-r55-result.json').write_text(json.dumps(record,ensure_ascii=False),encoding='utf8')
print({k:v for k,v in record.items() if k not in ('phases','failures','errors')});print({'failures':len(result.failures),'errors':len(result.errors)})
