"""🧪 Exercise cancellation and section authority through the actual auditor interface."""
import hashlib,importlib.util,json,sys,time,traceback
from pathlib import Path
from jsonschema import Draft202012Validator
sys.dont_write_bytecode=True
T=Path(__file__).parent;R=next(p for p in T.parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(R));D=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧊️solids/➖️difference';paths=[Path(__file__),T/'whole-expanded-interference-r92.py',T/'whole-semantic-lifecycle-r95.json',T/'whole-semantic-lifecycle-r95.schema.json',D/'🧪️tests/🔣️.json',D/'🧪️tests/🔬️scipy.py']
def hashes():
    """🔐 Bind actual callbacks, independent integration and neutral lifecycle inputs."""
    return{str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
before=hashes();cases=json.loads(paths[2].read_text(encoding='utf8'));Draft202012Validator(json.loads(paths[3].read_text(encoding='utf8'))).validate(cases);source=json.loads(paths[4].read_text(encoding='utf8'));spec=importlib.util.spec_from_file_location('whole_lifecycle_semantic',paths[1]);A=importlib.util.module_from_spec(spec);spec.loader.exec_module(A);rows=[];start=time.monotonic()
for c in cases['cases']:
    operand=next(v['part']['csg']for v in source['cases']if v['id']==c['geometry']);solid=A.rawsolid({'csg':operand});stock=operand['stock'];part={'mesh':{'vertices':stock['vertices'],'faces':stock['faces']}}if stock['kind']=='polyhedron'else{k:stock[k]for k in('outline','holes','z0','z1')};other=A.rawsolid(part);row={'id':c['id'],'pass':False};events=[]
    try:
        if c['mode']=='reject_section':
            try:solid.section((solid.lo+solid.hi)/2)
            except ValueError as error:assert'analytic sections'in str(error);row['pass']=True
        else:
            try:A.intersection(solid,other,cancel=lambda:c['mode']=='cancel_before'or bool(events),progress=events.append)
            except InterruptedError:row['pass']=True
            if c['mode']=='cancel_after_progress':assert len(events)==1 and events[0]['phase']=='independent_csg_height';row['actual_height_events']=events
    except Exception:row['error']=traceback.format_exc()
    rows.append(row)
after=hashes();out={'pass':all(r['pass']for r in rows)and before==after,'stable':before==after,'cases':len(rows),'rows':rows,'before':before,'after':after,'seconds':time.monotonic()-start};(T/'🗑️generated/whole-semantic-lifecycle-r95.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k not in('rows','before','after')}));print('[DEBUG] failures='+str([r['id']for r in rows if not r['pass']]));raise SystemExit(not out['pass'])
