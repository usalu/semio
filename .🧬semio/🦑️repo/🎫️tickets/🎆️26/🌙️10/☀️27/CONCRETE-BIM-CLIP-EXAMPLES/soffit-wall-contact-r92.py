"""🔬 Independently measure exterior-ramp soffit interfaces against actual owned masonry."""
import importlib,json,sys,time,hashlib,traceback
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
ticket=Path(__file__).parent;api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');source=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';apipath=source.with_name('🐍️.py');m=api.load_model();start=time.monotonic();out={'source_before':hashlib.sha256(source.read_bytes()).hexdigest(),'api_before':hashlib.sha256(apipath.read_bytes()).hexdigest(),'rows':[],'numeric_boundaries':[],'candidates':[]}
from scipy.spatial import ConvexHull,HalfspaceIntersection,QhullError
from scipy.optimize import linprog
import numpy as np
from functools import lru_cache
@lru_cache(maxsize=16384)
def hull(vertices):
    points=np.asarray(vertices,dtype=float);return ConvexHull(points).equations

def prepared(parts):
    return [cell for part in parts for cell in api._native_material_cells(part)]

def overlap(a,b,label):
    if any(x[1]<=y[0]or y[1]<=x[0]for x,y in zip(api._material_bounds(a),api._material_bounds(b))):return 0.
    va=tuple(tuple(float(x)for x in p)for p in api.part_vertices(a));vb=tuple(tuple(float(x)for x in p)for p in api.part_vertices(b));planes=np.vstack([hull(va),hull(vb)]);normals=planes[:,:3];offsets=planes[:,3];norm=np.linalg.norm(normals,axis=1)
    result=linprog([0,0,0,-1],A_ub=np.column_stack([normals,norm]),b_ub=-offsets,bounds=[(None,None)]*4,method='highs')
    if not result.success:
        if result.status!=2:out['numeric_boundaries'].append({'label':label,'solver':result.message})
        return 0.
    if result.x[3]<=0:return 0.
    try:return float(ConvexHull(HalfspaceIntersection(planes,result.x[:3]).intersections).volume)
    except QhullError as error:
        out['numeric_boundaries'].append({'label':label,'radius':float(result.x[3]),'solver':str(error).splitlines()[0]});return 0.

try:
    for cover,wall in [('FIN-SOFFIT-RF-2A','W-TERRACE-RAMP'),('FIN-SOFFIT-RF-2B','W-PARENT-BATH')]:
        print('[DEBUG] Soffit contact '+cover+' / '+wall,flush=True)
        stocks=api._solid_geometry(m,api._find(m,cover),resolve_embedded=False)['parts'];other=api._wall_geometry(m,api._find(m,wall))['parts'];a,b=prepared(stocks),prepared(other)
        rows=[{'part':i,'role':p.get('role'),'intersection':sum(overlap(cell,cut,cover+'/'+wall)for cut in b)}for i,(p,cell)in enumerate(zip(stocks,a))]
        out['rows'].append({'cover':cover,'wall':wall,'parts':len(stocks),'wall_parts':len(other),'contacts':rows,'total':sum(r['intersection']for r in rows)})
        candidate,removed=api._subtract_native_material(stocks,other)
        for part in candidate:api._polyhedron_part(api.part_vertices(part),api.part_faces(part))
        post=sum(overlap(cell,cut,cover+'/'+wall+'/after')for cell in prepared(candidate)for cut in b)
        out['rows'][-1].update(native_removed=removed,remaining_overlap=post,candidate_parts=len(candidate))
        out['candidates'].append({'cover':cover,'parts':candidate,'world':True})
    out['pass']=True
except Exception:out['pass']=False;out['error']=traceback.format_exc();print(out['error'],flush=True)
out.update(seconds=time.monotonic()-start,source_after=hashlib.sha256(source.read_bytes()).hexdigest(),api_after=hashlib.sha256(apipath.read_bytes()).hexdigest());(ticket/'🗑️generated'/'soffit-wall-contact-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps({k:v for k,v in out.items()if k!='candidates'},indent=2));raise SystemExit(not out['pass'])
