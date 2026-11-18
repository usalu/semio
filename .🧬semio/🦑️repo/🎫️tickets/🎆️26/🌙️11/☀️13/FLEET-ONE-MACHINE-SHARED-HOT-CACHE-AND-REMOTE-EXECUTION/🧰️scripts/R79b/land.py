#!/usr/bin/env python3
"""R79b landing 1 in ONE lock hold: exact physical identity tokens in the protected maintenance request (Windows NTFS/ReFS ids above 2^53), SQLite open failures explained with path, errno probe and free space in English and German, missing parents created once. Usage: land.py --dry-run | land.py --land (holds landing.lock; records the new laws red on the old tree, applies the stage three-way merged, type-checks against the baseline, runs the tests and the standing preflights, restores every file if one is red, appends to landings.md)."""
import os,sys,subprocess,datetime,tempfile,shutil
R="/Users/ueli/Documents/semio"
H=os.path.dirname(os.path.abspath(__file__))
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";AU=L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";CL=B+"📥️source/🔏️closure/"
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/"
OBS=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/📸️observation/"
SQL="🧰️framework/🔨️modules/⚙️runtime/🗃️sqlite/"
GROWTH=OBS+"🧪️tests/🪪️exact-identity/🟦️.ts"
TESTS=[GROWTH,SQL+"🧪️tests/🩺️open-failure/🟦️.ts"]
PREFLIGHT=[SI+"🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts",SI+"🪆️heavy-questions/🧪️tests/🟦️.ts"]
TSC="/Users/ueli/.cache/c6repo2/typescript/bin/tsc"
TSCONFIG=os.path.expanduser("/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R79b/tsconfig.json")
def tscset():
    r=subprocess.run(["node",TSC,"-p",TSCONFIG],capture_output=True,text=True,cwd=R)
    import re
    return set(re.sub(r"\([0-9,]*\)","",l) for l in (r.stdout+r.stderr).splitlines() if l and not any(c in l for c in("TS2307","TS2591","TS2580","ImportMeta")))
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
    d=tempfile.mkdtemp(prefix="r79b-merge-")
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
art=T+"/🗑️generated/R79b-art"
def tests(dirs,stop=True):
    res=[]
    for rel in dirs:
        os.makedirs(art,exist_ok=True)
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=art
        cmd=[BUN,"test",R+"/"+rel,"--timeout","600000"]
        r=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True);out=r.stdout+r.stderr
        if os.environ.get("R79b_PROBE"):print("\n".join(l[:700] for l in out.splitlines() if l.startswith(("error","Expected","Received","(fail)","(pass)"," ")) and "|" not in l[:6])[-6000:])
        failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0 and stop:i=out.find("error:");print(out[max(0,i-800):i+1500],flush=True);break
    return res
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R79b-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r79b.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=mk()
    if any(h=="CONFLICT" for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    before={R+"/"+rel:(rd(R+"/"+rel) if os.path.exists(R+"/"+rel) else None) for i,rel,kind,data,how in plan}
    first=[x for x in plan if "🪪️exact-identity" in x[1] or "🩺️open-failure" in x[1]]
    for i,rel,kind,data,how in first:put(R+"/"+rel,data)
    base_errors=tscset()
    pre=tests(TESTS,stop=False)
    print("BEFORE (old tree, new tests):",pre,flush=True)
    for i,rel,kind,data,how in plan:
        p=R+"/"+rel
        if how!="same":put(p,data);applied.append(p)
    for i,rel,kind,data,how in first:
        if R+"/"+rel not in applied:applied.append(R+"/"+rel)
    if os.environ.get("R79b_PROBE"):
        tests([GROWTH,DP+"🧪️tests/🚧️fence/🟦️.ts",DS+"🧪️tests"],stop=False)
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
        print("PROBE: files restored");sys.exit(0)
    new_errors=tscset()-base_errors
    print("tsc new errors:",len(new_errors),flush=True)
    for l in sorted(new_errors)[:40]:print("  ",l[:1500])
    res=tests(TESTS,stop=False);ok=all(x[1]==0 for x in res) and len(res)==len(TESTS) and not new_errors
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
        print("RED: files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R79b landing 1 (%s): exact physical identity tokens, SQLite open diagnostics\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- R79b: `nativePhysicalIdentityTokenV1` (platform contract): dev and ino of the protected maintenance request are the JSON integer while safe, else decimal digits (schema `PhysicalIdentity`, canonical check in the reader); the dependency observation, the fixed backend physical facts and roster (bigint stats), the platform publication terminal root follow; `NativeProtectedMaintenanceRequestV1` no longer derives from the parent-read request. Fixes `Exact safe dependency physical identity required` on Windows (NTFS file ids above 2^53). SQLite: `SqliteDatabaseV1` explains an unopenable file with path, create-probe errno, database state, free space (EN+DE, original code kept) and creates missing parents of a database opened for creation once. Laws `🪪️exact-identity`, `🩺️open-failure`. Script `🧰️scripts/R79b/land.py`.\n")
        f.write("- before (new test on old tree): %s\n"%pre)
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
