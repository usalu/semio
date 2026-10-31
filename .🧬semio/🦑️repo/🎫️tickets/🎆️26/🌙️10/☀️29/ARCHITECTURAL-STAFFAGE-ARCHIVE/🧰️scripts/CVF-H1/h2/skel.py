import sys; sys.path.insert(0,'.')
from lib import *
a=sys.argv[1]; pose=sys.argv[2] if len(sys.argv)>2 else None
d=load(a)['asset']; w=pose_world(d,pose)
pens={"body":0.25,"detail":0.15,"texture":0.08,"glazing":0.1,"ground":0.3}
sh=d['geometry']['elevation'] if 'geometry' in d else []
bounds=(-0.5,-0.05,1.1,1.95) if len(sys.argv)<4 else tuple(map(float,sys.argv[3].split(',')))
den=float(sys.argv[4]) if len(sys.argv)>4 else 8
svg=render_svg([s for s in sh if s['role']!='texture'],w,den,pens,bounds=bounds,extra=overlay(d,w,bounds,den))
print(to_png(svg,'skel-'+a+'-'+(pose or 'default'),96))
for b in d['rig']['bones']:
    p0=ap(w[b['id']],0,0);p1=ap(w[b['id']],b['lengthM'],0)
    print(f"{b['id']:14s} {p0[0]:7.3f} {p0[1]:7.3f} -> {p1[0]:7.3f} {p1[1]:7.3f}")
for an in d['anchors']:
    p=ap(w[an['bone']],an['x'],an['y']); print('A',an['id'],round(p[0],3),round(p[1],3))
