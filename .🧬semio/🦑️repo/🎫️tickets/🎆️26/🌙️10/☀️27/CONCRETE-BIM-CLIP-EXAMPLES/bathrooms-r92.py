"""🚽 Execute current sanitary contracts with stable source and actual result records."""
import importlib,inspect,json,sys,time,traceback,hashlib
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
stage=sys.argv[1];start=time.monotonic();source=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';before=hashlib.sha256(source.read_bytes()).hexdigest();rows=[]
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛁️bathrooms')
for name,fn in inspect.getmembers(module,inspect.isfunction):
    if not name.startswith('test_'):continue
    if stage=='guest-route' and name!='test_original_guest_niche_routes_keep_full_operated_leaf_and_actual_torso_clear':continue
    if stage=='red' and name!='test_original_shared_bathroom_has_distinct_bidet_basin_and_capsule_tub':continue
    error=None;beg=time.monotonic();print('[DEBUG] '+name,flush=True)
    try:fn()
    except Exception:error=traceback.format_exc();print(error,flush=True)
    rows.append({'id':module.__name__+'.'+name,'error':error,'seconds':time.monotonic()-beg})
result={'stage':stage,'rows':rows,'source_before':before,'source_after':hashlib.sha256(source.read_bytes()).hexdigest(),'seconds':time.monotonic()-start,'pass':all(r['error'] is None for r in rows)};(Path(__file__).parent/'🗑️generated'/('bathrooms-'+stage+'-r92.json')).write_text(json.dumps(result,indent=2),encoding='utf8');print(json.dumps({k:v for k,v in result.items()if k!='rows'}));raise SystemExit(not result['pass'])
