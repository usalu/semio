#!/usr/bin/env python3
"""R65 landing: the lean slice read (inference in short-lived children, first child always, lean parent in the memory ledger, settle before compute, doubt without a whole-run file map). Staged in stage/ (<id>.base = the live file when staged, <id>.new = the package file; index.txt lists id, path, mod|new). Three-way merge against the live files at landing time (git merge-file on temporary copies, nothing in the repository is touched). Nothing is written to the live tree before landing.lock is held. Usage: land.py --dry-run | --land. NEVER run --land while workers are in their last setup stage: every landing in the build module makes all graph owners stale."""
import os,sys,subprocess,hashlib,datetime,tempfile,shutil
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=os.path.dirname(os.path.abspath(__file__))+"/stage"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/"
CK=B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/"
TESTS=[B+"🧠️memory/🪶️lean/🧪️tests",B+"🧠️memory/🪜️plan-stage/🧪️tests",B+"🧠️memory/🪜️plan-stage/🧪️tests/🧮️tree-plan",B+"🧠️memory/✂️cut/🧪️tests",CK+"🏭️walk-farm/🧪️tests",CK+"🧪️tests/⏱️throughput",PN+"🏊️project-farm/🧪️tests/🪶️lean",PN+"🏊️project-farm/🧪️tests",B+"🏡️resident/🧪️tests/✂️cut-load",B+"🏡️resident/🧪️tests/🛡️plan-guard"]
PREFLIGHT=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
sha=lambda b:hashlib.sha256(b).hexdigest()[:12]
rows=[l.split(" ") for l in open(S+"/index.txt",encoding="utf8").read().splitlines() if l]
rd=lambda p:open(p,"rb").read()
def merged(i,rel,kind):
    live=R+"/"+B+rel;new=rd(S+"/"+i+".new")
    if kind=="new":
        if os.path.exists(live):return (rd(live),"exists") if rd(live)!=new else (new,"same")
        return new,"create"
    base=rd(S+"/"+i+".base")
    cur=rd(live)
    if cur==base:return new,"clean"
    if cur==new:return new,"same"
    d=tempfile.mkdtemp(prefix="r65-merge-")
    try:
        for n,b in(("live",cur),("base",base),("new",new)):open(d+"/"+n,"wb").write(b)
        r=subprocess.run(["git","merge-file","-p",d+"/live",d+"/base",d+"/new"],capture_output=True)
        return (r.stdout,"merged" if r.returncode==0 else "CONFLICT")
    finally:shutil.rmtree(d,ignore_errors=True)
plan=[(i,rel,kind)+merged(i,rel,kind) for i,rel,kind in rows]
for i,rel,kind,data,how in plan:print("%-8s %s"%(how,rel))
if any(h in("CONFLICT","exists") for *_,h in plan):sys.exit("conflicts: merge by hand before landing")
if "--dry-run" in sys.argv:print("dry ok",len(plan),"files");sys.exit(0)
if "--land" not in sys.argv:sys.exit("--dry-run or --land")
def tests(dirs):
    res=[]
    for rel in dirs:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=tempfile.mkdtemp(prefix="r65-art-")
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000","--bail"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        shutil.rmtree(env["SEMIO_TEST_ARTIFACT_DIR"],ignore_errors=True)
        out=r.stdout+r.stderr;failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0:i=out.find("error:");print(out[max(0,i-1500):i+1500],flush=True);break
    return res
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R65-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r65.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=[(i,rel,kind)+merged(i,rel,kind) for i,rel,kind in rows]
    if any(h in("CONFLICT","exists") for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    before={}
    for i,rel,kind,data,how in plan:
        p=R+"/"+B+rel;before[p]=rd(p) if os.path.exists(p) else None
        if how!="same":put(p,data);applied.append(p)
    res=tests(TESTS);ok=all(x[1]==0 for x in res) and len(res)==len(TESTS)
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
        print("RED: files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R65 landing (%s): lean slice read\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- new `LIB/🏗️build/🧠️memory/🪶️lean` (schema, plan, tests); edited plan-stage ledger (`leanSliceV1`, lean parent and child budget, first child always), project farm (lean policy, one child, engage at once), farm pool plan (`minimumWorkers`), project-nodes (maximumWorkers), graph host (`releaseCutoff` settles memory before compute), doubt (file map cleared per chunk); tree-plan and walk-farm tests run the old law with `SEMIO_SLICE_LEAN=off`.\n")
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
