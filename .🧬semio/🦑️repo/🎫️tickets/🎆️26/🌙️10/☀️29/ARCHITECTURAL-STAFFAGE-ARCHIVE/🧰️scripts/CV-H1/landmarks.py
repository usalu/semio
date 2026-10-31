import sys
from lib import *
aid=sys.argv[1]; pid=sys.argv[2] if len(sys.argv)>2 else None
a=load(aid)['asset']
pose=next(p for p in a['poses'] if p['id']==(pid or a['presentation']['defaultPose']))
W=fk(a,pose)
for b in a['rig']['bones']:
    o=apply(W[b['id']],(0,0)); e=apply(W[b['id']],(b['lengthM'],0))
    print(f"{b['id']:12s} joint ({o[0]:.3f},{o[1]:.3f}) end ({e[0]:.3f},{e[1]:.3f})")
print()
for s in a['geometry']['elevation']:
    if s['role']!='body': continue
    bb=bbox_world([s],W)
    print(f"{s['id']:16s} {s['bone']:10s} z{s['z']} x[{bb[0]:.3f},{bb[2]:.3f}] y[{bb[1]:.3f},{bb[3]:.3f}] {s.get('region')}")
