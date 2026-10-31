"""🔗 Independently measure occupied material overlap and real contact against actual niche hosts."""
import json,hashlib,importlib,sys
from pathlib import Path
from shapely.geometry import Polygon
r=next(p for p in Path(__file__).resolve().parents if (p/'package.json').exists());sys.path.insert(0,str(r));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');sp=Path(a.__file__).with_name('🔣️.json');raw=sp.read_bytes();m=a.load_model();tags=['FIX-SERVANT-BASIN-1','FIX-SERVANT-BASIN-2','FIX-CHAUFFEUR-BASIN','FIX-SERVANT-STORAGE-1-LONG','FIX-SERVANT-BASIN-CABINET-1'];hosts=['W-G-W','W-G-SERVICE','W-G-ROOM-13','W-G-E'];objects={t:a.solid_geometry(t,m)['parts'] if a.element(t,m)['class']=='IfcFurnishingElement' else a.service_geometry(t,m)['parts'] for t in tags};objects.update({t:a._wall_geometry(m,a.element(t,m))['parts'] for t in hosts})
objects['TILE-SERVANT-BASIN-2']=a.solid_geometry('TILE-SERVANT-BASIN-2',m)['parts']
def pair(left,right,brackets=False):
 brackets=brackets or left in tags[:3]
 volume=0.;vertical=0.;horizontal=0.;nearest=None
 for p in objects[left]:
  if brackets and p.get('role')!='basin_bracket':continue
  q=Polygon(p['outline'],p.get('holes',()));
  for v in objects[right]:
   s=Polygon(v['outline'],v.get('holes',()));h=min(p['z1'],v['z1'])-max(p['z0'],v['z0']);dist=q.distance(s);dz=max(0,max(p['z0'],v['z0'])-min(p['z1'],v['z1']));dd=(dist*dist+dz*dz)**.5;nearest=dd if nearest is None else min(nearest,dd)
   if h>1e-10:
    area=q.intersection(s).area;volume+=area*h
    if area<1e-12 and dist<1e-9:vertical+=q.boundary.intersection(s.boundary.buffer(1e-9,cap_style=2)).length*h
   if abs(p['z0']-v['z1'])<1e-9 or abs(p['z1']-v['z0'])<1e-9:horizontal+=q.intersection(s).area
 return {'left':left,'right':right,'brackets_only':brackets,'positive_overlap_volume':volume,'vertical_contact_area':vertical,'horizontal_contact_area':horizontal,'nearest_distance':nearest}
rows=[pair(t,h) for t in tags for h in hosts]+[pair('FIX-SERVANT-BASIN-1','FIX-SERVANT-BASIN-CABINET-1',True),pair('FIX-SERVANT-BASIN-2','W-G-ROOM-13',True),pair('FIX-CHAUFFEUR-BASIN','W-G-E',True),pair('FIX-SERVANT-BASIN-2','TILE-SERVANT-BASIN-2',True),pair('TILE-SERVANT-BASIN-2','W-G-ROOM-13')];out={'source':hashlib.sha256(raw).hexdigest(),'source_stable':sp.read_bytes()==raw,'rows':rows,'limits':'Prismatic source parts only; exact XY/z occupied overlap;1nm boundary search for floating-point coincident contact, no source geometry alteration. Native part closure/volumes separately audited.'};Path(__file__).with_name('🗑️generated').joinpath('niche-contact-independent-r92.json').write_text(json.dumps(out));print(json.dumps(out),flush=True)
