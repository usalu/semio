import os,sys,json
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
from lib import *
SC=os.environ["R145_SCRATCH"];state=json.load(open(SC+"/land-state.json"))
for m in state["mods"]:
    cur=read(m["path"])
    if sha(cur)==m["after"]:
        tmp=m["path"]+".r145tmp";open(tmp,"w",encoding="utf8").write(read(m["backup"]));os.replace(tmp,m["path"]);print("restored",os.path.basename(os.path.dirname(m["path"])))
    else:print("SKIP (changed by a peer)",m["path"])
for p in state["news"]:
    if os.path.exists(p):os.remove(p)
    d=os.path.dirname(p)
    try:os.rmdir(d)
    except OSError:pass
    try:os.rmdir(os.path.dirname(d)) if d.endswith("🧪️tests") else None
    except OSError:pass
print("restore done")
