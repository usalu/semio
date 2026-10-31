import sys; sys.path.insert(0,'.')
from lib import *
a=sys.argv[1]; ids=sys.argv[2].split(',')
d=load(a)['asset']; w=pose_world(d)
for s in d['geometry']['elevation']:
    if s['id'] in ids:
        segs=shape_path_local(s); m=w[s['bone']]
        print(s['id'],s['bone'],s['kind'],' '.join(f"{c}"+"".join(f"({ap(m,x,y)[0]*100:.1f},{ap(m,x,y)[1]*100:.1f})" for x,y in ps) for c,ps in segs))
