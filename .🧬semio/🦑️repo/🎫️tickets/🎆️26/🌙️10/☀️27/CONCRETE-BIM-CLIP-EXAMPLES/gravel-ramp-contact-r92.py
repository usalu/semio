"""🪨 Independently inspect new pilotis gravel against actual low-level ramp RC stocks."""
import importlib,json,sys,time,hashlib
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
ticket=Path(__file__).parent;api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();start=time.monotonic();out={'rows':[],'numeric_boundaries':[]};stocks=api._solid_geometry(m,api._find(m,'FIN-PILOTIS-GRAVEL'),resolve_embedded=False)['parts']
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

cutters=[('RF-1A',api._ramp_reference_stock(m,api._find(m,'RF-1A'))),('BEAM-RAMP-FIRST-STRINGER',api._ramp_stringer_reference_stock(m,api._find(m,'BEAM-RAMP-FIRST-STRINGER')))]
for tag,cutter in cutters:
    relevant=[p for p in stocks if all(a[0]<b[1]and b[0]<a[1]for a,b in zip(api._material_bounds(p),api._material_bounds(cutter)))];owned,removed=api._subtract_native_material(relevant,[cutter]);volume=sum(overlap(a,cutter,tag)for a in prepared(relevant));out['rows'].append({'owner':tag,'stocks':len(relevant),'native_removed':removed,'independent_overlap':volume})
out['seconds']=time.monotonic()-start;(ticket/'🗑️generated'/'gravel-ramp-contact-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(out),flush=True)
