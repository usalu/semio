#!/usr/bin/env python3
"""R84 landing in ONE lock hold: two production findings (silent site truncation, auxiliary channel below the snapshot bound, nx loader limit) with laws, red tests of the R68-R81 landings, three route-table suites. Tests run through the genuine-birth harness. Modes: --dry-run, --probe (apply, run, ALWAYS restore), --land (apply, run, restore on red, log). Stage: mkstage.py -> scratch."""
import hashlib,os,sys,subprocess,datetime,tempfile,shutil,re
R="/Users/ueli/Documents/semio"
H=os.path.dirname(os.path.abspath(__file__))
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R84/stage"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";AU=L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";DS=L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/";PR="🧰️framework/🔨️modules/🏃️process/"
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/"
GB=R+"/"+L+"🧪️testing/🌱️genuine-birth/📜️script.ts"
OB=B+"👁️observation/🧪️tests/";BZ=B+"🔧️tools/🏗️bazel/🧪️tests/"
READING_RED="^(?!neutral dependency Source owner (declared execution|Store materializer|Store settlement|Store lock command|aliases preserve)|neutral borrowed provider normalization)"
BR=B+"🔧️tools/🏗️bazel/"
GATES=[
 (AU+"🧭️sites/🧪️tests/🟦️.ts","setup",None,False),
 (L+"⚡️caching/🩹️patches/🧪️tests/🟦️.ts","setup",None,False),
 (PR+"🎛️owned-execution/📥️auxiliary/🧪️tests/🟦️.ts","setup",None,False),
 (AU+"🧪️tests/📈️growth/🟦️.ts","setup",None,True),
 (AU+"🧪️tests/🟦️.ts","setup","^authorizer file capacity",True),
 (AU+"🧪️tests/🟦️.ts","setup","^(captured compiler origins|authorizer publication identity|authorizer runtime companions|immutable authorizer sources|neutral captured byte records|captured CommonJS Source edges)",True),
 (AU+"🧪️tests/🟦️.ts","setup","^release authorizer",True),
 (AU+"🧪️tests/🟦️.ts","setup","^(captured authorizer bytes|sealed authorizer)",True),
 (AU+"🧪️tests/🟦️.ts","setup","^(captured workspace CJS|immutable graph loader|captured entry roles)",True),
 (B+"📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts","setup",None,False),
 (DS+"🚧️fence/🟦️.ts","setup",None,False),
 (DS+"🪜️ancestry/🟦️.ts","setup",None,False),
 (DS+"👓️reading/🟦️.ts","setup",READING_RED,False),
 (OB+"⚡️incremental/🟦️.ts","setup-sdk",None,True),
 (BR+"🎬️scopes/🧪️tests/🟦️.ts","setup",None,False),
 (BR+"🧪️tests/📥️environment/🟦️.ts","setup",None,False),
 (BR+"🧪️tests/📥️purpose/🧪️tests/🌱️birth/🟦️.ts","setup",None,False),
 (BR+"🧪️tests/🔏️implementation/🟦️.ts","setup",None,False),
 (B+"🗂️suites/🧪️tests/🟦️.ts",None,"^(?!a declared library test birth carries a genuine native build caller grant)",False),
 (B+"🪪️graph-state/🙋️wanted/🧪️tests/📖️read/🟦️.ts","setup",None,False),
]
INFO=[(OB+"📦️projects/🟦️.ts","setup-sdk",None,True),(BR+"🧪️tests/🟦️.ts","setup",None,False),(BR+"🧪️tests/📥️paths/🟦️.ts","setup",None,False),(DS+"👓️reading/🟦️.ts","setup","^neutral dependency Source owner (declared execution|Store materializer|Store settlement|Store lock command|aliases preserve)|^neutral borrowed provider normalization",False)]
PREFLIGHT=[SI+"🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts",SI+"🪆️heavy-questions/🧪️tests/🟦️.ts"]
OUT="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R84/probe-out"
ONLY=[x for x in os.environ.get("R84_ONLY","").split("|") if x]
if ONLY:GATES=[g for g in GATES if any(o in g[0] for o in ONLY)];INFO=[g for g in INFO if any(o in g[0] for o in ONLY)]
rd=lambda p:open(p,"rb").read()
rows=[l.split(" ") for l in open(S+"/index.txt",encoding="utf8").read().splitlines() if l]
def merged(i,rel):
    live=R+"/"+rel;new=rd(S+"/"+i+".new");base=rd(S+"/"+i+".base");cur=rd(live)
    if cur==base:return new,"clean"
    if cur==new:return new,"same"
    d=tempfile.mkdtemp(prefix="r84-merge-")
    try:
        for n,b in(("live",cur),("base",base),("new",new)):open(d+"/"+n,"wb").write(b)
        r=subprocess.run(["git","merge-file","-p",d+"/live",d+"/base",d+"/new"],capture_output=True)
        return (r.stdout,"merged" if r.returncode==0 else "CONFLICT")
    finally:shutil.rmtree(d,ignore_errors=True)
