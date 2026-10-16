import json,sys,os,subprocess
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from common import *
import defscan
F=json.load(open(SC+'osnd/host-files.json'))
order=["instance","registry","workflow","raster","codec","space"]
for k in sys.argv[1:] or order:
    owner=F[k]
    others=[f for kk,v in F.items() if kk!=k for f in v]
    otoks={}
    for f in others:otoks[f]=defscan.idents(REPO+f)
    print("==",k,len(owner),"files vs",len(others))
    for f in owner:
        for kind,name,line,vis in defscan.defs(REPO+f):
            if kind in("field","member") and len(name)<5:continue
            users=sorted({kk for kk,v in F.items() if kk!=k for g in v if name in otoks[g]})
            if users and (kind not in("field","member") or vis!="private" or True):
                print(f"  {kind}\t{name}\t{vis}\t{os.path.basename(os.path.dirname(f))}:{line}\tused-by {users}")
