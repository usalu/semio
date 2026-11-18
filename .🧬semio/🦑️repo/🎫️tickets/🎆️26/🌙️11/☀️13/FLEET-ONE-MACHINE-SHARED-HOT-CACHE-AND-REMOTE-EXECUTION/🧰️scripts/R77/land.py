#!/usr/bin/env python3
"""R77 landing in ONE lock hold: the source-closure bounds become one proportional family (16384 files, 64 MiB sources, 96 MiB snapshot, 32 MiB compiler output; census, discovery, memo and compiler law scaled with it), the refusal is bilingual and actionable, and the kernel canaries fire at half of the policy. Usage: land.py --dry-run | land.py --land (holds landing.lock; first records the new growth test red on the old tree, applies the stage three-way merged, runs the tests, restores every file if one is red, appends to landings.md)."""
import os,sys,subprocess,datetime,tempfile,shutil
R="/Users/ueli/Documents/semio"
H=os.path.dirname(os.path.abspath(__file__))
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";AU=L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";CL=B+"📥️source/🔏️closure/"
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/"
TESTS=[AU+"🧪️tests/📈️growth/🟦️.ts",AU+"🧪️tests/📏️capacity/🟦️.ts",AU+"🧪️tests/🚪️entry-law/🟦️.ts",AU+"📦️snapshot/🧪️tests/🟦️.ts",CL+"🧪️tests/📏️kernels/🟦️.ts",B+"🏛️sealed/🧪️tests/📏️closure/🟦️.ts",B+"📥️inputs/🔏️revision/🧪️tests/🟦️.ts",B+"🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🟦️.ts"]
PREFLIGHT=[SI+"🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts",SI+"🪆️heavy-questions/🧪️tests/🟦️.ts"]
GROWTH=AU+"🧪️tests/📈️growth/🟦️.ts"
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
art=T+"/🗑️generated/R77-art"
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
open(lock+"/owner","w").write("R77-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r77.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=mk()
    if any(h=="CONFLICT" for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    gi=[x for x in plan if x[1]==GROWTH][0];before_growth=rd(R+"/"+GROWTH) if os.path.exists(R+"/"+GROWTH) else None
    put(R+"/"+GROWTH,gi[3]);pre=tests([GROWTH],stop=False)
    print("BEFORE (old tree, new test):",pre,flush=True)
    before={}
    for i,rel,kind,data,how in plan:
        p=R+"/"+rel;before[p]=rd(p) if os.path.exists(p) else None
        if how!="same":put(p,data);applied.append(p)
    before[R+"/"+GROWTH]=before_growth
    if R+"/"+GROWTH not in applied:applied.append(R+"/"+GROWTH)
    if os.environ.get("R77_PROBE"):
        tests([x for x in TESTS[4:5]+TESTS[7:8]],stop=False)
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
        f.write("\n## Package R77 landing (%s): source-closure bounds are one proportional family\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- R77: closure Policy 2048 files/20 MiB/32 MiB -> 16384 files/64 MiB/96 MiB (one file per 4096 bytes, snapshot 1.5x), compiler output 8 -> 32 MiB, census 65536/256 MiB, data discovery 8192, memo 32 MiB, compiler law 12288 files; bilingual actionable refusal `nxAuthorizerCapacityTextV1`; kernel canaries fire at half of the policy. New test `AU/🧪️tests/📈️growth`. Script `🧰️scripts/R77/land.py`.\n")
        f.write("- before (new test on old tree): %s\n"%pre)
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
