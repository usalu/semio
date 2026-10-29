import importlib,json,time,hashlib,marshal,unittest
from pathlib import Path
base=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');ticket=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES');out=ticket/'🗑️generated'
main=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️');api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
ids=json.loads((ticket/'feature-prefix-r62.json').read_text())['passed_canonical_ids'][:6]
def flatten(s):
 for x in s:
  if isinstance(x,unittest.TestSuite):yield from flatten(x)
  else:yield x
def identity(x):
 if isinstance(x,unittest.FunctionTestCase):
  f=x._testFunc
  if f.__name__=='<lambda>' and f.__defaults__:return f.__defaults__[0].__name__+'.run'
  return f.__module__+'.'+f.__name__
 return x.id()
loader=unittest.defaultTestLoader;tests=list(flatten(main.load_tests(loader,unittest.TestSuite(),None)));allids=[identity(t) for t in tests];assert len(allids)==len(set(allids))==355;assert not loader.errors
selected=[]
for key in ids:
 matches=[t for t in tests if identity(t)==key];assert len(matches)==1,(key,len(matches));selected+=matches
def hashes():
 return {str(p.relative_to(base)):hashlib.sha256(p.read_bytes()).hexdigest() for p in base.rglob('*.py') if not any(part.startswith('🧪') or part=='__pycache__' for part in p.relative_to(base).parts)}
def runtime():return {identity(t):hashlib.sha256(marshal.dumps(getattr(t,t._testMethodName).__func__.__code__)).hexdigest() for t in selected}
def source():return hashlib.sha256((base/'🧱️model/🔣️.json').read_bytes()).hexdigest()
def provider():return hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
before={'source':source(),'provider_canonical_model':provider(),'production':hashes(),'runtime_code':runtime()};started=time.monotonic();completed=[]
class Result(unittest.TestResult):
 def startTest(self,test):
  super().startTest(test);self.begin=time.monotonic();(out/'supports-current-r85-progress.json').write_text(json.dumps({'active':identity(test),'completed':completed,'seconds':time.monotonic()-started}))
 def stopTest(self,test):
  completed.append({'id':identity(test),'seconds':time.monotonic()-self.begin});super().stopTest(test);(out/'supports-current-r85-progress.json').write_text(json.dumps({'active':None,'completed':completed,'seconds':time.monotonic()-started}))
result=Result();unittest.TestSuite(selected).run(result);after={'source':source(),'provider_canonical_model':provider(),'production':hashes(),'runtime_code':runtime()};record={'hook_count':len(allids),'selected_ids':ids,'run':result.testsRun,'failures':[(identity(t),msg) for t,msg in result.failures],'errors':[(identity(t),msg) for t,msg in result.errors],'seconds':time.monotonic()-started,'completed':completed,'before':before,'after':after,'stable':before==after}
(out/'supports-current-r85-result.json').write_text(json.dumps(record));print(json.dumps({k:v for k,v in record.items() if k not in ('before','after','completed')}))
