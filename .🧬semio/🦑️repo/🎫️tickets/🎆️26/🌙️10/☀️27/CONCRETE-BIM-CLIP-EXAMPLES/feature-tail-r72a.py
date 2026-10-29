"""🧪 Run explicit registered feature indices49 through194 with observable immutable-scope evidence."""
import ast,hashlib,inspect,json,runpy,time,unittest
from pathlib import Path
root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');ticket=Path(__file__).parent;generated=ticket/'🗑️generated'
source=root/'🧱️model/🔣️.json';prior=json.loads((generated/'features-r68-inputs.json').read_text())
def digest(data):return hashlib.sha256(data).hexdigest()
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
raw_before=digest(source.read_bytes());assert raw_before==prior['source_sha256']=='d2f3c0d1bb97c93836fb61b53adfe94756c59c31217585899decf09cda4f59f2'
ns=runpy.run_path(str(root/'🧪️tests/🐍️.py'));api=ns['model_api']();tests=list(flatten(ns['load_tests'](unittest.defaultTestLoader,unittest.TestSuite(),None)))
all_ids=[canonical(t) for t in tests];assert len(all_ids)==len(set(all_ids))==351
wanted=prior['selected_ids'][49:195];assert len(prior['selected_ids'])==341 and len(wanted)==146
assert len(set(wanted))==146 and set(wanted)<=set(all_ids)
lookup=dict(zip(all_ids,tests));selected=[lookup[key] for key in wanted]
core=json.loads((generated/'clips-baseline-r33-after-manifest.json').read_text());production_paths=sorted(core['production'])
def production():return {p:digest((root/p).read_bytes()) for p in production_paths}
def provider():return digest(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode())
modules={};selected_ast={}
for test in selected:
    fn=getattr(test,'_testFunc',None)
    if fn:
        bound=next((v for v in fn.__defaults__ or () if inspect.ismodule(v)),None)
        func=bound.run if bound and fn.__name__=='<lambda>' else fn
    else:func=getattr(test,test._testMethodName)
    path=Path(inspect.getsourcefile(func));relative=str(path.relative_to(root));modules.setdefault(relative,set()).add(func.__name__)
def ast_scope():
    output={}
    for relative,names in modules.items():
        tree=ast.parse((root/relative).read_text())
        for node in tree.body:
            if isinstance(node,ast.ClassDef):node.body=[n for n in node.body if not isinstance(n,(ast.FunctionDef,ast.AsyncFunctionDef)) or not n.name.startswith('test_') or n.name in names]
        tree.body=[n for n in tree.body if not isinstance(n,(ast.FunctionDef,ast.AsyncFunctionDef)) or not n.name.startswith('test_') or n.name in names]
        output[relative]=digest(ast.dump(tree,include_attributes=False).encode())
    return output
before={'source':raw_before,'provider':provider(),'production':production(),'selected_test_ast':ast_scope()}
inputs={'range_inclusive':[49,194],'selected_ids':wanted,'registered':351,'planned':146,'before':before,'selected_module_names':{p:sorted(n) for p,n in modules.items()},'scope':'actual load_tests selection; nonselected test methods excluded from AST continuity'}
(generated/'features-r72a-inputs.json').write_text(json.dumps(inputs,ensure_ascii=False),encoding='utf8')
phases=[]
class Result(unittest.TestResult):
    """⏱️ Persist every test boundary and failure without native rendering."""
    def save(self,phase,test):
        record={'debug':'[DEBUG] feature split r72a','phase':phase,'test':canonical(test),'completed':len(phases),'planned':146,'failures':len(self.failures),'errors':len(self.errors),'phases':phases}
        (generated/'features-r72a-progress.json').write_text(json.dumps(record,ensure_ascii=False),encoding='utf8')
        (generated/'features-r72a-failures.json').write_text(json.dumps({'failures':[(canonical(t),m) for t,m in self.failures],'errors':[(canonical(t),m) for t,m in self.errors]},ensure_ascii=False),encoding='utf8')
    def startTest(self,test):super().startTest(test);self.started=time.monotonic();self.save('started',test)
    def stopTest(self,test):
        phases.append({'id':canonical(test),'seconds':time.monotonic()-self.started});self.save('completed',test);super().stopTest(test)
        if (generated/'features-r72a-cancel').exists():self.stop()
    def addFailure(self,test,error):super().addFailure(test,error);self.save('failure',test)
    def addError(self,test,error):super().addError(test,error);self.save('error',test)
result=Result();started=time.monotonic();unittest.TestSuite(selected).run(result)
after={'source':digest(source.read_bytes()),'provider':provider(),'production':production(),'selected_test_ast':ast_scope()}
record={**inputs,'after':after,'run':result.testsRun,'seconds':time.monotonic()-started,'failures':[(canonical(t),m) for t,m in result.failures],'errors':[(canonical(t),m) for t,m in result.errors],'phases':phases,'cancelled':result.shouldStop,'source_stable':before['source']==after['source'],'provider_stable':before['provider']==after['provider'],'production_stable':before['production']==after['production'],'selected_test_ast_stable':before['selected_test_ast']==after['selected_test_ast']}
(generated/'features-r72a-result.json').write_text(json.dumps(record,ensure_ascii=False),encoding='utf8')
print({k:record[k] for k in ['run','seconds','source_stable','provider_stable','production_stable','selected_test_ast_stable']},'failures',len(result.failures),'errors',len(result.errors))
