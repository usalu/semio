import sys; from lib import *
a=sys.argv[1]; pose=sys.argv[2]
d=load_v1(a)['asset']
d['representations']={"50":{"geometry":d['geometry'],"pens":{"body":0.25,"detail":0.18,"texture":0.13,"glazing":0.13,"ground":0.35},"hatches":[]}}
p=next(p for p in d['poses'] if p['id']==pose)
render(d,"50",p['bones'],pose_root(d,p),f"{GEN}/v1-{a}-{pose}.png",pxmm=40,crop=(-0.7,0.9,-0.05,d['dimensions']['heightM']+0.1))
print(bbox(d,"50",p['bones'],pose_root(d,p)))
