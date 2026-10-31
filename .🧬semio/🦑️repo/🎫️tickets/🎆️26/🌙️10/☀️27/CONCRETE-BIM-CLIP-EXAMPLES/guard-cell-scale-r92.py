"""🔍 Preserve and inspect every positive authored guard-cell scale."""
import importlib,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();rows=[]
for tag in ('GUARD-RM-1','GUARD-RM-2'):
    for i,p in enumerate(api._solid_geometry(m,api._find(m,tag))['parts']):rows.append({'tag':tag,'index':i,'volume':api._polyhedron_volume(p),'role':p.get('role'),'part':p})
rows.sort(key=lambda r:r['volume']);(ticket/'🗑️generated/guard-cell-scale-r92.json').write_text(json.dumps(rows[:20],ensure_ascii=False),encoding='utf8');print('[DEBUG] '+json.dumps([{k:v for k,v in r.items() if k!='part'} for r in rows[:10]]))
