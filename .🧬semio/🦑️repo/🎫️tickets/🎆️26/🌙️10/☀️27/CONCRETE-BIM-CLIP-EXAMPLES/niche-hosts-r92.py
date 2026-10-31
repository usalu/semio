"""🏗️ Capture actual host material interfaces for inferred wall-basin construction."""
import importlib,json,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();rows=[]
for tag in ['W-G-W','W-G-E','W-G-ROOM-13','W-G-SERVICE']:
    g=a._wall_geometry(m,a.element(tag,m));parts=[p for p in g['parts']if p['z0']<.8<p['z1']];rows.append({'tag':tag,'parts':[{'outline':p['outline'],'material':p['material']}for p in parts]});print('[DEBUG] '+tag,flush=True)
(Path(__file__).parent/'🗑️generated'/'niche-hosts-r92.json').write_text(json.dumps(rows),encoding='utf8')
