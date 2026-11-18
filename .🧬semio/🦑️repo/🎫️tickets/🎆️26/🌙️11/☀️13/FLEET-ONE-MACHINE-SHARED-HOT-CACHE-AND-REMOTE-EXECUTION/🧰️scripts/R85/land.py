#!/usr/bin/env python3
"""R85 landing (R80 style): files are applied ONLY inside the landing lock; the new test files (and every changed file below a tests folder) go first and run against the old code (BEFORE), then the code, then the member tests and the standing preflight tests; any red restores every file.
usage: land.py STAGE_DIR LABEL --dry-run|--land   (STAGE_DIR has index.txt "id rel kind", base/ID and new/ID, tests.txt, before.txt, note.txt)"""
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
    live=R+"/"+rel;new=rd(S+"/new/"+i)
    if kind=="new":
        if os.path.exists(live):return (rd(live),"same") if rd(live)==new else (new,"exists")
        return new,"create"
    base=rd(S+"/base/"+i);cur=rd(live)
    if cur==base:return new,"clean"
    if cur==new:return new,"same"
    d=tempfile.mkdtemp(prefix="r85-merge-")
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
art="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R85/art"
def tests(dirs,stop=True):
    res=[]
    for rel in dirs:
        os.makedirs(art,exist_ok=True)
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=art
        r=subprocess.run([BUN,"test",R+"/"+rel,"--timeout","600000"],cwd=R,env=env,capture_output=True,text=True);out=r.stdout+r.stderr
        failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        os.makedirs(art+"-out",exist_ok=True);open(art+"-out/"+datetime.datetime.now().strftime("%H%M%S")+"-"+os.path.basename(os.path.dirname(rel))[:20].encode("ascii","ignore").decode()+".txt","w",encoding="utf8").write(out)
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):] if rel.startswith(L) else rel,failed,flush=True)
        if r.returncode!=0:
            i=out.find("error:");print(out[max(0,i-800):i+1500],flush=True)
            if stop:break
    return res
lock=T+"/landing.lock"
import time
if "--any-time" not in sys.argv:
    while True:
        now=datetime.datetime.now(datetime.timezone.utc)
        if now.minute%30<5:break
        if int(time.time())%60<3:print("waiting for the landing window (xx:00-xx:05, xx:30-xx:35 UTC), now",now.strftime("%H:%M"),flush=True)
        time.sleep(3)
for waited in range(1200):
    try:os.mkdir(lock);break
    except FileExistsError:
        if waited%30==0:print("waiting for landing.lock held by",open(lock+"/owner").read().strip() if os.path.exists(lock+"/owner") else "?",flush=True)
        time.sleep(3)
else:sys.exit("landing.lock stayed held")
open(lock+"/owner","w").write("R85-%s-%d %s\n"%(LABEL,os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r85.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=mk()
    if any(h=="CONFLICT" for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    before={R+"/"+rel:(rd(R+"/"+rel) if os.path.exists(R+"/"+rel) else None) for i,rel,kind,data,how in plan}
    pre=[]
    def restore():
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
    if os.path.exists(S+"/before.txt") and "--no-before" not in sys.argv:
        first=[x for x in plan if "🧪️tests" in x[1]]
        for i,rel,kind,data,how in first:
            if how!="same":put(R+"/"+rel,data);applied.append(R+"/"+rel)
        pre=tests([l for l in open(S+"/before.txt",encoding="utf8").read().splitlines() if l],stop=False);print("BEFORE (old code, new tests):",[(a[len(L):] if a.startswith(L) else a,b) for a,b,c in pre],flush=True)
    for i,rel,kind,data,how in plan:
        if how!="same" and R+"/"+rel not in applied:put(R+"/"+rel,data);applied.append(R+"/"+rel)
    res=tests(TESTS,stop=True)
    import re
    names=lambda failed:{re.sub(r" \[[0-9.]+m?s\]$","",x) for x in failed}
    baseline={a:names(c) for a,b,c in pre if os.path.exists(S+"/baseline.txt") and a in open(S+"/baseline.txt",encoding="utf8").read().split("\n")}
    res=[(rel,0 if code!=0 and rel in baseline and names(failed)<=baseline[rel] and failed else code,failed) for rel,code,failed in res]
    ok=all(x[1]==0 for x in res)
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:
        restore();print("RED: files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R85 landing %s (%s)\n"%(LABEL,datetime.datetime.now().strftime("%Y-%m-%d %H:%M")))
        f.write(open(S+"/note.txt",encoding="utf8").read()+"\n")
        if pre:f.write("- before (new laws on old code): %s\n"%[(a[len(L):] if a.startswith(L) else a,b) for a,b,c in pre])
        for rel,code,failed in res:f.write("- test `%s` exit %d, %d red\n"%(rel[len(L):] if rel.startswith(L) else rel,code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
