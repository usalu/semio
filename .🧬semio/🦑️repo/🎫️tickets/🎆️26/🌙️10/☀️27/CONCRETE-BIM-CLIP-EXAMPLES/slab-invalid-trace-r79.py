import importlib,json,hashlib
from pathlib import Path
from shapely.geometry import Polygon
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();slab=api.element('SL-STAIR-CANOPY',m);original=api._analysis._clip;bad=[]
def clip(a,b,*args,**kwargs):
 out=original(a,b,*args,**kwargs)
 if len(out)>=3 and not Polygon(out).is_valid:bad.append({'subject':a,'clip':b,'args':args,'kwargs':kwargs,'output':out})
 return out
api._analysis._clip=clip
g=api._slab_geometry(m,slab)
invalid=[{'index':i,'region':p.get('region'),'part':p} for i,p in enumerate(g['full_parts']) if not Polygon(p['outline']).is_valid]
result={'source_hash':hashlib.sha256(Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json').read_bytes()).hexdigest(),'slab':slab['tag'],'parts':len(g['full_parts']),'invalid':invalid,'invalid_clip_calls':bad,'outline':g['outline'],'regions':g['regions']}
Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/slab-invalid-trace-r79.json').write_text(json.dumps(result));print(json.dumps(result))
