"""📍 Bind inferred original service candidates to actual native steel and floor material."""
import importlib,json,sys,math
from pathlib import Path
from shapely.geometry import Polygon,Point
from shapely.ops import unary_union
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");s=a.load_model();cw=a.filling_geometry("CW-1",s);slab=a._slab_geometry(s,a._find(s,"SL-1"));out=[]
steel=unary_union([Polygon(r) for part in cw["parts"] if part.get("material")=="MAT-METAL" and part["z0"]<1.4<part["z1"] for r in a.part_regions(part,1.4)])
glass=unary_union([Polygon(r) for part in cw["parts"] if part.get("material")=="MAT-GLASS" and part["z0"]<1.4<part["z1"] for r in a.part_regions(part,1.4)])
for name,p,n in [("RAIN9",[3.42,9.16],[-.9308352765479929,-.3654390344966194]),("HEAT5",[7.85,4.87],[.7/math.hypot(1.74,.7),1.74/math.hypot(1.74,.7)])]:
 axis=[p[k]+(.075 if name=="RAIN9" else .1)*n[k] for k in range(2)];edges=[]
 for part in cw["parts"]:
  if part.get("material")!="MAT-METAL" or not (part["z0"]<1.4 and part["z1"]>1.4):continue
  for ring in a.part_regions(part,1.4):
   for u,v in zip(ring,ring[1:]+ring[:1]):
    length=math.dist(u,v)
    if length<.014:continue
    mid=[(u[k]+v[k])/2 for k in range(2)];normal=[(v[1]-u[1])/length,-(v[0]-u[0])/length]
    for fraction in (.25,.5,.75):
     point=[u[k]+fraction*(v[k]-u[k]) for k in range(2)];tangent=[(v[k]-u[k])/length for k in range(2)]
     face_samples=[[point[k]+side*.004*tangent[k] for k in range(2)] for side in (-1,0,1)]
     exposed=all(not steel.covers(Point([q[k]+normal[k]*1e-5 for k in range(2)])) and not glass.covers(Point([q[k]+normal[k]*1e-5 for k in range(2)])) and steel.covers(Point([q[k]-normal[k]*1e-5 for k in range(2)])) for q in face_samples)
     if exposed and sum(normal[k]*n[k] for k in range(2))>.45 and math.dist(point,p)<.07:edges.append({"a":u,"b":v,"point":point,"normal":normal,"length":length,"contact_width":.008,"distance":math.dist(point,axis)})
 edges.sort(key=lambda e:e["distance"]);floors=[]
 for z in [3.,3.155]:
  floors.append({"z":z,"inside":any(Polygon(r).covers(Point(axis)) for part in slab["parts"]+slab.get("finish_parts",[]) if part["z0"]<=z<=part["z1"] for r in a.part_regions(part,z))})
 out.append({"id":name,"path_vertex":p,"normal":n,"axis":axis,"steel_faces":edges[:3],"floor":floors})
p=Path(__file__).parent/"🗑️generated"/"mep-host-probe-r92.json";p.write_text(json.dumps(out,indent=2));print("[DEBUG] "+json.dumps(out),flush=True)
      