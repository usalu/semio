"""📏 Execute the actual standard column-type oracle with its minimal real fixture state."""
import hashlib,importlib,json,sys,time,traceback
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️');api=module.model_api();m=api.load_model();case=module.BimQuantityContract('test_each_column_type_volume_matches_independent_profile_integrals');case.api=api;case.model=m;case.geometry={'pilotis':[api.column_geometry(m,e)for e in m['elements']if e['class']=='IfcColumn']};start=time.monotonic();error=None
try:case.test_each_column_type_volume_matches_independent_profile_integrals()
except Exception:error=traceback.format_exc();print(error,flush=True)
result={'stage':sys.argv[1],'test':'BimQuantityContract.test_each_column_type_volume_matches_independent_profile_integrals','columns':len(case.geometry['pilotis']),'seconds':time.monotonic()-start,'pass':error is None,'error':error};(Path(__file__).parent/'🗑️generated'/('column-type-'+sys.argv[1]+'-r92.json')).write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in result.items()if k!='error'}),flush=True);raise SystemExit(error is not None)
