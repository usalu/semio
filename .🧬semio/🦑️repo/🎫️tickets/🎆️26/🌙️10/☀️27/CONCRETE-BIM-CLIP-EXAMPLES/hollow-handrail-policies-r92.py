"""🧪 Verify updated handrail policy assertions through their registered native source contracts."""
import importlib,json,sys,time,traceback
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True;start=time.monotonic();rows=[]
for name,fn in [('🏛️suite','test_u_handrails_keep_hollow_material_and_registered_miter_routes'),('🪜️stairs','test_original_roof_ramp_guard_is_open_tubular_construction')]:
    module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.'+name);error=None
    try:getattr(module,fn)()
    except Exception:error=traceback.format_exc();print('[DEBUG] '+error,flush=True)
    rows.append({'test':fn,'error':error});print('[DEBUG] '+fn+' '+('PASS'if error is None else'FAIL'),flush=True)
out={'pass':all(r['error']is None for r in rows),'rows':rows,'seconds':time.monotonic()-start};(Path(__file__).parent/'🗑️generated'/'hollow-handrail-policies-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');raise SystemExit(not out['pass'])
