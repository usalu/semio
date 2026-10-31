"""🔬 Capture exact opposing-flight fragment oracle inputs without a whole-building query."""
import importlib,sys,json
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull,HalfspaceIntersection
from scipy.optimize import linprog
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");f=a._surface_finishes();oracle=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️interfaces');m=a.load_model();parts=[p for tag in ('RF-1A','RF-1B','RF-2A','RF-2B') for p in a.ramp_geometry(tag,m)['parts']];rows=[]
for i,p in enumerate(parts):
 for j,q in enumerate(parts[:i]):
  v,w=np.asarray(a.part_vertices(p)),np.asarray(a.part_vertices(q));planes=np.concatenate([ConvexHull(v).equations,ConvexHull(w).equations]);fit=linprog([0,0,0,-1],A_ub=np.column_stack([planes[:,:3],np.ones(len(planes))]),b_ub=-planes[:,3],bounds=[(None,None)]*3+[(0,None)],method='highs')
  if fit.success and fit.x[3]>1e-10:
   common=HalfspaceIntersection(planes,fit.x[:3]).intersections;volume=ConvexHull(common).volume
   if volume>1e-10:
    row={'i':i,'j':j,'owner':[p['owner'],q['owner']],'volume':volume,'fit':fit.x.tolist(),'fit_residual':(planes[:,:3]@fit.x[:3]+planes[:,3]).tolist(),'vertex_max_residual':float(np.max(common@planes[:,:3].T+planes[:,3])),'independent_section_integral':oracle.common_volume(p,q),'parts':[p,q]};rows.append(row);print('[DEBUG] '+json.dumps({k:v for k,v in row.items() if k!='parts'}),flush=True)
(Path(__file__).parent/'🗑️generated/ramp-joint-probe-r92.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] fragments '+str(len(parts)),flush=True)
