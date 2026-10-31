"""🔬 Independently confirm large captured intersections with conditioned SciPy halfspaces."""
import hashlib,importlib.util,json,sys,time,traceback
from pathlib import Path
import numpy as np
from scipy.optimize import linprog
from scipy.spatial import ConvexHull,HalfspaceIntersection
sys.dont_write_bytecode=True
ticket=Path(__file__).parent;folder=ticket/'🗑️generated/whole-diagnostic-r95';capture=folder/'whole-expanded-owned-capture-r92.json';partial=folder/'whole-expanded-interference-partial-r92.json';data=json.loads(capture.read_text(encoding='utf8'));report=json.loads(partial.read_text(encoding='utf8'));spec=importlib.util.spec_from_file_location('pair_topology_r95',ticket/'whole-expanded-interference-r92.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);records={(r['tag'],r['index']):r for r in data['records']};wanted=[('GARDEN-WEATHER-CAP','SALON-CANOPY'),('SL-1','RAIN-KITCHEN-PATIO-9'),('GUARD-RM-1','LIGHT-HALL-BASIN-19'),('LIGHT-HALL-BASIN-19','RF-1B'),('FIN-KITCHEN-PATIO','RAIN-KITCHEN-PATIO-9'),('TAP-KITCHEN','SINK-KITCHEN-1'),('TAP-PANTRY','SINK-PANTRY')];rows=[];start=time.monotonic()
def signed_volume(vertices,faces):
    """📐 Evaluate the actual boundary around a local origin without hull replacement."""
    points=vertices-vertices.mean(axis=0);return abs(sum(float(np.dot(points[f[0]],np.cross(points[f[i]],points[f[i+1]])))/6 for f in faces for i in range(1,len(f)-1)))
for a,b in wanted:
    candidates=[r for r in report['positive_part_pairs']if(r['source']['tag'],r['target']['tag'])==(a,b)];pair=max(candidates,key=lambda r:r['volume']);left=records[(a,pair['source']['index'])];right=records[(b,pair['target']['index'])];row={'source':left['tag'],'source_index':left['index'],'target':right['tag'],'target_index':right['index'],'audit_volume':pair['volume'],'source_shape_sha256':hashlib.sha256(json.dumps(left['geometry'],sort_keys=True).encode()).hexdigest(),'target_shape_sha256':hashlib.sha256(json.dumps(right['geometry'],sort_keys=True).encode()).hexdigest()}
    try:
        assert not left['geometry'].get('holes')and not right['geometry'].get('holes'),'Hollow shape needs a separate exact decomposition; no hull substitution'
        raw=[module.topology(r['geometry'])for r in(left,right)];world=[np.asarray(v,float)for v,f in raw];allpoints=np.concatenate(world);origin=(allpoints.min(axis=0)+allpoints.max(axis=0))/2;scale=max(np.ptp(allpoints,axis=0));local=[(v-origin)/scale for v in world];hulls=[ConvexHull(v)for v in local]
        for vertices,(_,faces),hull in zip(local,raw,hulls):
            assert abs(signed_volume(vertices,faces)-hull.volume)<1e-11,'Source is not independently certified convex'
            for face in faces:
                p=vertices[face];normal=next((np.cross(p[i]-p[0],p[i+1]-p[0])for i in range(1,len(p)-1)if np.linalg.norm(np.cross(p[i]-p[0],p[i+1]-p[0]))>0),None);assert normal is not None;normal=normal/np.linalg.norm(normal);dist=(vertices-p[0])@normal;assert np.max(dist)<1e-11 or np.min(dist)>-1e-11,'A real face crosses the candidate convex hull'
        planes=np.concatenate([h.equations for h in hulls]);A=planes[:,:3];rhs=-planes[:,3];lp=linprog([0,0,0,-1],A_ub=np.column_stack([A,np.linalg.norm(A,axis=1)]),b_ub=rhs,bounds=[(None,None)]*3+[(0,None)],method='highs');assert lp.success and lp.x[3]>1e-8,'No safely interior large positive intersection';intersection=HalfspaceIntersection(planes,lp.x[:3]);volume=ConvexHull(intersection.intersections).volume*scale**3;assert abs(volume-pair['volume'])<max(1e-10,volume*1e-8),(volume,pair['volume']);row.update(confirmed=True,independent_volume=volume,witness=(origin+scale*lp.x[:3]).tolist(),conditioned_interior_radius=lp.x[3])
    except Exception:row.update(confirmed=False,error=traceback.format_exc())
    rows.append(row);print('[DEBUG] '+json.dumps(row),flush=True)
out={'source':data['source'],'scope':'Independent large-witness confirmation of a diagnostic snapshot, not current/final whole-model acceptance. Convexity verified before SciPy hull/halfspace use; all unsupported witnesses retained.','capture_sha256':hashlib.sha256(capture.read_bytes()).hexdigest(),'rows':rows,'seconds':time.monotonic()-start};(folder/'fixture-overlap-confirmation-r95.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');raise SystemExit(not all(r['confirmed']for r in rows))
