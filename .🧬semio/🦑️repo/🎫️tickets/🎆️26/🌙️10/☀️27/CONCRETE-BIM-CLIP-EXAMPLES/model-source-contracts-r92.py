"""🧬 Execute unchanged source-only standard contracts without unrelated geometry setup."""
import importlib,json,sys,time,traceback
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️');api=module.model_api();source=api.load_model();items={obj['tag']:obj for group in ('elements','types','spaces','storeys','materials') for obj in source[group]};items.update({source[key]['tag']:source[key] for key in ('project','site','building')});rows=[];start=time.monotonic()
for name in ['test_top_level_identity_and_languages','test_every_localized_text_has_both_languages','test_every_quantity_is_sourced_or_assumed_with_a_unit','test_every_element_is_complete_and_references_exist','test_every_opening_has_a_native_host_and_optional_filling','test_class_registry_reaches_ifc_root_for_every_used_class']:
    case=module.BimModelStructureContract(name);case.api=api;case.model=source;case.items=items;error=None
    try:getattr(case,name)()
    except Exception:error=traceback.format_exc();print(error,flush=True)
    rows.append({'id':'BimModelStructureContract.'+name,'error':error})
out={'seconds':time.monotonic()-start,'products':len(source['elements']),'pass':all(r['error'] is None for r in rows),'rows':rows};(Path(__file__).parent/'🗑️generated'/'model-source-contracts-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k!='rows'}),flush=True);raise SystemExit(not out['pass'])
