"""🧱 Independently measure actual wall interfaces before publishing explicit RC owners."""
import importlib,json,sys,time,hashlib,traceback,math
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');ticket=Path(__file__).parent;sourcepath=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';apipath=sourcepath.with_name('🐍️.py');m=api.load_model();scope=sys.argv[1]if len(sys.argv)>1 else'all';start=time.monotonic();out={'source_before':hashlib.sha256(sourcepath.read_bytes()).hexdigest(),'api_before':hashlib.sha256(apipath.read_bytes()).hexdigest(),'rows':[],'qualifying_owners':{},'numeric_boundaries':[]}
specs={'W-G-GARAGE':['GUARD-RM-1','RF-1A'],'W-G-GARAGE-LOWER':['GUARD-RM-1','RF-1A'],'W-PARENT-BATH':['GUARD-RM-2','RF-2B'],'W-TERRACE-RAMP':['GUARD-RM-2','RF-2A']}
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
    guards={tag:api._find(m,tag)for tag in ('GUARD-RM-1','GUARD-RM-2')};out['guard_meshes']={tag:sum('mesh'in p for p in e['geometry']['parts'])for tag,e in guards.items()}
    if scope=='flights':specs={tag:[owner for owner in owners if not owner.startswith('GUARD-')]for tag,owners in specs.items()}
    if scope!='flights' and not all(out['guard_meshes'].values()):
        out['status']='WAIT_CONTINUOUS_GUARD_SOURCE';raise InterruptedError('Continuous guard source publication is pending')
    baselines={}
    for walltag,owners in specs.items():
        print('[DEBUG] Actual wall baseline '+walltag,flush=True);wall=api._find(m,walltag);g=api._wall_geometry(m,wall);baselines[walltag]=g;cells=prepared(g['parts']);qualified=[]
        for owner in owners:
            print('[DEBUG] Independent 3D '+walltag+' / '+owner,flush=True);cutters=api._interface_owner_parts(m,owner);remaining,removed=api._subtract_native_material(g['parts'],cutters);other=prepared(cutters);oracle=sum(overlap(a,b,walltag+'/'+owner)for a in cells for b in other)
            row={'wall':walltag,'owner':owner,'wall_parts':len(g['parts']),'owner_parts':len(cutters),'native_removed_volume':removed,'independent_intersection_volume':oracle,'agreement':math.isclose(removed,oracle,rel_tol=1e-6,abs_tol=1e-9)};out['rows'].append(row)
            if removed>1e-8 and oracle>1e-8:
                assert row['agreement'],row;qualified.append(owner)
        out['qualifying_owners'][walltag]=qualified
        if qualified:
            candidate={**wall,'geometry':{**wall['geometry'],'interface_owners':qualified}};owned,removed=api._wall_interface_parts(m,candidate,g['parts']);volume=sum(api._polyhedron_volume(p)for p in owned);out.setdefault('combined',[]).append({'wall':walltag,'before_volume':g['solid_volume'],'remaining_volume':volume,'removed_volume':removed,'volume_conserved':math.isclose(g['solid_volume'],volume+removed,rel_tol=1e-8,abs_tol=1e-9),'parts':len(owned),'references_preserved':len(g['parts'])});assert out['combined'][-1]['volume_conserved']
    out['status']='POSITIVE_OWNER_MAPPING_PROVEN';out['pass']=True
except InterruptedError as error:out['error']=str(error);out['pass']=False
except Exception:out['status']='ERROR';out['error']=traceback.format_exc();out['pass']=False;print(out['error'],flush=True)
out.update(seconds=time.monotonic()-start,source_after=hashlib.sha256(sourcepath.read_bytes()).hexdigest(),api_after=hashlib.sha256(apipath.read_bytes()).hexdigest());out['stable_source']=out['source_before']==out['source_after'];out['stable_api']=out['api_before']==out['api_after'];(ticket/'🗑️generated'/('wall-interface-actual-flights-r92.json'if scope=='flights'else'wall-interface-actual-r92.json')).write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps(out,indent=2));raise SystemExit(0 if out['pass']else 2 if out['status']=='WAIT_CONTINUOUS_GUARD_SOURCE'else 1)
