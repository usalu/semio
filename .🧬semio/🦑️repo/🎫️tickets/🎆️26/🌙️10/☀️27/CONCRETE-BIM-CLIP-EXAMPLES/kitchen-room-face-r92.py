"""📐 Capture actual facade clear faces before reconciling room-context polygons."""
import importlib,json,sys
from pathlib import Path
r=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(r));sys.dont_write_bytecode=True
a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();g=a._wall_geometry(m,a.element('W-S',m));rows=[]
for z in [3.20,3.95,4.07]:
    parts=[p for p in g['parts']if p['z0']<z<p['z1']and any(.3<v[0]<4.75 for v in a.part_vertices(p))];rows.append({'z':z,'parts':[{'material':p['material'],'bounds':[[min(v[k]for v in a.part_vertices(p)),max(v[k]for v in a.part_vertices(p))]for k in(0,1)]}for p in parts]})
(Path(__file__).parent/'🗑️generated'/'kitchen-room-face-r92.json').write_text(json.dumps(rows,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(rows),flush=True)
