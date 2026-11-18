import os,sys,json
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
from lib import *
import patches as P
SC=os.environ["R145_SCRATCH"];STATE=SC+"/land-state.json"
B=LIB+"/🏗️build"
MODS=[(P.NODES,P.patch_nodes),(P.POOL,P.patch_pool)]
NEWS=[(B+"/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🐘️never-here/🟦️.ts",HERE+"/new/never-here/🟦️.ts"),(B+"/🏡️resident/🧪️tests/🚫️doomed-handoff/🟦️.ts",HERE+"/new/doomed-handoff/🟦️.ts"),(B+"/👁️observation/🧪️tests/🌊️owner-scale/🟦️.ts",HERE+"/new/owner-scale/🟦️.ts")]
plan=[]
for path,fn in MODS:
    old=read(path);new=fn(old);plan.append((path,old,new))
for path,src in NEWS:
    assert not os.path.exists(path),("exists",path)
os.makedirs(SC+"/backup",exist_ok=True)
state={"mods":[],"news":[]}
for i,(path,old,new) in enumerate(plan):
    write(SC+"/backup/%d"%i,old);state["mods"].append({"path":path,"backup":SC+"/backup/%d"%i,"before":sha(old),"after":sha(new)})
write(STATE,json.dumps(state,ensure_ascii=False,indent=1))
for path,old,new in plan:
    tmp=path+".r145tmp";open(tmp,"w",encoding="utf8").write(new);os.replace(tmp,path)
for path,src in NEWS:
    write(path,read(src));state["news"].append(path)
write(STATE,json.dumps(state,ensure_ascii=False,indent=1))
for m in state["mods"]:print(m["before"],"->",m["after"],os.path.relpath(m["path"],LIB))
print("new files",len(state["news"]))
