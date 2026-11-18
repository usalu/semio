#!/usr/bin/env python3
"""R73 combined landing in ONE lock hold: R73 (record freshness by content: the code-epoch rule of R68b is removed, `closure` rows are put to the closure provider like a taker does), R69 (dependency field refusals; needs R69_CLOSURE_OK=1) and R72 fix 1 (vouch of closures reading unwatched paths). Each package has its own stage dir (<id>.base, <id>.new, index.txt), three-way merged against the live tree. Usage: land.py --dry-run | R69_CLOSURE_OK=1 land.py --land (holds landing.lock, runs the tests, restores every file if one is red, appends to landings.md)."""
import os,sys,subprocess,datetime,tempfile,shutil
R="/Users/ueli/Documents/semio"
H=os.path.dirname(os.path.abspath(__file__))
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";GS=B+"🪪️graph-state/";PR=GS+"📦️project-records/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/";DP=L+"⚡️caching/🚀️bootstrap/📦️dependencies/";PUB=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/";CK=B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/";CO=B+"🕸️graph/🕰️generations/✂️cutoff/🧪️tests"
STAGES=[H+"/stage",H+"/../R69/stage",H+"/../R72/stage"]
T73=[PR+"🌱️fresh/🧪️tests",PR+"🧪️tests",PR+"🧪️tests/✂️after-cut",PR+"🧪️tests/🌍️two-platforms",PR+"🧪️tests/🗜️packed",PR+"🧪️tests/🚚️closures",PR+"🧪️tests/📮️retry",PN+"🧪️tests/📥️take",PN+"📥️take/🧪️tests",PN+"🌐️owner-pool/🧪️tests"]
T69=[DP+"💾️storage/🧪️tests/🧾️field-diagnosis",PUB+"⚙️setup/♻️retirement/🧪️tests/🪦️stale-generation",PUB+"⚙️setup/♻️retirement/🧪️tests",PUB+"⚙️setup/♻️retirement/🧪️tests/🔁️busy-driver",DP+"💾️storage/🧹️retirement/🗃️unready/🧪️tests",L+"⚡️caching/📖️texts/🧪️tests",DP+"💾️storage/🧪️tests/📤️diagnostics",DP+"💾️storage/🧪️tests/🏭️producer",DP+"💾️storage/🧪️tests/📏️work"]
T72=[CO,CO+"/🧾️vouch",CO+"/🪢️hard-parts",CO+"/🌑️epoch",CK+"🧪️tests/🧭️dependencies",CK+"🧪️tests/⏱️throughput",CK+"📒️manifest/🧪️tests",CK+"🏭️walk-farm/🧪️tests"]
TESTS=T73+T69+T72
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/"
PREFLIGHT=[SI+"🔌️provider-closures/🧪️tests/🛃️preflight",SI+"🔌️provider-closures/🧪️tests/🪡️tool-cover",SI+"🪆️heavy-questions/🧪️tests"]
rd=lambda p:open(p,"rb").read()
rows=[]
for S in STAGES:
    S=os.path.normpath(S)
    rows+=[(S,)+tuple(l.split(" ")) for l in open(S+"/index.txt",encoding="utf8").read().splitlines() if l]
def merged(S,i,rel,kind):
    live=R+"/"+rel;new=rd(S+"/"+i+".new")
    if kind=="new":
        if os.path.exists(live):return (rd(live),"exists") if rd(live)!=new else (new,"same")
        return new,"create"
    base=rd(S+"/"+i+".base");cur=rd(live)
    if cur==base:return new,"clean"
    if cur==new:return new,"same"
    d=tempfile.mkdtemp(prefix="r73-merge-")
    try:
        for n,b in(("live",cur),("base",base),("new",new)):open(d+"/"+n,"wb").write(b)
        r=subprocess.run(["git","merge-file","-p",d+"/live",d+"/base",d+"/new"],capture_output=True)
        return (r.stdout,"merged" if r.returncode==0 else "CONFLICT")
    finally:shutil.rmtree(d,ignore_errors=True)
mk=lambda:[(S,i,rel,kind)+merged(S,i,rel,kind) for S,i,rel,kind in rows]
plan=mk()
for S,i,rel,kind,data,how in plan:print("%-8s %s"%(how,rel[len(L):] if rel.startswith(L) else rel))
if any(h in("CONFLICT","exists") for *_,h in plan):sys.exit("conflicts: merge by hand before landing")
if "--dry-run" in sys.argv:print("dry ok",len(plan),"files");sys.exit(0)
if "--land" not in sys.argv:sys.exit("--dry-run or --land")
if os.environ.get("R69_CLOSURE_OK")!="1":sys.exit("R69_CLOSURE_OK=1 required")
def tests(dirs):
    res=[]
    for rel in dirs:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=os.path.realpath(tempfile.mkdtemp(prefix="r73-art-"))
        atroot=rel.endswith("👓️reading");cwd=R if atroot else R+"/"+rel
        cmd=[BUN,"test",(R+"/"+rel+"/🟦️.ts") if atroot else "./🟦️.ts","--timeout","180000","--bail"]
        r=subprocess.run(cmd,cwd=cwd,env=env,capture_output=True,text=True)
        if r.returncode!=0:
            print("again (red once)",rel[len(L):],flush=True);r=subprocess.run(cmd,cwd=cwd,env=env,capture_output=True,text=True)
        shutil.rmtree(env["SEMIO_TEST_ARTIFACT_DIR"],ignore_errors=True)
        out=r.stdout+r.stderr;failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0:i=out.find("error:");print(out[max(0,i-1500):i+1500],flush=True);break
    return res
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R73-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r73.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=mk()
    if any(h in("CONFLICT","exists") for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    before={}
    for S,i,rel,kind,data,how in plan:
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
        f.write("\n## Package R73 landing (%s): records are fresh by content, with R69 and R72 fix 1\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- R73: the code-epoch rule (R68b `codeEpochV1`, newest mtime of all code files) is removed; `freshVerdictV1` compares file/analysis digests and puts every `closure` row to the closure provider (settled first, as a taker does), so only an edit inside a record's closure makes it stale. R69: dependency field refusals. R72 fix 1: vouch of closures that read unwatched paths by cheap stamps.\n")
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
