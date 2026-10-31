"""🔎 Compare a raw warped plane with its canonical actual facets."""
import importlib,json,sys
from pathlib import Path
sys.path.insert(0,str(next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())))
import numpy as np
V=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
quad=np.array([[0.,0.,0.],[2.,0.,0.],[2.,2.,1.],[0.,2.,0.]])
patch=np.array([[.4,.4,.1],[.6,.4,.1],[.6,.6,.1],[.4,.6,.1]])
n=np.cross(quad,np.roll(quad,-1,axis=0)).sum(axis=0);n/=np.linalg.norm(n)
def item(p,n):
    return V.Item(0,p.mean(axis=0),p,lambda v:None,n,(p,))
a,b=item(quad,n),item(patch,np.array([0.,0.,1.]))
basis=np.array([[1.,0.,0.],[0.,1.,0.]])
towards=np.array([0.,0.,1.])
project=lambda p:[V._prepare_polygon(p@basis.T)]
order=V._overlap_plane_order(a,b,project(quad),project(patch),basis,towards)
facets=V._polygon_facets(quad)
centre=np.array([.5,.5]); depths=[]
for f in facets:
    weights=np.linalg.solve(np.vstack([f[:,:2].T,np.ones(3)]),np.r_[centre,1.])
    if np.min(weights)>=0:depths.append(float(weights@f[:,2]))
raw=float((n@quad[0]-(basis@n)@centre)/(n@towards))
result={'raw_order':order,'raw_quad_depth':raw,'patch_depth':.1,'actual_quad_depths_at_patch_centre':depths,'canonical_facets':[f.tolist() for f in facets],'raw_normal':n.tolist(),'patch_xy_bounds':[[.4,.4],[.6,.6]],'wrong_uniform_order':bool(raw>.1 and max(depths)<.1)}
assert result['wrong_uniform_order'] and order!=0
out=Path(__file__).parent/'🗑️generated'/'warped-order-counterexample.json';out.write_text(json.dumps(result,indent=2),encoding='utf8')
print(json.dumps(result))
