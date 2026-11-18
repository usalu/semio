#!/usr/bin/env python3
"""R77b landing in ONE lock hold: the source-closure bounds become one proportional family (16384 files, 64 MiB sources, 96 MiB snapshot, 32 MiB compiler output; census, discovery, memo and compiler law scaled with it), the refusal is bilingual and actionable, and the kernel canaries fire at half of the policy. Usage: land.py --dry-run | land.py --land (holds landing.lock; first records the new growth test red on the old tree, applies the stage three-way merged, runs the tests, restores every file if one is red, appends to landings.md)."""
import os,sys,subprocess,datetime,tempfile,shutil
R="/Users/ueli/Documents/semio"
H=os.path.dirname(os.path.abspath(__file__))
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";AU=L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";CL=B+"📥️source/🔏️closure/"
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/"
DP=L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/"
TESTS=[AU+"🧪️tests/📈️growth/🟦️.ts",AU+"🧪️tests/📏️capacity/🟦️.ts",DP+"🧪️tests/📶️progress/🟦️.ts",B+"🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧪️tests/🟦️.ts"]
PREFLIGHT=[SI+"🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts",SI+"🪆️heavy-questions/🧪️tests/🟦️.ts"]
GROWTH=DP+"🧪️tests/📶️progress/🟦️.ts"
S=H+"/stage"
rd=lambda p:open(p,"rb").read()
rows=[l.split(" ") for l in open(S+"/index.txt",encoding="utf8").read().splitlines() if l]
def merged(i,rel,kind):
    live=R+"/"+rel;new=rd(S+"/"+i+".new")
    if kind=="new":
        if os.path.exists(live):return (rd(live),"same") if rd(live)==new else (new,"exists")
        return new,"create"
    base=rd(S+"/"+i+".base");cur=rd(live)
    if cur==base:return new,"clean"
    if cur==new:return new,"same"
    d=tempfile.mkdtemp(prefix="r77-merge-")
    try:
        for n,b in(("live",cur),("base",base),("new",new)):open(d+"/"+n,"wb").write(b)
        r=subprocess.run(["git","merge-file","-p",d+"/live",d+"/base",d+"/new"],capture_output=True)
        return (r.stdout,"merged" if r.returncode==0 else "CONFLICT")
    finally:shutil.rmtree(d,ignore_errors=True)
mk=lambda:[(i,rel,kind)+merged(i,rel,kind) for i,rel,kind in rows]
plan=mk()
for i,rel,kind,data,how in plan:print("%-8s %s"%(how,rel[len(L):] if rel.startswith(L) else rel))
if any(h in("CONFLICT",) for *_,h in plan):sys.exit("conflicts: restage")
if "--dry-run" in sys.argv:print("dry ok",len(plan),"files");sys.exit(0)
if "--land" not in sys.argv:sys.exit("--dry-run or --land")
art=T+"/🗑️generated/R77b-art"
def tests(dirs,stop=True):
    res=[]
    for rel in dirs:
        os.makedirs(art,exist_ok=True)
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=art
        cmd=[BUN,"test",R+"/"+rel,"--timeout","600000"]
        r=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True);out=r.stdout+r.stderr
        if os.environ.get("R77_PROBE"):print(out[-3000:])
        failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0 and stop:i=out.find("error:");print(out[max(0,i-800):i+1500],flush=True);break
    return res
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R77b-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r77b.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=mk()
    if any(h=="CONFLICT" for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    before={R+"/"+rel:(rd(R+"/"+rel) if os.path.exists(R+"/"+rel) else None) for i,rel,kind,data,how in plan}
    first=[x for x in plan if "📈️growth" in x[1] or "📶️progress" in x[1]]
    for i,rel,kind,data,how in first:put(R+"/"+rel,data)
    pre=tests([GROWTH,AU+"🧪️tests/📈️growth/🟦️.ts"],stop=False)
    print("BEFORE (old tree, new tests):",pre,flush=True)
    for i,rel,kind,data,how in plan:
        p=R+"/"+rel
        if how!="same":put(p,data);applied.append(p)
    for i,rel,kind,data,how in first:
        if R+"/"+rel not in applied:applied.append(R+"/"+rel)
    if os.environ.get("R77_PROBE"):
        tests([AU+"🧪️tests/📈️growth/🟦️.ts"],stop=False)
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
        print("PROBE: files restored");sys.exit(0)
    res=tests(TESTS,stop=False);ok=all(x[1]==0 for x in res) and len(res)==len(TESTS)
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
        print("RED: files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R77b landing (%s): one declared family for the closure file ceilings, producer progress counters\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- R77b: `SourceClosureFamily` in the authorizer schema declares every file-count ceiling of the source-closure family (13 members equal to the Policy bound, 13 exempt with EN+DE reasons); the law in `AU/🧪️tests/📈️growth` walks every schema of build and caching and fails on an undeclared ceiling below the bound or a bound within a quarter of the real runtime-kernel closure. Bazel scope traces and the revision fixture follow 16384. Dependency producer: the input replay and the Source endpoint publication publish counters (`dependencies:store-input-replay:N`, `dependencies:publish-binding:N`) so the 10-minute stall rule cuts hangs, not slow phases (`DP/🧪️tests/📶️progress`). Script `🧰️scripts/R77b/land.py`.\n")
        f.write("- before (new test on old tree): %s\n"%pre)
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
