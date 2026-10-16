import json,sys,os
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from common import *
import defscan
F=json.load(open(SC+'osnd/neural-files.json'))
RANKS={"M":0,"R":1,"B":2,"E":3}
only=sys.argv[1:] 
for k in ["M","R","B"]:
    if only and k not in only:continue
    owner=F[k]
    others={kk:v for kk,v in F.items() if RANKS[kk]>RANKS[k]}   # only higher crates can violate privacy
    otoks={}
    for kk,v in others.items():
        for f in v:otoks[(kk,f)]=defscan.idents(REPO+f)
    print("==",k,len(owner),"files; higher crates:",list(others))
    for f in owner:
        for kind,name,line,vis in defscan.defs(REPO+f):
            if kind in("field","member") and len(name)<4:continue
            users=sorted({kk for (kk,g),s in otoks.items() if name in s})
            if users:print(f"  {kind}\t{name}\t{vis}\t{os.path.basename(os.path.dirname(f))}:{line}\tused-by {users}")
