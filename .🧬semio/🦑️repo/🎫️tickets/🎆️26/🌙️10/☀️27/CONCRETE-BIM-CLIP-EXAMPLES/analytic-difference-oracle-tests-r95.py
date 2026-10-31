"""🧪 Cross-check the shared semantic difference vectors with an independent SciPy/GEOS interval oracle."""
import hashlib,importlib.util,json,math,sys,time,traceback
from pathlib import Path
from jsonschema import Draft202012Validator
sys.dont_write_bytecode=True
R=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());T=Path(__file__).parent;D=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧊️solids/➖️difference';oracle_path=T/'analytic-difference-oracle-r95.py';paths=[Path(__file__),D/'🧬️.schema.json',D/'🧪️tests/🔣️.json']+([oracle_path]if oracle_path.is_file()else[])+[D/'🧪️tests/🔬️scipy.py']
def hashes():
    """🔐 Bind the independent oracle and the actual shared language-neutral CSG contract."""
    return{str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
before=hashes();schema=json.loads(paths[1].read_text(encoding='utf8'));case=json.loads(paths[2].read_text(encoding='utf8'));validator=Draft202012Validator(schema);oracle=None
if oracle_path.is_file():
    spec=importlib.util.spec_from_file_location('analytic_difference_independent_r95',oracle_path);oracle=importlib.util.module_from_spec(spec);spec.loader.exec_module(oracle)
rows=[];start=time.monotonic()
for c in case['cases']:
    row={'id':c['id'],'pass':False}
    try:
        validator.validate(c['part']);assert oracle is not None,'Independent analytic CSG oracle is absent';volume,error=oracle.volume(c['part']['csg']);expected=c['expected'];assert volume>=0 and error>=0
        if expected.get('owned_volume')==0:assert volume==0,(volume,error,expected)
        elif 'owned_volume'in expected:assert math.isclose(volume,expected['owned_volume'],rel_tol=2e-12,abs_tol=1e-13),(volume,error,expected)
        if expected.get('owned_volume_positive'):assert volume>0
        if expected.get('unchanged_stock'):
            stock,error_stock=oracle.volume(c['part']['csg']['stock']);assert volume==stock
        if 'full_volume'in expected:
            full=c['part']['csg']['stock']
            while full.get('operator')=='difference':full=full['stock']
            reference,error_reference=oracle.volume(full);assert math.isclose(reference,expected['full_volume'],rel_tol=1e-13,abs_tol=1e-13)
        if c.get('cancel_after_progress'):
            events=[];cancelled=False
            try:oracle.volume(c['part']['csg'],cancel=lambda:len(events)>=c['cancel_after_progress'],progress=events.append)
            except InterruptedError:cancelled=True
            assert cancelled and len(events)==c['cancel_after_progress'];row['cancelled_after_progress']=True
        row.update(volume=volume,estimated_absolute_error=error);row['pass']=True
    except Exception:row['error']=traceback.format_exc()
    rows.append(row)
after=hashes();out={'pass':all(r['pass']for r in rows)and before==after,'stable':before==after,'cases':len(rows),'rows':rows,'before':before,'after':after,'seconds':time.monotonic()-start,'scope':'Independent SciPy real-section integration and GEOS polygon slicing of the declared semantic CSG, with no display-mesh authority and no production implementation import.'};(T/'🗑️generated/analytic-difference-oracle-tests-r95.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k not in('rows','before','after')}),flush=True);print('[DEBUG] failing_cases='+str([r['id']for r in rows if not r['pass']]),flush=True);raise SystemExit(not out['pass'])
