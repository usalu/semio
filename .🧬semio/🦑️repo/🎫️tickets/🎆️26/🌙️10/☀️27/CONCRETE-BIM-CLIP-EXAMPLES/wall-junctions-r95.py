"""🔗 Retain source-bound declared wall junction RED/GREEN native and independent results."""
import hashlib,importlib,inspect,json,sys,time,traceback
from pathlib import Path
R=Path.cwd();sys.path.insert(0,str(R));P=Path(__file__).parent;O=P/'🗑️generated/wall-junctions-r95';O.mkdir(parents=True,exist_ok=True);stage=sys.argv[1];T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔗️wall-junctions');paths=[Path(T.__file__),Path(T.__file__).with_suffix('.json'),Path(T.__file__).with_suffix('.schema.json'),Path(T.A.__file__),Path(T.A.__file__).parent/'🧬️schema/🔣️.schema.json',Path(__file__)]
def hashes():
    """🔐 Capture actual selected native/source/schema and first-party independent fixture bytes."""
    return {str(path.relative_to(R)):hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}
paths.extend([Path(T.A.__file__).with_name('🔣️.json'),Path(T.A._wall_junction_backend().__file__),Path(T.A._exchange.__file__),Path(T.A.__file__).parent.parent/'🧪️tests/🐍️.py'])
paths.append(Path(T.__file__).with_name('🔗️wall-csg.json'))
paths.append(Path(T.__file__).with_name('🔗️wall-csg.schema.json'))
before=hashes();rows=[];started=time.monotonic()
for name,fn in inspect.getmembers(T,inspect.isfunction):
    if not name.startswith('test_'):continue
    print('[DEBUG] Wall junction '+name,flush=True);error=None;begin=time.monotonic()
    try:fn()
    except Exception as failure:error=traceback.format_exc();print('[DEBUG] '+type(failure).__name__+': '+getattr(failure,'message',str(failure).splitlines()[0]),flush=True)
    rows.append({'id':name,'error':error,'seconds':time.monotonic()-begin})
after=hashes();report={'stage':stage,'success':all(row['error'] is None for row in rows),'stable':before==after,'before':before,'after':after,'rows':rows,'seconds':time.monotonic()-started};(O/(stage+'.json')).write_text(json.dumps(report,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({key:value for key,value in report.items() if key not in ('before','after','rows')}),flush=True)
if stage=='red' or stage.endswith('-red'):assert not report['success'] and any(row['id']=='test_undeclared_wall_penetration_remains_visible' and row['error'] is None for row in rows)
else:assert report['success'] and report['stable']
