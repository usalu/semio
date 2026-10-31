"""🧬 Retain the actual thin paint triangle instead of silently discarding positive material."""
from pathlib import Path
import hashlib,importlib.util,json,sys,time
import numpy as np
T=Path(__file__).resolve().parent
R=next(p for p in T.parents if (p/'🧰️framework').is_dir())
B=R/'🎓️teaching/🏛️architecture/🏢️nakagin'
P=T/'pixel-nearest-surface.py'
paths=[P,T/'pixel-plane-lineage-cases.json',T/'pixel-plane-lineage.schema.json',T/'pixel-ray-cases.json',Path(__file__),B/'🧪️tests/🚶️circulation/🐍️.py',B/'🧪️cases/🚶️circulation/🔣️.json',B/'🐍️.py',B/'🔣️.json',B/'🧬️schema/🔣️.schema.json']
hashes=lambda:{str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes();start=time.monotonic()
spec=importlib.util.spec_from_file_location('paint_lineage_oracle',P);M=importlib.util.module_from_spec(spec);sys.modules[spec.name]=M;spec.loader.exec_module(M)
c=json.loads((T/'pixel-plane-lineage-cases.json').read_text());base=json.loads((T/'pixel-ray-cases.json').read_text());items=base['cases'][0]['items']
parent=c['parent'];item=dict(items[0],index=2,key='original-carrier',removed=False,normal=parent['normal'],world_vertices=parent['vertices'],pixel_polygons=[[[x,y] for x,y,z in parent['vertices']]])
scene=M.SurfaceScene([*items,item],base['towards'])
row={'stage':'culled','rank':0,'piece':0,'source':2,'polygon':0,'world_vertices':c['fragment']['vertices'],'normal':c['fragment']['normal']}
vertices=np.asarray(c['fragment']['vertices'],float)
independent_area=float(np.linalg.norm(np.cross(vertices[1]-vertices[0],vertices[2]-vertices[0]))/2)
assert independent_area>0
error=None;count=None;kind=None;depth_error=None
try:
    paint=M.FragmentPaint(scene,[row]);count=len(paint.triangles);kind='silently-dropped-positive-material' if count==0 else 'retained-material'
    assert count==c['expected_triangles'],f'Expected one positive triangle, got {count}'
    point=np.asarray(c['fragment']['vertices']).mean(axis=0)[:2]
    hit=paint.contributors(paint.intersections([point])[0],'culled')
    assert hit['visible_ids']==[2],hit
    original=np.asarray(c['parent']['vertices'],float)
    _,_,vh=np.linalg.svd(original-original.mean(axis=0))
    plane=vh[-1]
    basepoint=(point-scene.offset)@scene.inverse.T
    expected=float(plane@(original[0]-basepoint)/(plane@scene.towards))
    depth_error=abs(float(np.nanmax(paint.intersections([point])))-expected)
    assert depth_error<=64*np.spacing(float(np.abs(original).max()))
except Exception as exc:
    error=repr(exc)
    if kind is None:kind='explicit-rejection'
after=hashes();result={'success':error is None,'kind':kind,'triangles':count,'independent_triangle_area':independent_area,'carrier_depth_error':depth_error,'error':error,'elapsed':time.monotonic()-start,'python':sys.version,'numpy':np.__version__,'before':before,'after':after,'unchanged':before==after}
out=T/'🗑️generated/paint-lineage';out.mkdir(exist_ok=True)
(out/(sys.argv[1]+'.json')).write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] Paint lineage '+json.dumps(result),flush=True)
raise SystemExit(0 if result['success'] and result['unchanged'] else 1)
