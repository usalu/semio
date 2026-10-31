"""📍 Capture exact authored floor/support mismatch."""
import sys,importlib,json
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();sl=a._slab_geometry(m,a._find(m,'SL-1'));caps={}
for p in sl['full_parts']:caps.setdefault(p['z1'],[]).append(Polygon(p['outline']))
caps={z:unary_union(r) for z,r in caps.items()};out=[]
for e in m['elements']:
 if e['class']!='IfcCovering' or e.get('predefined_type')!='FLOORING':continue
 for i,p in enumerate(e['geometry']['parts']):
  if p['role']!='bedding':continue
  poly=Polygon(p['outline']);z=round(3.17+p['z0'],9)
  if poly.difference(caps[z]).area>1e-7:out.append({'tag':e['tag'],'part':i,'z':z,'outside':poly.difference(caps[z]).area,'outline':p['outline']})
print('[DEBUG] '+json.dumps(out[:5]),flush=True);(Path(__file__).parent/'🗑️generated/floor-support-probe-r92.json').write_text(json.dumps(out),encoding='utf8')
