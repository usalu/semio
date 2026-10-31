"""🛡️ Execute the permanent source/structure witness contracts with exact input provenance."""
import hashlib,importlib,json,sys,time,traceback
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚿️ground');source=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';before=hashlib.sha256(source.read_bytes()).hexdigest();rows=[]
for name in ['test_registered_ground_sanitary_programme_and_space_ownership','test_registered_bath_clearance_keeps_the_structural_column_occupied']:
    start=time.monotonic();error=None;print('[DEBUG] '+name,flush=True)
    try:getattr(module,name)()
    except Exception:error=traceback.format_exc();print(error,flush=True)
    rows.append({'id':module.__name__+'.'+name,'error':error,'seconds':time.monotonic()-start})
out={'source_before':before,'source_after':hashlib.sha256(source.read_bytes()).hexdigest(),'rows':rows,'pass':all(r['error']is None for r in rows)};(Path(__file__).parent/'🗑️generated'/'bath-column-controls-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps(out),flush=True);raise SystemExit(not out['pass'])
