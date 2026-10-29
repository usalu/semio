"""🧪 Verify current explicit feature scopes with source, producer and loaded-bytecode evidence."""
import hashlib,importlib,inspect,json,marshal,math,os,runpy,sys,tempfile,time,unittest
from pathlib import Path
root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');ticket=Path(__file__).parent;generated=ticket/'🗑️generated';mode=globals().get('cohort_mode','features');stem='parking-r85' if mode=='parking' else 'features-r85';source=root/'🧱️model/🔣️.json';expected='528b3ffeab26e4c038b9eb603e7ae1e9bc52c66a1f068b3e799de8fcd66ccc8f';prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
output=generated/'rt85';output.mkdir(exist_ok=True);os.environ['SEMIO_TEACHING_OUTPUT_ROOT']=str(output);tempfile.tempdir=str(output)
def digest(data):return hashlib.sha256(data).hexdigest()
def write(name,value):
    (generated/name).write_text(json.dumps(value,ensure_ascii=False,allow_nan=False),encoding='utf8')
write(stem+'-progress.json',{'debug':'[DEBUG] current cohort','phase':'loading','completed':0})
assert digest(source.read_bytes())==expected
ns=runpy.run_path(str(root/'🧪️tests/🐍️.py'));api=ns['model_api']()
def canonical(test):
    fn=getattr(test,'_testFunc',None)
    if fn:
        bound=next((value for value in fn.__defaults__ or () if inspect.ismodule(value)),None)
        return bound.__name__+'.run' if bound and fn.__name__=='<lambda>' else fn.__module__+'.'+fn.__qualname__
    value=test.id()
    return prefix+'🧪️tests.🐍️.'+value[len('<run_path>.'):] if value.startswith('<run_path>.') else value
def flatten(suite):
    for test in suite:
        if isinstance(test,unittest.TestSuite):yield from flatten(test)
        else:yield test
registered=list(flatten(ns['load_tests'](unittest.defaultTestLoader,unittest.TestSuite(),None)));ids=[canonical(t) for t in registered];assert len(ids)==len(set(ids))==355
support=json.loads((ticket/'feature-prefix-r62.json').read_text())['passed_canonical_ids'][:6];assert len(support)==6 and all('.SparseSupportContract.' in key for key in support)
parking=[key for key in ids if '.ParkingContract.' in key];assert len(parking)==10 and set(support)<=set(ids)
selected=[test for test in registered if canonical(test) in parking] if mode=='parking' else [test for test in registered if canonical(test) not in set(support+parking)]
assert len(selected)==(10 if mode=='parking' else 339)
boundary=[]
if mode!='parking':
    for name in ('BimLanguageContract','BimSelectedSourceContract'):boundary.extend(flatten(unittest.defaultTestLoader.loadTestsFromTestCase(ns[name])))
    assert len(boundary)==7
selected_ids=[canonical(t) for t in selected];boundary_ids=[canonical(t) for t in boundary]
write(stem+'-selected.json',{'registered_ids':ids,'selected_ids':selected_ids,'excluded_support_ids':support,'excluded_parking_ids':[] if mode=='parking' else parking,'runtime_boundary_ids':boundary_ids,'source':expected,'scope':'actual registered load_tests; runtime7 existing mocked boundaries separate'})
if boundary:write('runtime-r85-selected.json',{'selected_ids':boundary_ids,'native_commands':False,'native_frames':'unverified'})
paths=set(json.loads((generated/'clips-baseline-r33-after-manifest.json').read_text())['production'])|{'🧱️model/🔣️.json','🧱️model/🧬️schema/🔣️.schema.json','🧱️model/🧮️analysis/🧬️schema/🚧️clash.schema.json','🎨️visuals/🏛️savoye/🧊️axon/📏️geometry.json'}
all_paths=set(json.loads((generated/'features-r68-inputs.json').read_text())['files_before'])|{'🧱️model/🔌️exchange/🧪️tests/🪑furnishings.py','🧱️model/🔌️exchange/🧪️tests/🪑furnishings.json','🧱️model/🧪️tests/🧺washfixtures.json'}
def files(names):return {p:digest((root/p).read_bytes()) for p in sorted(names)}
def provider():return digest(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode())
def atom(value,depth=0):
    if depth>8:return {'type':type(value).__module__+'.'+type(value).__qualname__}
    if value is None or isinstance(value,(str,bool,int)):return value
    if isinstance(value,float):return value if math.isfinite(value) else {'float':str(value)}
    if isinstance(value,(tuple,list)):return [atom(v,depth+1) for v in value]
    if isinstance(value,dict):return {str(k):atom(v,depth+1) for k,v in value.items()}
    if isinstance(value,Path):return str(value)
    return {'type':type(value).__module__+'.'+type(value).__qualname__,'module':getattr(value,'__module__',None),'name':getattr(value,'__qualname__',getattr(value,'__name__',None))}
def function(value):
    try:value=inspect.unwrap(value)
    except ValueError:return None
    value=getattr(value,'__func__',value)
    if not hasattr(value,'__code__'):return None
    return {'code':digest(marshal.dumps(value.__code__)),'defaults':atom(getattr(value,'__defaults__',None)),'keyword_defaults':atom(getattr(value,'__kwdefaults__',None)),'closure':[atom(cell.cell_contents) for cell in value.__closure__ or ()]}
