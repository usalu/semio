"""🧭 Prove positive planar bearing against original structural owner faces."""
import importlib,sys,json,math,itertools
from fractions import Fraction as Q
from pathlib import Path
from shapely.geometry import Polygon
sys.path.insert(0,str(Path.cwd()))
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m=a.load_model();rows=[]
def cross(u,v):return [u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
for tag in ('RF-1A','RF-1B','RF-2A','RF-2B'):
 parts=a.ramp_geometry(tag,m)['parts'];hits=[]
 for owner in a._find(m,tag)['geometry']['interface_owners']:
  obj=a._find(m,owner)
  if obj['class']=='IfcColumn' and a.column_geometry(m,obj)['parts'][0].get('profile',{}).get('shape')=='circle':continue
  for stock in a._interface_reference_parts(m,owner):
   vs=a.part_vertices(stock)
   for face in a.part_faces(stock):
    anchor=vs[face[0]];normal=None
    for i,j in itertools.combinations(face[1:],2):
     n=cross([v-u for u,v in zip(anchor,vs[i])],[v-u for u,v in zip(anchor,vs[j])])
     if any(n):normal=n;break
    if normal is None:raise ValueError('Original owner has a degenerate face')
    offset=sum(n*v for n,v in zip(normal,anchor));axis=max(range(3),key=lambda k:abs(normal[k]));drop=lambda p:[float(p[k]) for k in range(3) if k!=axis];host=Polygon([drop(vs[i]) for i in face])
    for part in parts:
     pv=a.part_vertices(part)
     for pf in a.part_faces(part):
      coords=[pv[i] for i in pf]
      if not all(abs(sum(n*Q(v) for n,v in zip(normal,p))-offset)<=sum(abs(n)*Q(math.ulp(float(v)))/2 for n,v in zip(normal,p)) for p in coords):continue
      shape=Polygon([drop(p) for p in coords])
      if not shape.is_valid:raise ValueError('Published material face must be valid')
      area=host.intersection(shape).area
      if area>0:hits.append({'owner':owner,'projected_area':area,'projection_axis':axis})
 assert hits,tag
 rows.append({'tag':tag,'contacts':hits,'projected_area_sum':sum(h['projected_area'] for h in hits)})
 print('[DEBUG] '+json.dumps(rows[-1]),flush=True)
(Path(__file__).parent/'🗑️generated/ramp-bearing-preflight-r92.json').write_text(json.dumps(rows,indent=2),encoding='utf8')
