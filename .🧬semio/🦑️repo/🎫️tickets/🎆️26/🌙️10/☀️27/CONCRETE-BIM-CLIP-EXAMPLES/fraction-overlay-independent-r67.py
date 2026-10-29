import importlib,json,time,math
from pathlib import Path
from shapely.geometry import Polygon,box,mapping
from shapely.ops import unary_union
p=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES');backend=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚗️overlay')
def coords(poly):return [[list(p) for p in poly.exterior.coords]]+[[list(p) for p in r.coords] for r in poly.interiors]
hole=Polygon([(0,0),(3,0),(3,3),(0,3)],holes=[[(1,1),(2,1),(2,2),(1,2)]]);reverse=Polygon(list(hole.exterior.coords)[::-1],holes=[list(r.coords)[::-1] for r in hole.interiors])
cases=[('overlap',[box(0,0,2,2),box(1,0,3,2)],[box(0,0,3,2)]),('holes-orientation',[hole],[reverse]),('hole-missing',[hole],[box(0,0,3,3)]),('vertical-edges',[box(0,0,1,2)],[box(0,0,1,1)]),('tangent-edge',[box(0,0,1,1)],[box(1,0,2,1)]),('tangent-vertex',[box(0,0,1,1)],[box(1,1,2,2)]),('t-junction',[box(0,0,2,1),box(0,1,1,2),box(1,1,2,2)],[box(0,0,2,2)]),('collinear-overlap',[box(-2,-1,1,1),box(0,-1,2,1)],[box(-2,-1,2,1)]),('expected-only',[],[box(-2.5,-1.25,-.5,.75)]),('both-empty',[],[]),('tiny-positive',[box(0,0,1e-9,1e-9)],[]),('crossing-triangles',[Polygon([(0,0),(2,0),(1,2)])],[Polygon([(0,1),(2,1),(1,-1)])])]
rows=[]
for name,a,b in cases:
 req={'cases':[{'id':name,'actual':[coords(x) for x in a],'expected':[coords(x) for x in b]}]};r=json.loads(backend.overlay_json(json.dumps(req)));value=r['cases'][0]['xor_area'];oracle=unary_union(a).symmetric_difference(unary_union(b)).area;assert abs(value-oracle)<1e-12,(name,value,oracle)
 if name=='tiny-positive':assert value>0
 rows.append({'id':name,'fraction_area':value,'geos_oracle':oracle})
record=json.loads((p/'🗑️generated/overlay-case-error-r65.json').read_text())['isolated_errors'][0];a=[Polygon(x[0],x[1:]) for x in record['actual']];b=[Polygon(x[0],x[1:]) for x in record['expected']];started=time.monotonic();annular=json.loads(backend.overlay_json(json.dumps({'cases':[{'id':'annular','actual':record['actual'],'expected':record['expected']},{'id':'annular-area','actual':record['actual'],'expected':[]}]})));seconds=time.monotonic()-started;area=annular['cases'][1]['xor_area'];simple_area=sum(x.area for x in a);assert abs(area-simple_area)<1e-12;assert annular['cases'][0]['xor_area']<1e-8
result={'portable':rows,'annular':annular,'annular_simple_geos_polygon_area':simple_area,'annular_seconds':seconds};(p/'🗑️generated/fraction-overlay-independent-r67.json').write_text(json.dumps(result));print(json.dumps(result))
