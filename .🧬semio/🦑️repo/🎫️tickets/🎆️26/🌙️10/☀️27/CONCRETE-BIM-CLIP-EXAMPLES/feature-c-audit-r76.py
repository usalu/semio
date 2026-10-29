import importlib,json
from shapely.geometry import Polygon,box
from shapely.ops import unary_union
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model()
item=api._solid_geometry(m,api.element('FIX-LAUNDRY',m));col=api.column_geometry(m,api.element('32',m));hits=[]
for i,p in enumerate(item['parts']):
 for j,c in enumerate(col['parts']):
  lo,hi=max(p['z0'],c['z0']),min(p['z1'],c['z1'])
  if hi>lo:
   shape=Polygon(p['outline'],p.get('holes',[])).intersection(Polygon(c['outline'],c.get('holes',[])))
   if shape.area>1e-12:hits.append({'part':i,'column_part':j,'xy_area':shape.area,'z0':lo,'z1':hi,'volume':shape.area*(hi-lo),'bounds':list(shape.bounds),'outline':p['outline']})
light=api._solid_geometry(m,api.element('ROOFLIGHT-1',m));basin=api._solid_geometry(m,api.element('FIX-GUEST-BASIN',m));aperture=unary_union([Polygon(p['outline']) for p in light['parts']]);bb=unary_union([Polygon(p['outline']) for p in basin['parts']]);print(json.dumps({'laundry_hits':hits,'laundry_overlap_volume_sum':sum(h['volume'] for h in hits),'guest_light_union_area':aperture.area,'guest_basin_projected_area':bb.area,'guest_basin_outside_light':bb.difference(aperture).area,'guest_light_bounds':list(aperture.bounds),'guest_basin_bounds':list(bb.bounds)}))
