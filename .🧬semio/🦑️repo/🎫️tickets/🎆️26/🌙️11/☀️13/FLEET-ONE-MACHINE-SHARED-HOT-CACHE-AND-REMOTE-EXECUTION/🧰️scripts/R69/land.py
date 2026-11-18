#!/usr/bin/env python3
"""R69 landing (STAGED, NOT LANDED; another agent lands one combined batch): (1) a refused closed selected dependency row names every missing and unexpected field; (2) the retirement plan survives a retained generation whose `bindings.json` no longer has the CURRENT field set or version (`JavascriptDependencyRowMismatchV1`): it is classified `unready` (retire set), a bilingual `dependency-store.stale-generation` line names prefix, file, missing, unexpected, and the roles/admission/protocol accept `bindings.json` inside a never-ready generation; the receipt of a generation in `current.scopes` stays a hard error naming prefix and file. Staged in stage/ (<id>.base = live file the package was built on, <id>.new = the package's file, index.txt = id, path below the repository root, mod or new). The edited files are inside the declared tool closure `driver-stages` (see closure.py and result-R69.md), so landing re-stales every package record on the fleet: this script refuses to land unless R69_CLOSURE_OK=1 is set by whoever accepts that. Usage: land.py --dry-run | R69_CLOSURE_OK=1 land.py --land (holds landing.lock, runs the tests below, restores every file when one is red, appends to landings.md)."""
import os,sys,subprocess,datetime,tempfile,shutil
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=os.path.dirname(os.path.abspath(__file__))+"/stage"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";DP=L+"⚡️caching/🚀️bootstrap/📦️dependencies/";PUB=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/"
TESTS=[DP+"💾️storage/🧪️tests/🧾️field-diagnosis",PUB+"⚙️setup/♻️retirement/🧪️tests/🪦️stale-generation",PUB+"⚙️setup/♻️retirement/🧪️tests",PUB+"⚙️setup/♻️retirement/🧪️tests/🔁️busy-driver",DP+"💾️storage/🧹️retirement/🗃️unready/🧪️tests",L+"⚡️caching/📖️texts/🧪️tests",DP+"💾️storage/🧪️tests/📤️diagnostics",DP+"💾️storage/🧪️tests/🏭️producer",DP+"💾️storage/🧪️tests/👓️reading",DP+"💾️storage/🧪️tests/📏️work"]
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
    d=tempfile.mkdtemp(prefix="r69-merge-")
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
if os.environ.get("R69_CLOSURE_OK")!="1":sys.exit("refused: the staged files are inside the tool closure driver-stages (result-R69.md); set R69_CLOSURE_OK=1 only when that re-stale of the fleet is accepted")
def tests(dirs):
    res=[]
    for rel in dirs:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=os.path.realpath(tempfile.mkdtemp(prefix="r69-art-"))
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000","--bail"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        shutil.rmtree(env["SEMIO_TEST_ARTIFACT_DIR"],ignore_errors=True)
        out=r.stdout+r.stderr;failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0:i=out.find("error:");print(out[max(0,i-1500):i+1500],flush=True);break
    return res
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R69-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r69.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
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
        f.write("\n## Package R69 landing (%s): selected dependency field refusals name their fields and the retained generation\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- `captureJavascriptDependencySelectedFieldsV1` (storage and `🏷️roles`): `Exact selected dependency fields required: missing [...], unexpected [...], found N of M allowed`, thrown as `JavascriptDependencyRowMismatchV1` in storage (also for a ready receipt of another version). Retirement plan (`captureRepositoryDependencySelectedPlanV1`): a retained generation outside `current.scopes` whose `bindings.json` mismatches the current field set/version becomes status `unready` (retire set) with the bilingual line `dependency-store.stale-generation` (prefix, file, missing, unexpected); `bindings.json` is now an `unreadyPayloadRoles` entry, accepted inside `unready-generation` by the role reader and the fresh admission (still refused inside a retained-unready capsule); a mismatching receipt of a current generation stays a hard error via `describeRepositoryDependencyCandidateV1`. Tests: new `🪦️stale-generation` (retirement tests) and `🧾️field-diagnosis`, updated unready `♻️roles` fixture and oracle. Not covered: a capsule `.p…` with payload whose intent `i` mismatches (stays a hard error naming prefix and file).\n")
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
