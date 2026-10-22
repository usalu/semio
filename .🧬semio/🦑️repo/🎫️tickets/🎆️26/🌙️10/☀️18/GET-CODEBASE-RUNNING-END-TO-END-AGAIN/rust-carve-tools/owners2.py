import sys,json
sys.path.insert(0,"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rust-carve-tools")
from lib import *
C=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
inp=sys.argv[1];out=sys.argv[2]
prior=json.load(open(sys.argv[3])) if len(sys.argv)>3 else {}
files=json.load(open(inp))
res={}
for f in files:
    if prior.get(f):res[f]=prior[f];continue
    o=true_owners(R+"/"+f)
    res[f]=sorted(x.replace(R+"/","") for x in o)
json.dump(res,open(out,"w"),ensure_ascii=False,indent=1)
for f,v in res.items():
    if len(v)!=1:print(f[-80:],[x[-70:] for x in v])
print("done",len(res))
