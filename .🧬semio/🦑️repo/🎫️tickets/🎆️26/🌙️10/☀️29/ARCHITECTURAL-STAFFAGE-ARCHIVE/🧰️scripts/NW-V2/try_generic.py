import sys,os,importlib; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
b=importlib.import_module(sys.argv[1]); tag=sys.argv[2]
SC={'50':(dict(body=.25,detail=.18,texture=.13,ground=.35),50),'100':(dict(body=.2,detail=.13,texture=.1,ground=.3),100),'200':(dict(body=.18,detail=.13,ground=.25),200),'500':(dict(body=.15,ground=.25),500)}
print('mass',b.MASS)
for k,(p,s) in SC.items():
    if len(sys.argv)>3 and k not in sys.argv[3:]: continue
    fn=getattr(b,'build_'+k,None)
    if not fn: continue
    sh=fn(); bb=bounds_world(b.B,sh); print(k,len(sh),[round(v,3) for v in bb],'w',round(bb[1]-bb[0],3),'h',round(bb[3]-bb[2],3))
    quick_svg(b.B,sh,p,s,tag+'-q'+k, mm_scale=(14 if s<=100 else 30))
