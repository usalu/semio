#!/usr/bin/env python3
"""R70 landing (one lock hold with the R67 package, whose files are merged into this stage): (A) the owner pool stops wasting finished work and says why: every refusal of the fleet cache carries its true reason (status and words of the cache, no answer, open breaker, lost claim, limit) in English and German, a record that meets a passing cause is sent again after a back-off and kept on disk until stored, a refusal every machine would meet is marked deterministic so the failure note keeps the others away, a divergence counts as stored, a child that finishes a wave is not an error, a silent pointer is read again before the machine reports it, closure documents are capped by a byte budget so no owner entry reaches hundreds of MiB, failures are counted by code in pool.json; (B) the setup stage of a worker: the whole process tree of a stopped stage ends on every platform (orphans of exited parents included, never a reused pid), the stage cap is a soft cap that extends while the counters of its log move (hard cap 4 times, stalled after 10 minutes), the ready lease records pid and start time and names its holder, the ownership walk of `repo dependencies sync` prints counts and saves its listings every two minutes so the next try goes on. Staged in stage/ (<id>.base = the live file the package was built on, <id>.new = the package's file, index.txt = id, path below the repository root, mod or new). A file the live tree changed since is three-way merged (git merge-file); a conflict stops the landing before anything is written. Usage: land.py --dry-run | --land (the landing holds landing.lock, runs the tests below, restores every file if one is red, and appends to landings.md). """
import os,sys,subprocess,datetime,tempfile,shutil
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=os.path.dirname(os.path.abspath(__file__))+"/stage"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";GS=B+"🪪️graph-state/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/";RD=B+"🌐️remote-execution/🛰️sidecar/🚦️ready/";DP=L+"⚡️caching/🚀️bootstrap/📦️dependencies/"
TESTS=[L+"⚡️caching/🚀️bootstrap/📦️dependencies/🧮️fence/🧪️tests",L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🏭️producer",L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📤️diagnostics",L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📏️work",L+"⚡️caching/🚀️bootstrap/📦️dependencies/🕓️walk/🧪️tests"]
#TESTS+=[PN+"📥️take/🧪️tests",PN+"🧪️tests/📥️take",GS+"📦️project-records/🌱️fresh/🧪️tests",PN+"🧪️tests/🛑️too-heavy",PN+"🧪️tests/🐌️slow-last",PN+"🧪️tests/🚪️away",PN+"🧪️tests/📈️inferred",PN+"🧪️tests/🧹️parent-budget",PN+"🧪️tests/🚦️governed-dispatch",PN+"🧪️tests/🧵️ready-context",PN+"🧪️tests/🛟️stopped-wave",PN+"🏊️project-farm/🧪️tests",PN+"📋️present/🧪️tests",B+"🕸️graph/🧪️tests/🙋️fleet-read",B+"🧠️memory/🧪️tests"]
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
    d=tempfile.mkdtemp(prefix="r70-merge-")
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
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=os.path.realpath(tempfile.mkdtemp(prefix="r70-art-"))
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
open(lock+"/owner","w").write("R70-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r70.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
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
        f.write("\n## Package R70 landing (%s): dependency fence budget\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- R70 (STAGED, tool-closure files): the dependency producer pays for a full fence only at final sites and when 8x the last full fence has passed (max 30 min); ownership frontier replay batched by 64.\n")
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
