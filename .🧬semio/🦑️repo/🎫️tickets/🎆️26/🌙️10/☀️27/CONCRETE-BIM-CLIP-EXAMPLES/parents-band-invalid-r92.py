import importlib,sys,json
from pathlib import Path
from shapely.geometry import Polygon
from shapely.validation import explain_validity
r=next(p for p in Path(__file__).resolve().parents if (p/'package.json').exists());sys.path.insert(0,str(r));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();rows=[]
for tag in ('BASIN-PARENTS','BIDET-PARENTS','FIX-BATH'):
 g=a.service_geometry(tag,m)
 for i,p in enumerate(g['parts']):
  if p['z1']<=3.2 or p['z0']>=4.97:continue
  for ring in a.part_band_footprints(p,max(3.2,p['z0']),min(4.97,p['z1'])):
   q=Polygon(ring)
   if not q.is_valid:rows.append({'tag':tag,'part':i,'role':p.get('role'),'invalid':explain_validity(q),'ring':ring,'part_keys':list(p)})
Path(__file__).with_name('🗑️generated').joinpath('parents-band-invalid-r92.json').write_text(json.dumps(rows),encoding='utf8');print(json.dumps(rows))
