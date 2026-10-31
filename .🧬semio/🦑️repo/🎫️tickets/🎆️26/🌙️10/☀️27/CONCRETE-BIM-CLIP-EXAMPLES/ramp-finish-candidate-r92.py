"""🧪 Validate placed wearing candidates against independent volume and actual structural owners."""
import importlib,sys,json,copy,math
from pathlib import Path
from scipy.spatial import ConvexHull
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");o=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️interfaces");m=a.load_model();delta=json.loads((Path(__file__).parent/'🗑️generated/ramp-finish-joints-r92.json').read_text());rows=[]
for row in delta['rows']:
 obj=a._find(m,row['tag']);obj['geometry']['parts']=row['parts'];obj['geometry']['base_level']=row['base_level'];parts=o.occupied(m,row['tag']);vol=0.;minimum=float('inf')
 for part in parts:
  a._polyhedron_part(a.part_vertices(part),a.part_faces(part));v=ConvexHull(a.part_vertices(part)).volume;assert v>0;vol+=v;minimum=min(minimum,v)
 errors=[]
 for owner in row['owners']:
  for cutter in a._interface_owner_parts(m,owner):
   for part in parts:
    if cutter.get('profile',{}).get('shape')=='circle':
     margin=o.circle_clearance(part,cutter)
     if margin<0:errors.append((owner,'circle',margin))
    else:
     common=o.common_volume(part,cutter)
     if common>1e-10:errors.append((owner,common))
 assert not errors,errors
 rows.append({'tag':row['tag'],'parts':len(parts),'volume':vol,'minimum':minimum,'independent_removed':row['independent_removed']});print('[DEBUG] '+json.dumps(rows[-1]),flush=True)
(Path(__file__).parent/'🗑️generated/ramp-finish-candidate-r92.json').write_text(json.dumps(rows,indent=2),encoding='utf8')
