"""🧮 Integrate independently decoded native cap triangles as occupied unions, retaining all overlaps."""
import hashlib,importlib,json,math,time
from pathlib import Path
from shapely.geometry import Polygon
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.';overlay=importlib.import_module(prefix+'🧱️model.🧪️tests.🚗️overlay');ticket=Path(__file__).parent;generated=ticket/'🗑️generated';data=json.loads((generated/'furnishing-all-r77-decoded.json').read_text());cache={};results=[];started=time.monotonic()
for index,sample in enumerate(data['decoded']):
    key=hashlib.sha256(json.dumps(sample['parts'],sort_keys=True,separators=(',',':')).encode()).hexdigest()
    if key not in cache:
        parts=sample['parts'];levels=sorted({p[k] for p in parts for k in ('z0','z1')});cases=[];heights=[]
        for i,(lo,hi) in enumerate(zip(levels,levels[1:])):
            active=[polygon for p in parts if p['z0']<(lo+hi)/2<p['z1'] for polygon in p['polygons']]
            assert all(Polygon(polygon[0]).is_valid and Polygon(polygon[0]).area>0 for polygon in active)
            cases.append({'id':str(i),'actual':active,'expected':[]});heights.append(hi-lo)
        areas=json.loads(overlay.overlay_json(json.dumps({'cases':cases})))['cases'];cache[key]=math.fsum(area['xor_area']*height for area,height in zip(areas,heights))
    actual=cache[key];expected=sample['source_owned_volume'];bound=max(1e-7,expected*1e-5)
    assert abs(actual-expected)<bound,(sample['tag'],actual,expected,bound)
    results.append({'tag':sample['tag'],'language':sample['language'],'native_occupied_union':actual,'source_occupied_union':expected,'absolute_error':abs(actual-expected),'existing_native_volume_bound':bound,'decoded_geometry_sha256':key})
    (generated/'furnishing-all-r77-union-progress.json').write_text(json.dumps({'debug':'[DEBUG] native occupied prism unions','completed':index+1,'planned':len(data['decoded']),'tag':sample['tag'],'seconds':time.monotonic()-started}))
record={'source':data['source'],'samples':len(results),'unique_native_geometries':len(cache),'results':results,'seconds':time.monotonic()-started};(generated/'furnishing-all-r77-union.json').write_text(json.dumps(record));print({'samples':len(results),'unique_geometries':len(cache),'seconds':record['seconds'],'max_error':max(r['absolute_error'] for r in results)})
