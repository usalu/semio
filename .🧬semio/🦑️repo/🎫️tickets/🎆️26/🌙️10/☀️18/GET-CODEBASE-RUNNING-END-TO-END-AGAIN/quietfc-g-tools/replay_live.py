import json,sys,time
sys.path.insert(0,__file__.rsplit('/',1)[0])
from model import *
st=json.load(open('/Users/ueli/Library/Caches/semio/host-capacity/estimates.json'))['classes']
leaf='run:build:@teaching/architecture-course:build'
lin=['run','run:build',leaf]
for hh,mm in [(16,55),(18,15)]:
    now=time.mktime((2026,10,26,hh,mm,0,0,0,-1))*1000
    # drop samples later than now
    s={k:[r for r in v['samples'] if r['atMs']<=now] for k,v in st.items()}
    for k in lin: 
        w=weights(POL,s.get(k,[]),now); print(hh,mm,k,len(s.get(k,[])),round(sum(w)/U,2),estimate(POL,s,[k],now))
    print(hh,mm,'lineage',estimate(POL,s,lin,now),'forecast',forecast(POL,s,leaf,now))
