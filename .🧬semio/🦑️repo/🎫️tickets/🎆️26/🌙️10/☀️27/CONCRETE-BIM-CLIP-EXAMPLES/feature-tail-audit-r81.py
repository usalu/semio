import importlib,json
from shapely.geometry import Polygon
from shapely.ops import unary_union
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();rows=[]
for tag in ['SL-1','SL-R']:
 g=api._slab_geometry(m,api.element(tag,m));root=Polygon(g['outline']).difference(unary_union([Polygon(v) for v in g['voids']]))
 rows.append({'tag':tag,'expected_root_area':root.area,'owned_area':sum(Polygon(p['outline']).area for p in g['parts']),'full_area':sum(Polygon(p['outline']).area for p in g['full_parts']),'shared_area':sum(Polygon(p['outline']).area for p in g['shared_parts']),'owned_volume':sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in g['parts']),'full_volume':sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in g['full_parts']),'quantity_volume':api.quantities(tag,m)['volume']})
print(json.dumps(rows))
