"""🧪 Run current guard construction contracts with a bounded source certificate."""
import hashlib,importlib,inspect,json,sys,time,traceback
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️stringer');source=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';before=hashlib.sha256(source.read_bytes()).hexdigest();rows=[];started=time.monotonic()
for name,fn in inspect.getmembers(module,inspect.isfunction):
    if not name.startswith('test_'):continue
    begin=time.monotonic();error=None
    try:fn()
    except Exception:error=traceback.format_exc();print('[DEBUG] '+error,flush=True)
    rows.append({'test':name,'error':error,'seconds':time.monotonic()-begin});print('[DEBUG] '+name+' '+('FAIL' if error else 'PASS'),flush=True)
report={'rows':rows,'source_before':before,'source_after':hashlib.sha256(source.read_bytes()).hexdigest(),'seconds':time.monotonic()-started,'pass':all(r['error'] is None for r in rows)}
(ticket/'🗑️generated'/('stringer-'+sys.argv[1]+'-r92.json')).write_text(json.dumps(report,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k!='rows'}),flush=True);raise SystemExit(not report['pass'])
