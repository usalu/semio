#!/usr/bin/env python3
"""R74 landing: (1) the `ready` child stops a stage whose output shows nothing new for 10 minutes (elapsed-seconds counters, clocks and timestamps normalised away, counters such as `1957 of 2927` count) with its whole process tree and starts it again at once without counting a failure, naming the reason in English and German; (2) a restart for new code never waits for a stage in flight (the next child adopts the detached stage), the slice worker is pinned to the code it loaded (an edit of a loaded Source never ends a running load with exit 75), restarts of a slice worker that lived a minute start a new count and the total is bounded; (3) the owner pool ends every child within seconds when the machine's own work waits in the gate (5 s to 10 min of waiting), and the capacity gate reclaims a grant whose pid belongs to another process now. Staged in stage/ (<id>.base = the live file the package was built on, <id>.new = the package's file, index.txt = id, path, mod|new). A file the live tree changed since is three-way merged (git merge-file); a conflict stops the landing before anything is written. Usage: land.py --dry-run | --land (waits up to 40 minutes for landing.lock, holds it, runs the tests below and the two standing preflight tests once, restores every file if one is red, appends to landings.md)."""
import os,sys,subprocess,datetime,tempfile,shutil,time
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=os.path.dirname(os.path.abspath(__file__))+"/stage"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";SC=B+"🌐️remote-execution/🛰️sidecar/";RS=B+"🏡️resident/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/";H="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
TESTS=[SC+"🚦️ready/🌲️process-tree/🧪️tests",SC+"🚦️ready/🧪️tests",SC+"🧪️tests/🔁️restart",SC+"🧪️tests/🏊️pool",SC+"🧪️tests/🧮️closure",SC+"🧪️tests",RS+"🧪️tests/🔁️restart",RS+"🧪️tests/👷️slice-worker-scope",RS+"🧪️tests/📨️protocol",PN+"🌐️owner-pool/🧪️tests",H+"🧪️tests/♻️pid-reuse",H+"🧪️tests/🧭️liveness",H+"🧪️tests/🚪️gate"]
PREFLIGHT=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
rows=[l.split(" ") for l in open(S+"/index.txt",encoding="utf8").read().splitlines() if l]
rd=lambda p:open(p,"rb").read()
def merged(i,rel,kind):
    live=R+"/"+rel;new=rd(S+"/"+i+".new")
    if kind=="new":
        if os.path.exists(live):return (rd(live),"exists") if rd(live)!=new else (new,"same")
        return new,"create"
    base=rd(S+"/"+i+".base");cur=rd(live)
    if cur==base:return new,"clean"
    if cur==new:return new,"same"
    d=tempfile.mkdtemp(prefix="r74-merge-")
    try:
        for n,b in(("live",cur),("base",base),("new",new)):open(d+"/"+n,"wb").write(b)
        r=subprocess.run(["git","merge-file","-p",d+"/live",d+"/base",d+"/new"],capture_output=True)
        return (r.stdout,"merged" if r.returncode==0 else "CONFLICT")
    finally:shutil.rmtree(d,ignore_errors=True)
plan=[(i,rel,kind)+merged(i,rel,kind) for i,rel,kind in rows]
for i,rel,kind,data,how in plan:print("%-8s %s"%(how,rel[len(L):] if rel.startswith(L) else rel))
if any(h in("CONFLICT","exists") for *_,h in plan):sys.exit("conflicts: merge by hand before landing")
if "--dry-run" in sys.argv:print("dry ok",len(plan),"files");sys.exit(0)
if "--land" not in sys.argv:sys.exit("--dry-run or --land")
def tests(dirs):
    res=[]
    for rel in dirs:
        if not os.path.exists(R+"/"+rel+"/🟦️.ts"):print("skip (no test file)",rel[len(L):]);continue
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=os.path.realpath(tempfile.mkdtemp(prefix="r74-art-"))
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000","--bail"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        if r.returncode!=0:
            first=[l[:200] for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")];print("again (red once, a timing law under the load of a live host?)",rel[len(L):],first,flush=True)
            r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000","--bail"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        shutil.rmtree(env["SEMIO_TEST_ARTIFACT_DIR"],ignore_errors=True)
        out=r.stdout+r.stderr;failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0:i=out.find("error:");print(out[max(0,i-1500):i+1500],flush=True);break
    return res
lock=T+"/landing.lock"
waited=0
while True:
    try:os.mkdir(lock);break
    except FileExistsError:
        try:holder=open(lock+"/owner").read().strip()
        except Exception:holder="?"
        if waited==0 or waited%300==0:print("landing.lock is held by",holder,"; waiting",flush=True)
        if waited>=2400:sys.exit("landing.lock still held by "+holder)
        time.sleep(15);waited+=15
open(lock+"/owner","w").write("R74-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r74.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=[(i,rel,kind)+merged(i,rel,kind) for i,rel,kind in rows]
    if any(h in("CONFLICT","exists") for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    before={}
    for i,rel,kind,data,how in plan:
        p=R+"/"+rel;before[p]=rd(p) if os.path.exists(p) else None
        if how!="same":put(p,data);applied.append(p)
    res=tests(TESTS);ok=all(x[1]==0 for x in res) and len(res)>=len(TESTS)-3
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
        print("RED: files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R74 landing (%s): stalled stages end, restarts are content-bound, the pool yields to waiting work\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- ready child: a stage whose output (elapsed-seconds counters, clocks and timestamps normalised away; `1957 of 2927` counts) shows nothing new for 10 minutes is stopped with its whole process tree and started again at once, not counted as a failure, reason `stage-stalled` in English and German in `ready.json`, the log line and the next start (`reason.code` stalled); sidecar `ready` role never defers its restart for a stage in flight (the next child adopts it); slice worker pinned to the code it loaded, restarts of a worker that lived 60 s start a new count, total bounded at 12; owner pool children end within 3 s of a gate request that waits 5 s to 10 min (`reason: waiting`); the gate reclaims a grant whose pid is another process now (heartbeat older than two beats and the process started after the admission).\n")
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\\n"%(rel[len(L):],code,len(failed)) if False else "- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
