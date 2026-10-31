"""📏 Locate actual curtain contact before choosing inferred pipe clearance."""
import importlib,sys,json
from pathlib import Path
from scipy.spatial import ConvexHull
from scipy.optimize import linprog
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');s=a.load_model();host=a.filling_geometry('CW-1',s)['parts'];out=[]
for tag in ['HEAT-RISER-VESTIBULE-5-SUPPLY','HEAT-RISER-VESTIBULE-5-RETURN','HEAT-RISER-VESTIBULE-5-MOUNTS']:
 g=a.solid_geometry(tag,s) if tag.endswith('MOUNTS') else a.service_geometry(tag,s)
 for pi,p in enumerate(g['parts']):
  pv=a.part_vertices(p);h=ConvexHull(pv)
  for hi,solid in enumerate(host):
   v=a.part_vertices(solid)
   if any(min(max(q[k] for q in v),max(q[k] for q in pv))-max(min(q[k] for q in v),min(q[k] for q in pv))<1e-8 for k in range(3)):continue
   eq=list(h.equations)+list(ConvexHull(v).equations);fit=linprog([0,0,0,-1],A_ub=[list(e[:3])+[1] for e in eq],b_ub=[-e[3] for e in eq],bounds=[(None,None)]*3+[(0,None)],method='highs')
   if fit.success and fit.x[3]>1e-8:out.append({'tag':tag,'part':pi,'host':hi,'host_role':solid.get('role'),'host_vertices':v,'witness':fit.x.tolist()})
print('[DEBUG] '+json.dumps(out),flush=True);(Path(__file__).parent/'🗑️generated/heat-clearance-r92.json').write_text(json.dumps(out),encoding='utf8')
