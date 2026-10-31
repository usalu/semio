"""render rep geometry (from the written catalog asset) in a given pose at print size and zoomed: view_pose.py aid pose scale [zoom]"""
import sys
from lib import *
aid,pid,den=sys.argv[1],sys.argv[2],int(sys.argv[3]); zoom=float(sys.argv[4]) if len(sys.argv)>4 else 8
a=load(aid)['asset']
pose=next(p for p in a['poses'] if p['id']==pid)
W=fk(a,pose)
r=a['representations'][str(den)]
sh=r['geometry']['elevation']
bb=bbox_world(sh,W)
view=(bb[0]-0.15,min(bb[1],0)-0.05,bb[2]+0.15,bb[3]+0.05)
pbr={k:v for k,v in r['pens'].items() if v}
render(sh,W,f"{GEN}/{aid}-{den}-{pid}-zoom.png",den,px_per_mm=11.8*zoom,view=view,pen_by_role=pbr,grid=0.1)
render(sh,W,f"{GEN}/{aid}-{den}-{pid}-print.png",den,px_per_mm=11.8,view=view,pen_by_role=pbr,grid=0)
print([round(v,3) for v in bb])
