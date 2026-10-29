import importlib,json
from shapely.geometry import Polygon,box
from shapely.affinity import translate
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();results={}
for tag,dy in [('FIX-LAUNDRY',.3)]:
 g=api._solid_geometry(m,api.element(tag,m));candidate=[{**p,'outline':[[x,y+dy] for x,y in p['outline']]} for p in g['parts']];hits=[]
 for e in m['elements']:
  if e['class']=='IfcColumn':parts=api.column_geometry(m,e)['parts']
  elif e['class']=='IfcWall' and e['storey']=='ground':parts=api._wall_geometry(m,e)['parts']
  else:continue
  v=sum(Polygon(p['outline']).intersection(Polygon(q['outline'],q.get('holes',[]))).area*max(0,min(p['z1'],q['z1'])-max(p['z0'],q['z0'])) for p in candidate for q in parts)
  if v>1e-10:hits.append({'tag':e['tag'],'volume':v})
 results[tag]={'candidate_y_shift':dy,'positive_column_wall_hits':hits}
guest=box(2.975,10.925,3.475,11.425);hits=[]
for e in m['elements']:
 if e['class']=='IfcColumn':parts=api.column_geometry(m,e)['parts']
 elif e['class']=='IfcWall' and e['storey']=='first':parts=api._wall_geometry(m,e)['parts']
 elif e['tag']=='FIX-GUEST-WARDROBE':parts=api._solid_geometry(m,e)['parts']
 else:continue
 area=sum(guest.intersection(Polygon(p['outline'],p.get('holes',[]))).area for p in parts if p['z0']<4.29 and p['z1']>3.8)
 if area>1e-10:hits.append({'tag':e['tag'],'area':area})
results['guest_50cm_candidate']={'bounds':list(guest.bounds),'positive_column_wall_wardrobe_hits':hits}
print(json.dumps(results))
