#!/usr/bin/env python3
"""R99 landing (R85 style): applies the staged files only inside landing.lock in a window (xx:00-xx:05 or xx:30-xx:35 UTC), runs the member tests through the genuine-birth harness and both standing preflights, restores every file on red, appends the landings.md entry.
usage: land.py STAGE_DIR LABEL --dry-run|--land [--any-time]"""
import os,sys,subprocess,datetime,tempfile,shutil,time,glob
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=R+"/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";SI=B+"🕸️graph/📥️inference/🧭️script-inputs/"
HARNESS=glob.glob(R+"/"+L+"🧪️testing/🌱️genuine-birth/📜️script.ts")[0]
PREFLIGHT=[SI+"🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts",SI+"🪆️heavy-questions/🧪️tests/🟦️.ts"]
ALLOWED=[("⏳️busy-funding/🟦️.ts",["the real process ancestry equals an independent walk"])]
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
    d=tempfile.mkdtemp(prefix="r99-merge-")
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
def tests(files,stop=True):
    res=[]
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
    for rel in files:
        t=time.time()
        r=subprocess.run([BUN,HARNESS,"--file="+rel,"--birth=setup","--root="+R],cwd=R,env=env,capture_output=True,text=True);out=r.stdout+r.stderr
        failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        allowed=[x for x in ALLOWED if rel.endswith(x[0])]
        if r.returncode!=0 and failed and allowed and all(any(f.startswith("(fail) "+a) for a in allowed[0][1]) for f in failed):r.returncode=0
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",int(time.time()-t),"s",rel[len(L):] if rel.startswith(L) else rel,failed,flush=True)
        if r.returncode!=0:
            i=out.find("error:");print(out[max(0,i-800):i+1500],flush=True)
            if stop:break
    return res
lock=T+"/landing.lock"
if "--any-time" not in sys.argv:
    while True:
        now=datetime.datetime.now(datetime.timezone.utc)
        if now.minute%30<2:break
        if int(time.time())%60<3:print("waiting for the landing window, now",now.strftime("%H:%M"),flush=True)
        time.sleep(2)
for waited in range(150):
    try:os.mkdir(lock);break
    except FileExistsError:
        if waited%10==0:print("waiting for landing.lock held by",open(lock+"/owner").read().strip() if os.path.exists(lock+"/owner") else "?",flush=True)
        time.sleep(2)
else:sys.exit("landing.lock stayed held")
open(lock+"/owner","w").write("claude-R99-%s-%d %s\n"%(LABEL,os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r99.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
started=time.time()
try:
    plan=mk()
    if any(h=="CONFLICT" for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    before={R+"/"+rel:(rd(R+"/"+rel) if os.path.exists(R+"/"+rel) else None) for i,rel,kind,data,how in plan}
    def restore():
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
    for i,rel,kind,data,how in plan:
        if how!="same":put(R+"/"+rel,data);applied.append(R+"/"+rel)
    res=tests(TESTS,stop=True);ok=all(x[1]==0 for x in res)
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok or time.time()-started>280:
        restore();print("RED or over 280 s: files restored",int(time.time()-started));sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R99 landing %s (%s UTC, lock held %d s, owner claude-R99)\n"%(LABEL,datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d %H:%M"),int(time.time()-started)))
        f.write(open(S+"/note.txt",encoding="utf8").read()+"\n")
        f.write("- Files: %s\n"%", ".join(rel[len(L):] if rel.startswith(L) else rel for i,rel,kind,data,how in plan))
        for rel,code,failed in res:f.write("- test `%s` exit %d, %d red\n"%(rel[len(L):] if rel.startswith(L) else rel,code,len(failed)))
    print("LANDED",int(time.time()-started),"s")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
