import sys
from lib import *
aid=sys.argv[1]; pid=sys.argv[2] if len(sys.argv)>2 else None
d=load(aid); a=d['asset']
pose=next(p for p in a['poses'] if p['id']==(pid or a['presentation']['defaultPose']))
W=fk(a,pose)
bb=bbox_world(a['geometry']['elevation'],W)
print(aid,pose['id'],[round(v,3) for v in bb],'root',pose.get('root'))
view=(bb[0]-0.1,bb[1]-0.05,bb[2]+0.1,bb[3]+0.05)
render(a['geometry']['elevation'],W,f"{GEN}/old-{aid}-{pose['id']}.png",50,px_per_mm=22,view=view,pen_by_role={'body':0.25,'detail':0.12,'texture':0.06,'glazing':0.1,'ground':0.2},grid=0.1)
