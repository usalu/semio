import importlib,json,time,hashlib
from pathlib import Path
from shapely.geometry import Polygon,mapping
from shapely.ops import unary_union
from shapely.strtree import STRtree
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
ticket=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES');out=ticket/'🗑️generated'
m=api.load_model();g=api._slab_geometry(m,api.element('SL-1',m))
expected=Polygon(g['outline']).difference(unary_union([Polygon(v) for v in g['voids']]))
polygons=[Polygon(p['outline']) for p in g['full_parts']];tree=STRtree(polygons)
(out/'slab-operation-geometry-r75.json').write_text(json.dumps({'source_sha256':hashlib.sha256((Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json')).read_bytes()).hexdigest(),'slab':'SL-1','polygons':[mapping(p) for p in polygons],'expected':mapping(expected)}),encoding='utf8')
assert abs(sum(p.area for p in polygons)-expected.area)<0.5e-6
ordinal=0;started=time.monotonic()
def marker(op,i,j=None):
    global ordinal
    ordinal+=1
    (out/'slab-operation-marker-r75.json').write_text(json.dumps({'op':op,'i':i,'j':j,'ordinal':ordinal}),encoding='utf8')
for i,polygon in enumerate(polygons):
    assert polygon.is_valid
    marker('difference',i)
    assert polygon.difference(expected).area<1e-8
    for j in tree.query(polygon):
        if j>i:
            marker('intersection',i,int(j))
            assert polygon.intersection(polygons[j]).area<1e-8
result={'operations':ordinal,'seconds':time.monotonic()-started,'parts':len(polygons),'status':'PASS-SL1-only'}
(out/'slab-operation-result-r75.json').write_text(json.dumps(result),encoding='utf8')
print(json.dumps(result))
