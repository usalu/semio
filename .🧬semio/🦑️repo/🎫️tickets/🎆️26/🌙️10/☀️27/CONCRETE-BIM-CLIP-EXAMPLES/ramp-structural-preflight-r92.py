"""🏛️ Independently validate owned ramp mass and unchanged analytic structural interfaces."""
import importlib,sys,json,time,hashlib
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull,HalfspaceIntersection
from scipy.optimize import linprog
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");o=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️interfaces");start=time.monotonic();m=a.load_model();rows=[]
for tag in ('RF-1A','RF-1B','RF-2A','RF-2B'):
 obj=a._find(m,tag);g=a.ramp_geometry(tag,m);raw=a._ramp_reference_stock(m,obj);cutters=[a._ramp_reference_stock(m,a._find(m,k)) for k in obj['geometry'].get('flight_joint_owners',[])]+[p for k in obj['geometry'].get('interface_owners',[]) for p in a._interface_reference_parts(m,k)];knots=[];v=np.asarray(a.part_vertices(raw),dtype=float);ph=ConvexHull(v)
 for q in cutters:
  w=np.asarray(a.part_vertices(q),dtype=float)
  if np.any(np.minimum(v.max(0),w.max(0))<=np.maximum(v.min(0),w.min(0))):continue
  planes=np.concatenate([ph.equations,ConvexHull(w).equations]);fit=linprog([0,0,0,-1],A_ub=np.column_stack([planes[:,:3],np.ones(len(planes))]),b_ub=-planes[:,3],bounds=[(None,None)]*3+[(0,None)],method='highs')
  if fit.success and fit.x[3]>0:
   try:knots.extend(p[2] for p in HalfspaceIntersection(planes,fit.x[:3]).intersections if np.all(np.isfinite(p)))
   except Exception:pass
 base=ph.volume;removed=o.common_union_volume(raw,cutters,knots);actual=sum(ConvexHull(np.asarray(a.part_vertices(p),dtype=float)).volume for p in g['parts']);expected=base-removed
 assert abs(actual-expected)<1e-9,(tag,actual,expected,removed)
 rows.append({'tag':tag,'raw':base,'expected_removed':removed,'expected':expected,'scipy_owned':actual,'parts':len(g['parts']),'independent_knots':len(knots)});print('[DEBUG] '+json.dumps(rows[-1]),flush=True)
o.test_ramp_structural_interfaces_are_exclusive_with_analytic_round_owners();print('[DEBUG] analytic circle and actual planar structural clearance PASS',flush=True)
(Path(__file__).parent/'🗑️generated/ramp-structural-preflight-r92.json').write_text(json.dumps({'rows':rows,'seconds':time.monotonic()-start,'source':hashlib.sha256(Path(a.__file__).with_name('🔣️.json').read_bytes()).hexdigest()},indent=2),encoding='utf8')
