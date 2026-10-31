"""🔬 Retain actual independent analytic union RED/GREEN and unchanged intersection semantics."""
import hashlib,importlib,inspect,json,sys,time,traceback
from pathlib import Path
R=Path.cwd();sys.path.insert(0,str(R));T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🧪️tests.🔬️union');D=Path(T.__file__).parent;O=Path(__file__).parent/'🗑️generated/analytic-union-r95';O.mkdir(parents=True,exist_ok=True);stage=sys.argv[1];paths=[Path(__file__),Path(T.__file__),Path(T.__file__).with_suffix('.json'),Path(T.__file__).with_suffix('.schema.json'),Path(T.ORACLE.__file__),D.parent/'🧬️.schema.json'];before={str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths};rows=[];started=time.monotonic()
for name,fn in inspect.getmembers(T,inspect.isfunction):
    if not name.startswith('test_'):continue
    error=None
    try:fn()
    except Exception:error=traceback.format_exc()
    rows.append({'id':name,'error':error});print('[DEBUG] '+name+' '+str(error is None),flush=True)
after={str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths};out={'success':not any(row['error'] for row in rows),'stable':before==after,'before':before,'after':after,'rows':rows,'seconds':time.monotonic()-started};(O/(stage+'.json')).write_text(json.dumps(out,indent=2),encoding='utf8');assert out['stable'] and out['success']!=(stage=='red')
