"""🧪 Execute only the112 uncovered canonical feature identities after native clipping repair."""
import ast,hashlib,importlib,inspect,json,marshal,runpy,textwrap,time,unittest
from pathlib import Path
clip=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');ticket=Path("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES");gen=ticket/'🗑️generated';source=clip/'🧱️model/🔣️.json'
expected='1bad321155b30fcaed4d85f6ff15e0ea5102d93e71c0b367693d19cccf72bb34';analysis_path=clip/'🧱️model/🧮️analysis/🐍️.py'
assert hashlib.sha256(source.read_bytes()).hexdigest()==expected
assert hashlib.sha256(analysis_path.read_bytes()).hexdigest()=='be55d6f11f43d2372edf86c04293d2e03e6bdc11a6b73326fdbb20a6c2ddb91e'
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def manifest():return {str(p.relative_to(clip)):digest(p) for p in sorted(clip.rglob('*')) if p.is_file() and p.suffix in {'.py','.mjs','.json'}}
def flat(suite):
 for item in suite:
  if isinstance(item,unittest.TestSuite):yield from flat(item)
  else:yield item
def canonical(test):
 fn=getattr(test,'_testFunc',None)
 if fn:
  bound=next((v for v in fn.__defaults__ or () if inspect.ismodule(v)),None)
  return bound.__name__+'.run' if bound and fn.__name__=='<lambda>' else fn.__module__+'.'+fn.__qualname__
 return test.id()
def callable_of(test):
 fn=getattr(test,'_testFunc',None)
 if fn:
  bound=next((v for v in fn.__defaults__ or () if inspect.ismodule(v)),None)
  return bound.run if bound and fn.__name__=='<lambda>' else fn
 return getattr(test,test._testMethodName)
old=json.loads((gen/'features-r72c-controlled-inputs.json').read_text());assigned=old['selected_ids'][34:]
assert len(assigned)==len(set(assigned))==112
ns=runpy.run_path(str(clip/'🧪️tests/🐍️.py'));items=list(flat(ns['load_tests'](unittest.defaultTestLoader,unittest.TestSuite(),None)));all_ids=[canonical(t) for t in items]
assert len(all_ids)==len(set(all_ids))==352
assert set(assigned)<=set(all_ids)
selected=[t for t in items if canonical(t) in set(assigned)];selected_ids=[canonical(t) for t in selected]
assert len(selected_ids)==112 and set(selected_ids)==set(assigned)
analysis=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️')
runtime={}
for test in selected:
 fn=callable_of(test);runtime[canonical(test)]={'bytecode_sha256':hashlib.sha256(marshal.dumps(fn.__code__)).hexdigest(),'method_ast_sha256':hashlib.sha256(ast.dump(ast.parse(textwrap.dedent(inspect.getsource(fn))),include_attributes=False).encode()).hexdigest()}
files_before=manifest();api=ns['model_api']();provider_before=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
inputs={'source_sha256':digest(source),'files_before':files_before,'provider_before':provider_before,'all_registered_ids':all_ids,'selected_ids':selected_ids,'original_exact_slice':assigned,'selection':'old controlled c selected_ids[34:] only; no completed prefix replay','selected_runtime':runtime,'analysis_clip_runtime':{'bytecode_sha256':hashlib.sha256(marshal.dumps(analysis._clip.__code__)).hexdigest(),'co_names':list(analysis._clip.__code__.co_names)},'registered':352,'selected':112}
(gen/'features-r79-tail-inputs.json').write_text(json.dumps(inputs),encoding='utf8')
phases=[];start=time.monotonic()
class Result(unittest.TestResult):
 """⏱️ Persist each failure and each source-bound test boundary without blocking cancellation."""
 def save_failures(self):
  (gen/'features-r79-tail-failures.json').write_text(json.dumps({'failures':[(canonical(t),m) for t,m in self.failures],'errors':[(canonical(t),m) for t,m in self.errors]}),encoding='utf8')
 def addFailure(self,test,error):
  super().addFailure(test,error);self.save_failures()
 def addError(self,test,error):
  super().addError(test,error);self.save_failures()
 def startTest(self,test):
  super().startTest(test);self.started=time.monotonic();self.source_start=digest(source)
  self.progress(test,'started')
 def progress(self,test,phase):
  (gen/'features-r79-tail-progress.json').write_text(json.dumps({'debug':'[DEBUG] actual112 postclip tail','phase':phase,'test':canonical(test),'completed':len(phases),'planned':112,'registered':352,'failures':len(self.failures),'errors':len(self.errors),'source':digest(source),'elapsed':time.monotonic()-start}),encoding='utf8')
 def stopTest(self,test):
  phases.append({'id':canonical(test),'seconds':time.monotonic()-self.started,'source_start':self.source_start,'source_end':digest(source)})
  self.save_failures();self.progress(test,'completed')
  (gen/'features-r79-tail-phases.json').write_text(json.dumps(phases),encoding='utf8')
  if (gen/'features-r79-tail-cancel').exists():self.stop()
  super().stopTest(test)
result=Result();unittest.TestSuite(selected).run(result)
files_after=manifest();provider_after=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
record={**inputs,'run':result.testsRun,'seconds':time.monotonic()-start,'failures':[(canonical(t),m) for t,m in result.failures],'errors':[(canonical(t),m) for t,m in result.errors],'phases':phases,'source_after':digest(source),'provider_after':provider_after,'files_after':files_after,'changed_files':[p for p in sorted(set(files_before)|set(files_after)) if files_before.get(p)!=files_after.get(p)],'cancelled':result.shouldStop}
(gen/'features-r79-tail-result.json').write_text(json.dumps(record),encoding='utf8');print({'run':result.testsRun,'selected':112,'failures':len(result.failures),'errors':len(result.errors),'seconds':record['seconds'],'source_equal':inputs['source_sha256']==record['source_after'],'provider_equal':provider_before==provider_after,'changed_files':record['changed_files']})
