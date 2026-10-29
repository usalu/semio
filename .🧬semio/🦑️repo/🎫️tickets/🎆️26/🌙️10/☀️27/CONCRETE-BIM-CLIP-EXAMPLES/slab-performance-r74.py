import importlib,time,json,hashlib
from pathlib import Path
from shapely.geometry import Polygon
from shapely.strtree import STRtree
root=Path('/repo')
base=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model'
source_hash=hashlib.sha256((base/'🔣️.json').read_bytes()).hexdigest()
t=time.monotonic()
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
m=api.load_model(); load_seconds=time.monotonic()-t
t=time.monotonic();g=api._slab_geometry(m,api.element('SL-1',m));build_seconds=time.monotonic()-t
t=time.monotonic();polygons=[Polygon(p['outline']) for p in g['full_parts']];polygon_seconds=time.monotonic()-t
t=time.monotonic();tree=STRtree(polygons);tree_seconds=time.monotonic()-t
t=time.monotonic();pairs=0;all_hits=0;max_hits=0
for i,p in enumerate(polygons):
    hits=tree.query(p);all_hits+=len(hits);max_hits=max(max_hits,len(hits));pairs+=sum(int(j)>i for j in hits)
count_seconds=time.monotonic()-t
result={'source_sha256':source_hash,'products':len(m['elements']),'slab':'SL-1','exclusive_parts':len(g['parts']),'full_parts':len(polygons),'shared_parts':len(g['shared_parts']),'candidate_pairs_j_gt_i':pairs,'all_bbox_hits_including_self':all_hits,'max_bbox_hits':max_hits,'load_seconds':load_seconds,'build_seconds':build_seconds,'polygon_seconds':polygon_seconds,'tree_seconds':tree_seconds,'count_seconds':count_seconds,'intersections_executed':0}
print(json.dumps(result))
