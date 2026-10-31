"""🔬 Retain actual invalid planar ground-slab observations before any numeric repair."""
import importlib,json,sys
from pathlib import Path
from shapely.geometry import Polygon
from shapely.validation import explain_validity
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();g=api._slab_geometry(m,api._find(m,'SL-GROUND'));rows=[]
for i,p in enumerate(g['parts']):
    shape=Polygon(p['outline'])
    if not shape.is_valid or shape.area<=0:rows.append({'index':i,'reason':explain_validity(shape),'area':shape.area,'part':p})
ticket=Path(__file__).parent;out={'parts':len(g['parts']),'invalid':rows};(ticket/'🗑️generated/garage-slab-topology-r92.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');(ticket/'garage-slab-topology-r92.md').write_text('# Ground Slab Topology Investigation\n\nThe original ground-plate contract exposed'+str(len(rows))+' invalid or nonpositive outline observations after the garage threshold extension. This is a real independent planar-validity failure, despite the sparse native exchange aggregate/occupancy pass. No outline is discarded or healed. Detailed source vertices remain in the generated diagnostic for a targeted original-operand repair.\n',encoding='utf8');print('[DEBUG] '+json.dumps({'parts':len(g['parts']),'invalid':rows},ensure_ascii=False))
