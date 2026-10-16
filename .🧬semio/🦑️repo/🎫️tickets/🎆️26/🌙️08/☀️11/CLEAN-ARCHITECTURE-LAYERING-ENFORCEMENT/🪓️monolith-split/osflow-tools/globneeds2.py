"""🔎 Provider glob needs per planned crate. usage: globneeds2.py <crate-id> <provider relative file> [artifact]"""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from families import *
from pubnames import names
from usage import toks
cid=sys.argv[1];prov=sys.argv[2]
p=FL+prov
n=names(p) if os.path.exists(p) else {}
if prov.endswith("📔️registry/🦀️.rs") or True:pass
t=set()
for f in exists(cid):
    if f.endswith("/🦀️.rs") and f.count("🧱️core")+f.count("🎞️session"):continue
    t|=toks(f)
used=sorted(k for k in n if not k.startswith("EXTGLOB") and k in t)
print(cid,"<-",prov,len(n),"names;",len(used),"used:",used)
