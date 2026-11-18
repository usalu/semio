#!/usr/bin/env python3
"""R94 landing (R77 style): files are applied ONLY inside the landing lock; member tests and the standing preflight tests run; any red restores every file.
usage: land.py STAGE_DIR LABEL --dry-run|--land   (STAGE_DIR has index.txt "id rel kind", N.base/N.new, tests.txt with one test file per line relative to the worktree, note.txt for landings.md)"""
import os,sys,subprocess,datetime,tempfile,shutil
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";SI=B+"🕸️graph/📥️inference/🧭️script-inputs/"
PREFLIGHT=[SI+"🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts",SI+"🪆️heavy-questions/🧪️tests/🟦️.ts"]
S=sys.argv[1];LABEL=sys.argv[2]
rd=lambda p:open(p,"rb").read()
rows=[l.split(" ") for l in open(S+"/index.txt",encoding="utf8").read().splitlines() if l]
TESTS=[l for l in open(S+"/tests.txt",encoding="utf8").read().splitlines() if l]
def merged(i,rel,kind):
    live=R+"/"+rel;new=rd(S+"/"+i+".new")
    if kind=="new":
        if os.path.exists(live):return (rd(live),"same") if rd(live)==new else (new,"exists")
        return new,"create"
    base=rd(S+"/"+i+".base");cur=rd(live)
    if cur==base:return new,"clean"
    if cur==new:return new,"same"
    d=tempfile.mkdtemp(prefix="r94-merge-")
    try:
        for n,b in(("live",cur),("base",base),("new",new)):open(d+"/"+n,"wb").write(b)
        r=subprocess.run(["git","merge-file","-p",d+"/live",d+"/base",d+"/new"],capture_output=True)
        return (r.stdout,"merged" if r.returncode==0 else "CONFLICT")
    finally:shutil.rmtree(d,ignore_errors=True)
mk=lambda:[(i,rel,kind)+merged(i,rel,kind) for i,rel,kind in rows]
plan=mk()
for i,rel,kind,data,how in plan:print("%-8s %s"%(how,rel[len(L):] if rel.startswith(L) else rel))
if any(h=="CONFLICT" for *_,h in plan):sys.exit("conflicts: restage")
if "--dry-run" in sys.argv:print("dry ok",len(plan),"files");sys.exit(0)
if "--land" not in sys.argv:sys.exit("--dry-run or --land")
art=T+"/🗑️generated/R94-art"
def tests(dirs,stop=True):
    res=[]
    for rel in dirs:
        os.makedirs(art,exist_ok=True)
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=art
        r=subprocess.run([BUN,"test",R+"/"+rel,"--timeout","600000"],cwd=R,env=env,capture_output=True,text=True);out=r.stdout+r.stderr
        failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0:
            i=out.find("error:");print(out[max(0,i-800):i+1500],flush=True)
            if stop:break
    return res
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R94-%s-%d %s\n"%(LABEL,os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r94.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=mk()
    if any(h=="CONFLICT" for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    before={R+"/"+rel:(rd(R+"/"+rel) if os.path.exists(R+"/"+rel) else None) for i,rel,kind,data,how in plan}
    pre=[]
    if os.path.exists(S+"/before.txt"):
        first=[x for x in plan if x[2]=="new" and "🧪️tests" in x[1]]
        for i,rel,kind,data,how in first:
            if how!="same":put(R+"/"+rel,data);applied.append(R+"/"+rel)
        pre=tests([l for l in open(S+"/before.txt",encoding="utf8").read().splitlines() if l],stop=False);print("BEFORE (old code, new tests):",[(a[len(L):],b) for a,b,c in pre],flush=True)
    for i,rel,kind,data,how in plan:
        if how!="same" and R+"/"+rel not in applied:put(R+"/"+rel,data);applied.append(R+"/"+rel)
    res=tests(TESTS,stop=False);ok=all(x[1]==0 for x in res)
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
        print("RED: files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R94 landing %s (%s)\n"%(LABEL,datetime.datetime.now().strftime("%Y-%m-%d %H:%M")))
        f.write(open(S+"/note.txt",encoding="utf8").read()+"\n")
        if pre:f.write("- before (new law on old code): %s\n"%[(a[len(L):],b) for a,b,c in pre])
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
