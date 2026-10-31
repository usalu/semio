"""🚶 Execute current source-bound hall, fixture, mounting and timber acceptance before one coordinated publication."""
import copy,hashlib,importlib,inspect,json,runpy,sys,time,traceback,types,threading,_thread
from pathlib import Path
R=Path.cwd();P=Path(__file__).parent;O=P/'🗑️generated/native-r95-hall';sys.path.insert(0,str(R));sys.dont_write_bytecode=True
A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');started=time.monotonic();candidate=json.loads((O/'skirting-candidate.json').read_text(encoding='utf8'));cache=json.loads((O/'skirting-geometry.json').read_text(encoding='utf8'));assert candidate['source_stable'] and not candidate['stable'] and cache['snapshot']==candidate['before'] and cache['model_canonical_sha256']==candidate['model_canonical_sha256']
for name,value in candidate['after'].items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==value,('changed-input',name)
D=Path(A.__file__).parent/'🧪️tests';names=('🏠ground','🚿️niches','🪵️skirting','🚶️hall');paths={R/name for name in candidate['after']}|{Path(__file__),P/'native-hall-bind-r95.py',O/'skirting-candidate.json',O/'skirting-geometry.json'}
for name in names:
 for suffix in ('.py','.json','.schema.json'):paths.add(D/(name+suffix))
paths.add(D/'🛁️details.py')
def fingerprints():
    """🔐 Capture exact current author, geometry, helper, body and independent fixture inputs."""
    return {str(path.relative_to(R)):hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(paths)}
before=fingerprints();m=runpy.run_path(str(P/'native-hall-bind-r95.py'))['prepare'](A,A.load_model(),candidate);g=cache['geometry'];g['details'].extend(A._solid_geometry(m,obj) for obj in candidate['elements'])
def signature(value):
    """🧾 Keep structural data identity independent of Python dictionary order or aliases."""
    return hashlib.sha256(json.dumps(value,sort_keys=True,separators=(',',':'),allow_nan=False).encode()).hexdigest()
model_signature=signature(m);geometry_signature=signature(g)
def code_value(value):
    """🧬 Encode actual selected function code and compiler constants without reference bookkeeping."""
    if isinstance(value,types.CodeType):return [value.co_code.hex(),value.co_name,value.co_qualname,value.co_argcount,value.co_posonlyargcount,value.co_kwonlyargcount,list(value.co_names),list(value.co_varnames),[code_value(item) for item in value.co_consts]]
    if isinstance(value,tuple):return ['tuple',[code_value(item) for item in value]]
    return repr(value)
def code_digest(code):
    """🧪 Compare loaded test code against fresh compilation of the exact current source bytes."""
    return hashlib.sha256(json.dumps(code_value(code),sort_keys=True).encode()).hexdigest()
def observed(function):
    """📜 Require the actual selected body to exist in a current-source compilation."""
    pending=[compile(Path(function.__code__.co_filename).read_bytes(),function.__code__.co_filename,'exec',dont_inherit=True,optimize=sys.flags.optimize)];digests=[]
    while pending:
        code=pending.pop()
        if code.co_qualname==function.__code__.co_qualname:digests.append(code_digest(code))
        pending.extend(value for value in code.co_consts if isinstance(value,types.CodeType))
    digest=code_digest(function.__code__);assert digest in digests
    return {'module':function.__module__,'qualname':function.__qualname__,'file':function.__code__.co_filename,'code_sha256':digest,'fresh_code_equal':True}
def current_geometry(value=None):
    """🏛️ Reuse only the proven complete nominal native geometry for an identical current candidate model."""
    assert value is None or signature(value)==model_signature,'A changed model cannot consume captured nominal geometry'
    return g
rows=[];stop=threading.Event()
def cancel():
    """🛑 Keep expensive source contact and body checks cancellable without native traceback timers."""
    while not stop.wait(.5):
        if (O/'skirting.cancel').exists() or (P/'native-r92-cancel').exists():_thread.interrupt_main();return
threading.Thread(target=cancel,daemon=True).start();original_load,original_geometry=A.load_model,A.building_geometry
try:
    A.load_model=lambda:copy.deepcopy(m);A.building_geometry=current_geometry
    for name in names:
        module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.'+name)
        for identity,function in inspect.getmembers(module,inspect.isfunction):
            if not identity.startswith('test_'):continue
            print('[DEBUG] Candidate acceptance '+name+'.'+identity,flush=True);entry=observed(function);begin=time.monotonic();error=None
            try:function()
            except Exception:error=traceback.format_exc();print('[DEBUG] '+error,flush=True)
            rows.append({**entry,'seconds':time.monotonic()-begin,'error':error});(O/'acceptance-progress.json').write_text(json.dumps({'rows':len(rows),'last':identity,'seconds':time.monotonic()-started}),encoding='utf8')
finally:A.load_model,A.building_geometry=original_load,original_geometry;stop.set()
after=fingerprints();unchanged=signature(m)==model_signature and signature(g)==geometry_signature
report={'success':before==after and unchanged and len(rows)==14 and not any(row['error'] for row in rows),'candidate_only':True,'publishable':False,'source_stable':before==after,'model_and_geometry_unchanged':unchanged,'model_canonical_sha256':model_signature,'candidate_sha256':hashlib.sha256((O/'skirting-candidate.json').read_bytes()).hexdigest(),'inputs_before':before,'inputs_after':after,'rows':rows,'seconds':time.monotonic()-started,'scope':'Actual complete candidate native G plus all102new native stock bodies; unchanged77cm paths/complete panels, qualified89degree operation,90degree positive interference, independent floor support/containment/mounts, source fixtures and SciPy stock volumes. Nominal captured geometry is allowed only for byte-equivalent candidate models; no event-mutated model is substituted.'}
(O/'acceptance.json').write_text(json.dumps(report,indent=2),encoding='utf8');print('[DEBUG] Complete candidate acceptance '+str(report['success'])+' '+str(len(rows))+' '+str(report['seconds']),flush=True);assert report['success']