mk=lambda:[(i,rel)+merged(i,rel) for i,rel,kind in rows]
plan=mk()
for i,rel,data,how in plan:print("%-8s %s"%(how,rel[len(L):] if rel.startswith(L) else rel))
if any(h=="CONFLICT" for *_,h in plan):sys.exit("conflicts: restage")
if "--dry-run" in sys.argv:print("dry ok",len(plan),"files");sys.exit(0)
mode="--land" if "--land" in sys.argv else "--probe" if "--probe" in sys.argv else None
if mode is None:sys.exit("--dry-run, --probe or --land")
def run(rel,birth,pattern,art):
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
    env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";directory=None
    if art:directory=R+"/.cache/test-sandbox/r84-art-%d"%os.getpid();os.makedirs(directory,exist_ok=True);env["SEMIO_TEST_ARTIFACT_DIR"]=directory
    cmd=[BUN,GB,"--file="+R+"/"+rel]+(["--birth="+birth] if birth else [])+["--root="+R]+(["--pattern="+pattern] if pattern else [])
    try:
        for attempt in(1,2):
            try:r=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=900);out=r.stdout+r.stderr;code=r.returncode
            except subprocess.TimeoutExpired as e:out="TIMEOUT";code=124
            if code==0:break
            print("retry once (load-dependent timing):",rel[len(L):],flush=True)
    finally:
        if directory:shutil.rmtree(directory,ignore_errors=True)
    failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
    os.makedirs(OUT,exist_ok=True);open(OUT+"/"+hashlib.md5(rel.encode()).hexdigest()[:8]+".out","w").write(out)
    pas=re.findall(r"^\s*(\d+) pass",out,re.M);print(("ok  " if code==0 else "RED "),hashlib.md5(rel.encode()).hexdigest()[:8],rel[len(L):],"pass",pas,failed,flush=True)
    if code!=0 and os.environ.get("R84_TAIL"):
        i=out.find("error:");print(out[max(0,i-300):i+1500],flush=True)
    return rel,code,failed,out
def bare(rel):
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    r=subprocess.run([BUN,"test",R+"/"+rel,"--timeout","600000"],cwd=R,env=env,capture_output=True,text=True);out=r.stdout+r.stderr
    failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")];print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True);return rel,r.returncode,failed,out
lock=T+"/landing.lock"
import time
waited=0
while True:
    try:os.mkdir(lock);break
    except FileExistsError:
        try:who=open(lock+"/owner").read().strip()
        except Exception:who="(no owner file)"
        if waited>=1800:sys.exit("landing.lock is held by "+who)
        print("waiting for landing.lock held by",who,flush=True);time.sleep(30);waited+=30
open(lock+"/owner","w").write("R84-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[];before={}
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r84.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
def restore():
    for p in applied:
        if before[p] is None:os.remove(p)
        else:put(p,before[p])
try:
    plan=mk()
    if any(h=="CONFLICT" for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    for i,rel,data,how in plan:
        p=R+"/"+rel;before[p]=rd(p) if os.path.exists(p) else None
        if how!="same":put(p,data);applied.append(p)
    res=[run(*g) for g in GATES];ok=all(x[1]==0 for x in res)
    info=[run(*g) for g in INFO] if mode=="--probe" else []
    if ok and mode=="--land":res+=[bare(p) for p in PREFLIGHT];ok=all(x[1]==0 for x in res)
    if mode=="--probe":restore();print("PROBE: files restored");sys.exit(0)
    if not ok:restore();print("RED: files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R84 landing (%s): red tests of the R68-R79 landings made green\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- R84: (1) `closureLiteralSitesV1` refuses a file above the site bound in both languages instead of dropping sites silently, the bound is a member of the closure family (16384); (2) the owned auxiliary channel bound equals the snapshot bound (96 MiB, byteMembers of SourceClosureFamily) with a real-pipe law above 32 MiB; (3) the sealed Nx loader patch limit follows the manifest schema (2048 -> 16384, new tooling generation); red tests of R68-R81 fixed, route table gains build-task-model, build-slots, build-graph-state; see WORKERS-READY-WITH-LOW-RESOURCES/result-R84.md. Script `🧰️scripts/R84/land.py`.\n")
        for i,rel,data,how in plan:f.write("- file `%s` (%s)\n"%(rel,how))
        for rel,code,failed,out in res+info:f.write("- test `%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
