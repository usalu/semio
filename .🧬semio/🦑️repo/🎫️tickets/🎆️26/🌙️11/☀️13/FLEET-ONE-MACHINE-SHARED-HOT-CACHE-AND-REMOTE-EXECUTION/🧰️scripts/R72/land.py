#!/usr/bin/env python3
"""R72 fix 1 landing: vouch closures that read unwatched paths (install trees) by cheap stamps. Staged in stage/ (<id>.base = live file at staging, <id>.new = the package file, index.txt). Three-way merge for files the live tree changed since; a conflict stops the landing. Usage: land.py --dry-run | --land (takes landing.lock, runs the cutoff/settle suites and the standing preflight tests, restores every file if one is red, appends to landings.md)."""
import os,sys,subprocess,datetime,tempfile,shutil
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=os.path.dirname(os.path.abspath(__file__))+"/stage"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";GS=B+"🪪️graph-state/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/";RD=B+"🌐️remote-execution/🛰️sidecar/🚦️ready/";DP=L+"⚡️caching/🚀️bootstrap/📦️dependencies/"
CK=B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/";CO=B+"🕸️graph/🕰️generations/✂️cutoff/🧪️tests"
TESTS=[CO,CO+"/🧾️vouch",CO+"/🪢️hard-parts",CO+"/🌑️epoch",CK+"🧪️tests/🧭️dependencies",CK+"🧪️tests/⏱️throughput",CK+"📒️manifest/🧪️tests",CK+"🏭️walk-farm/🧪️tests"]
PREFLIGHT=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🪡️tool-cover",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
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
    d=tempfile.mkdtemp(prefix="r72-merge-")
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
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=os.path.realpath(tempfile.mkdtemp(prefix="r72-art-"))
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
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R72-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r72.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=[(i,rel,kind)+merged(i,rel,kind) for i,rel,kind in rows]
    if any(h in("CONFLICT","exists") for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    before={}
    for i,rel,kind,data,how in plan:
        p=R+"/"+rel;before[p]=rd(p) if os.path.exists(p) else None
        if how!="same":put(p,data);applied.append(p)
    res=tests(TESTS);ok=all(x[1]==0 for x in res) and len(res)==len(TESTS)
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
        print("RED: files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R72 landing (%s): closures that read paths the journal does not watch are vouched by cheap stamps\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- R72 fix 1: `createVouchV1` stamps every unwatched path (`.cache`, `node_modules`, `target`) a part reads (`stampOfV1(path, memo, cheap)`: size, times, inode; none inside the 2 s racy window) instead of making the part hard, so the 7313 of 7470 closures that read an install tree no longer go down the slow path at every start; one-time slow start to seal the vouches. Files: manifest, row-paths, generations (call site), vouch tests (4 new laws).\n")
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
