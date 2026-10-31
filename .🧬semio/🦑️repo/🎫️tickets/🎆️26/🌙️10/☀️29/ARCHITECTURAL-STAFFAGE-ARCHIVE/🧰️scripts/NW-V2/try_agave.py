import sys,os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import agave as b
from lib import *
reps={'50':(b.build_50(),dict(body=.25,detail=.18,texture=.13,ground=.35),50),'100':(b.build_100(),dict(body=.2,detail=.13,texture=.1,ground=.3),100),'200':(b.build_200(),dict(body=.18,detail=.13,ground=.25),200),'500':(b.build_500(),dict(body=.15,ground=.25),500)}
print('mass',b.MASS)
for k,(sh,p,s) in reps.items():
    bb=bounds_world(b.B,sh); print(k,len(sh),[round(v,3) for v in bb], 'w',round(bb[1]-bb[0],3),'h',round(bb[3]-bb[2],3))
    quick_svg(b.B,sh,p,s,'agave-q'+k, mm_scale=14 if s<=100 else 30)
