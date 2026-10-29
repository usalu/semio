import json, itertools, math
from pathlib import Path
import numpy as np
from shapely.geometry import box
from shapely.ops import unary_union
p=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES')
f=json.loads((p/'primary-beams-r12.fixture.json').read_text())
a=json.loads((p/'🗑️generated/primary-beams-r12.actual.json').read_text())
assert len(a)==f['expect']['physical_members']==31
assert len([s for s in f['specs'] if s['kind']=='flush-reference'])==7
expected=[]
for s in f['specs']:
 if s['kind']=='flush-reference': continue
 v=np.asarray(s['axis']); mid=v.mean(axis=0); w=s['width']/2
 if abs(v[1,0]-v[0,0])<abs(v[1,1]-v[0,1]): q=[mid[0]-w,v[:,1].min(),s['structural_top']-s['full_depth'],mid[0]+w,v[:,1].max(),s['soffit']]
 else:q=[v[:,0].min(),mid[1]-w,s['structural_top']-s['full_depth'],v[:,0].max(),mid[1]+w,s['soffit']]
 if 'occupied_z' in s:q[2],q[5]=s['occupied_z']
 expected.append(q)
actual=[b for m in a for b in m['boxes']]
levels=sorted(set(round(q[i],9) for q in expected+actual for i in (2,5)))
def volume(boxes):
 result=0
 for lo,hi in zip(levels,levels[1:]):
  mid=(lo+hi)/2; shapes=[box(q[0],q[1],q[3],q[4]) for q in boxes if q[2]<mid<q[5]]
  result+=unary_union(shapes).area*(hi-lo)
 return result
v=volume(expected); got=sum((q[3]-q[0])*(q[4]-q[1])*(q[5]-q[2]) for q in actual)
assert math.isclose(v,got,abs_tol=1e-7),(v,got)
for x,y in itertools.combinations(actual,2):
 assert max(0,min(x[3],y[3])-max(x[0],y[0]))*max(0,min(x[4],y[4])-max(x[1],y[1]))*max(0,min(x[5],y[5])-max(x[2],y[2]))<1e-9
for m in a:
 for q in m['boxes']:
  s=m['spec']
  if s['kind']=='upstand':assert q[2]>=s['structural_top']-1e-9 and q[5]<=3.41+1e-9
  else:assert q[5]<=s['soffit']+1e-9
print(json.dumps({'members':len(a),'parts':len(actual),'independent_shapely_union_m3':v,'assigned_parts_m3':got,'overlaps':0,'oracle':'NumPy axis/bounds + Shapely horizontal union integrated by z'}))
