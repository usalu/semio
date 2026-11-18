import os,sys,json,importlib.util
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
from lib import *
def load(name):
    s=importlib.util.spec_from_file_location(name,HERE+"/"+name+".py");m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
SC=os.environ["R140_SCRATCH"];STATE=SC+"/land-state.json"
pa,pd,pl,ptx,pt,pr=[load(n) for n in ["patch-admission","patch-discovery","patch-load","patch-texts","patch-tests","patch-restart"]]
MODS=[(pa.PATH,pa.patch),(pd.PATH,pd.patch),(pl.PATH,pl.patch),(ptx.PATH,ptx.patch),(pt.OBS,pt.patch_observation),(pt.RG,pt.patch_requested),(pt.ENV,pt.patch_environment),(pr.PROCESS,pr.patch_process),(pr.POOL,pr.patch_pool),(pr.SERVICE,pr.patch_service),(pr.SCHEMA,pr.patch_schema)]
B=LIB+"/🏗️build"
NEWS=[(B+"/🕸️graph/🚪️admission/🧪️tests/🧭️source-owners/🟦️.ts",HERE+"/new/source-owners/🟦️.ts"),(B+"/🕸️graph/🚪️admission/🧪️tests/🧭️source-owners/🐍️.py",HERE+"/new/source-owners/🐍️.py"),(B+"/🕸️graph/🚪️admission/🧪️tests/❓️unknown-input/🟦️.ts",HERE+"/new/unknown-input/🟦️.ts"),(B+"/🕸️graph/📥️load/🧠️admission-memo/🟦️.ts",HERE+"/new/admission-memo/module.ts"),(B+"/🕸️graph/📥️load/🧠️admission-memo/🧪️tests/🟦️.ts",HERE+"/new/admission-memo/🟦️.ts"),(B+"/🏡️resident/🧪️tests/🗣️restart-cause/🟦️.ts",HERE+"/new/restart-cause/🟦️.ts")]
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
    tmp=path+".r140tmp";open(tmp,"w",encoding="utf8").write(new);os.replace(tmp,path)
for path,src in NEWS:
    write(path,read(src));state["news"].append(path)
write(STATE,json.dumps(state,ensure_ascii=False,indent=1))
for m in state["mods"]:print(m["before"],"->",m["after"],os.path.relpath(m["path"],LIB))
print("new files",len(state["news"]))
