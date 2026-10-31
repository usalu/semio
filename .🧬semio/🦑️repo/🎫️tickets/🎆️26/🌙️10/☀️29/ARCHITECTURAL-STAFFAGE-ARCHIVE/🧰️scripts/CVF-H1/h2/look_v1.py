import sys; sys.path.insert(0,'.')
from lib import *
pens={"body":0.35,"detail":0.2,"texture":0.1,"glazing":0.1,"ground":0.3}
for a in sys.argv[1:]:
    d=load(a)['asset']; w=pose_world(d)
    sh=d['geometry']['elevation']
    print(a, 'bounds', )
    to_png(render_svg(sh,w,8,pens),'v1-'+a,96)
