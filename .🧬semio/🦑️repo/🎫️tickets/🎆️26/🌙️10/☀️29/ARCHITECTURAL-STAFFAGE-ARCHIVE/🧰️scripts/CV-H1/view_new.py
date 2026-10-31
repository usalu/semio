import sys, importlib
from lib import *
aid=sys.argv[1]; mod=importlib.import_module(sys.argv[2]); pid=sys.argv[3] if len(sys.argv)>3 else None
scales=[int(x) for x in sys.argv[4].split(',')] if len(sys.argv)>4 else [500,200,100]
a=load(aid)['asset'] if False else json.load(open(f"v1/{aid}.json"))['asset']
pose=next(p for p in a['poses'] if p['id']==(pid or a['presentation']['defaultPose']))
W=fk(a,pose)
R=mod.build(a,pose)
pens={500:0.15,200:0.18,100:0.2}
for den in scales:
    r=R[str(den)]
    sh=r['geometry']['elevation']
    bb=bbox_world(sh,W)
    print(den,[round(v,3) for v in bb])
    pen=r['pens']
    pbr={k:v for k,v in pen.items() if v}
    view=(bb[0]-0.1,bb[1]-0.05,bb[2]+0.1,bb[3]+0.05)
    render(sh,W,f"{GEN}/{aid}-{den}-{pose['id']}.png",den,px_per_mm={500:160,200:90,100:45}[den],view=view,pen_by_role=pbr,grid=0.1)
