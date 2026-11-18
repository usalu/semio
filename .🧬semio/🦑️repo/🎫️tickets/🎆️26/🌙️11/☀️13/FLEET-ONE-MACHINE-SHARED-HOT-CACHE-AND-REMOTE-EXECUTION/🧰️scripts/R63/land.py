#!/usr/bin/env python3
"""R63 landing: one global memory plan for the process tree (plan-stage ledger, project farm and walk farm reserve from it). Staged in scratch (stage/<id>.new built from stage/<id>.base by edits.py); nothing is written to the live tree before landing.lock is held. Applies the staged files, runs the targeted tests one at a time and the standing preflight, restores the base files at the first red. Usage: land.py [--dry-run|--land]"""
import os,sys,subprocess,hashlib,datetime
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R63/stage"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/"
CK=B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/"
NEWTEST=(B+"🧠️memory/🪜️plan-stage/🧪️tests/🧮️tree-plan/🟦️.ts",S+"/treeplan.ts")
TESTS=[B+"🧠️memory/🪜️plan-stage/🧪️tests",B+"🧠️memory/🪜️plan-stage/🧪️tests/🧮️tree-plan",B+"🧠️memory/✂️cut/🧪️tests",CK+"🏭️walk-farm/🧪️tests",CK+"🧪️tests/⏱️throughput",PN+"🏊️project-farm/🧪️tests",B+"🏡️resident/🧪️tests/✂️cut-load",B+"🏡️resident/🧪️tests/🛡️plan-guard"]
PREFLIGHT=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
files=[(l.split(" ")[0],B+l.split(" ")[1],l.split(" ")[2]) for l in open(S+"/index.txt",encoding="utf8").read().splitlines()]
def tests(dirs):
    res=[]
    for rel in dirs:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R63/artifacts"
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000","--bail"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        out=r.stdout+r.stderr;failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0:i=out.find('error:');print(out[max(0,i-1500):i+1500],flush=True);break
    return res
def put(path,data):
    tmp=path+".r63.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
bad=[(rel,sha(S+"/"+i+".base"),sha(R+"/"+rel)) for i,rel,_ in files if sha(S+"/"+i+".base")!=sha(R+"/"+rel)]
if bad:sys.exit("live files changed since staging: "+repr(bad))
if "--dry-run" in sys.argv:print("dry ok",len(files),"files");sys.exit(0)
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R63-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[];created=[]
try:
    bad=[(rel,sha(S+"/"+i+".base"),sha(R+"/"+rel)) for i,rel,_ in files if sha(S+"/"+i+".base")!=sha(R+"/"+rel)]
    if bad:sys.exit("live files changed while waiting for the lock: "+repr(bad))
    for i,rel,_ in files:
        if open(S+"/"+i+".base","rb").read()!=open(S+"/"+i+".new","rb").read():
            put(R+"/"+rel,open(S+"/"+i+".new","rb").read());applied.append((i,rel))
    os.makedirs(os.path.dirname(R+"/"+NEWTEST[0]),exist_ok=True);put(R+"/"+NEWTEST[0],open(NEWTEST[1],"rb").read());created.append(NEWTEST[0])
    res=tests(TESTS)+([] if True else [])
    ok=all(x[1]==0 for x in res) and len(res)==len(TESTS)
    if ok:
        res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:
        for i,rel in applied:put(R+"/"+rel,open(S+"/"+i+".base","rb").read())
        for rel in created:os.remove(R+"/"+rel);os.rmdir(os.path.dirname(R+"/"+rel))
        print("RED: base files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R63 landing (%s): one memory plan for the process tree\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- `LIB/🏗️build/🧠️memory/🪜️plan-stage/🟨️.mjs` (ledger `reserveFarmV1`, `farmReservationsV1`, `kind` of `planFarmWorkersV1`), its `.d.mts`, schema (`treeShare` 0.4 = the plan limit share of the slice guard, `farmPriority`); walk farm (`reserveWalkFarmV1`, lease released at close, recipe farm and settler farm draw from the plan); project farm (reservation at hub creation, released at retirement); project-nodes passes the kind. Tests: new `plan-stage/🧪️tests/🧮️tree-plan`, updated plan-stage, cut and walk-farm tests.\n")
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