def selected_runtime(test):
    fn=getattr(test,'_testFunc',None)
    if fn:
        result={'function':function(fn)};bound=next((value for value in fn.__defaults__ or () if inspect.ismodule(value)),None)
        if bound and fn.__name__=='<lambda>':result['bound_run']=function(bound.run)
        return result
    return {name:function(getattr(test,name)) for name in (test._testMethodName,'setUp','tearDown','setUpClass','tearDownClass') if hasattr(test,name)}
def runtime():
    records={}
    for name,module in sorted(tuple(sys.modules.items())):
        file=getattr(module,'__dict__',{}).get('__file__')
        if not file or not str(file).startswith('/repo/'):continue
        for key,value in tuple(module.__dict__.items()):
            if getattr(value,'__module__',None)!=name:continue
            record=function(value)
            if record:records[name+'::'+key]=record
            elif inspect.isclass(value):
                for member,method in tuple(value.__dict__.items()):
                    record=function(method)
                    if record:records[name+'::'+key+'.'+member]=record
    text=json.dumps(records,sort_keys=True,ensure_ascii=False,allow_nan=False);key=digest(text.encode());path=generated/('runtime-code-r85-'+key+'.json')
    if not path.exists():path.write_text(text,encoding='utf8')
    return key
before={'source':digest(source.read_bytes()),'provider':provider(),'production':files(paths),'all_recorded_files':files(all_paths),'selected_runtime':{canonical(t):selected_runtime(t) for t in selected+boundary},'loaded_runtime_snapshot':runtime()};write(stem+'-before.json',before)
phases=[];current_scope='parking' if mode=='parking' else 'features'
class Result(unittest.TestResult):
    """⏱️ Record exact test identities and real loaded producer bytecode at every boundary."""
    def save(self,phase,test):
        write(stem+'-progress.json',{'debug':'[DEBUG] current '+current_scope,'phase':phase,'test':canonical(test),'completed':len(phases),'planned':len(selected)+len(boundary),'failures':len(self.failures),'errors':len(self.errors),'phases':phases})
        write(stem+'-failures.json',{'failures':[(canonical(t),m) for t,m in self.failures],'errors':[(canonical(t),m) for t,m in self.errors]})
    def startTest(self,test):
        super().startTest(test);self.started=time.monotonic();self.start_state={'source':digest(source.read_bytes()),'provider':provider(),'runtime':runtime(),'selected_runtime':selected_runtime(test)};self.save('started',test)
    def stopTest(self,test):
        after={'source':digest(source.read_bytes()),'provider':provider(),'runtime':runtime(),'selected_runtime':selected_runtime(test)};phases.append({'id':canonical(test),'scope':current_scope,'seconds':time.monotonic()-self.started,'before':self.start_state,'after':after});self.save('completed',test)
        if current_scope=='runtime':write('runtime-r85-progress.json',{'debug':'[DEBUG] mocked runtime boundaries','completed':sum(p['scope']=='runtime' for p in phases),'planned':7,'test':canonical(test),'failures':len(self.failures),'errors':len(self.errors)})
        super().stopTest(test)
        if (generated/(stem+'-cancel')).exists():self.stop()
    def addFailure(self,test,error):super().addFailure(test,error);self.save('failure',test)
    def addError(self,test,error):super().addError(test,error);self.save('error',test)
result=Result();started=time.monotonic();unittest.TestSuite(selected).run(result)
if boundary and not result.shouldStop:
    current_scope='runtime';unittest.TestSuite(boundary).run(result)
after={'source':digest(source.read_bytes()),'provider':provider(),'production':files(paths),'all_recorded_files':files(all_paths),'selected_runtime':{canonical(t):selected_runtime(t) for t in selected+boundary},'loaded_runtime_snapshot':runtime()}
record={'scope':mode,'registered':355,'selected_feature_ids':selected_ids,'runtime_boundary_ids':boundary_ids,'run':result.testsRun,'seconds':time.monotonic()-started,'failures':[(canonical(t),m) for t,m in result.failures],'errors':[(canonical(t),m) for t,m in result.errors],'cancelled':result.shouldStop,'before':before,'after':after,'source_stable':before['source']==after['source'],'provider_stable':before['provider']==after['provider'],'production_stable':before['production']==after['production'],'selected_runtime_stable':before['selected_runtime']==after['selected_runtime'],'changed_recorded_files':[p for p in sorted(set(before['all_recorded_files'])|set(after['all_recorded_files'])) if before['all_recorded_files'].get(p)!=after['all_recorded_files'].get(p)],'phases':phases,'native_frames':'unverified'}
write(stem+'-summary.json',record)
if boundary:write('runtime-r85-summary.json',{'selected_ids':boundary_ids,'run':sum(p['scope']=='runtime' for p in phases),'failures':[(canonical(t),m) for t,m in result.failures if canonical(t) in boundary_ids],'errors':[(canonical(t),m) for t,m in result.errors if canonical(t) in boundary_ids],'phases':[p for p in phases if p['scope']=='runtime'],'native_frames':'unverified'})
print({k:record[k] for k in ('scope','run','seconds','source_stable','provider_stable','production_stable','selected_runtime_stable','changed_recorded_files')},'failures',len(result.failures),'errors',len(result.errors))
