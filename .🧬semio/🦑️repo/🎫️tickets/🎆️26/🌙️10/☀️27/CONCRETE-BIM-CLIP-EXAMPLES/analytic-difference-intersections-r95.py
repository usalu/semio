"""🧪 Verify analytic exclusion against real geometry rather than owner identifiers."""
import hashlib,importlib.util,json,math,sys,time,traceback
from pathlib import Path
from jsonschema import Draft202012Validator
sys.dont_write_bytecode=True
R=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());T=Path(__file__).parent;shared=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧊️solids/➖️difference/🧬️.schema.json';paths=[Path(__file__),T/'analytic-difference-oracle-r95.py',T/'analytic-difference-intersections-r95.json',T/'analytic-difference-intersections-r95.schema.json',shared,shared.parent/'🧪️tests/🔬️scipy.py']
def hashes():
    """🔐 Bind actual semantic primitives, adversarial controls and independent oracle code."""
    return{str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
before=hashes();schema=json.loads(shared.read_text(encoding='utf8'));operand=Draft202012Validator({'$ref':'#/$defs/operand','$defs':schema['$defs']});case=json.loads(paths[2].read_text(encoding='utf8'));Draft202012Validator(json.loads(paths[3].read_text(encoding='utf8'))).validate(case);spec=importlib.util.spec_from_file_location('csg_intersection_oracle_r95',paths[1]);O=importlib.util.module_from_spec(spec);spec.loader.exec_module(O);rows=[];start=time.monotonic()
for c in case['cases']:
    row={'id':c['id'],'pass':False}
    try:
        operand.validate(c['left']);operand.validate(c['right']);v,e=O.intersection_volume(c['left'],c['right']);assert v>=0 and e>=0
        if c.get('expected_zero'):assert v==0,(v,e)
        if 'expected_volume'in c:assert math.isclose(v,c['expected_volume'],rel_tol=2e-12,abs_tol=c.get('absolute_tolerance',1e-13)),(v,e,c['expected_volume'])
        if 'positive_range'in c:assert c['positive_range'][0]<v<c['positive_range'][1]and e<v/8,(v,e)
        row.update(volume=v,estimated_absolute_error=e);row['pass']=True
    except Exception:row['error']=traceback.format_exc()
    rows.append(row)
progress=[];cancelled=False
try:O.volume(case['cases'][0]['right'],cancel=lambda:True)
except InterruptedError:cancelled=True
assert cancelled;O.volume(case['cases'][0]['right'],progress=progress.append);assert progress and all(r['phase']=='independent_csg_height'for r in progress)
after=hashes();out={'pass':all(r['pass']for r in rows)and before==after,'stable':before==after,'cases':len(rows),'rows':rows,'cancelled_before_work':cancelled,'progress':progress,'before':before,'after':after,'seconds':time.monotonic()-start};(T/'🗑️generated/analytic-difference-intersections-r95.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k not in('rows','before','after')}),flush=True);print('[DEBUG] failing_cases='+str([r['id']for r in rows if not r['pass']]),flush=True);raise SystemExit(not out['pass'])
