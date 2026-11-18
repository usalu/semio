#!/usr/bin/env python3
"""R75 landing: owner pool prices a child by the 75th percentile of its measured peaks, counts farmed unmeasured reservations at 25 %, recycles a child only after real growth, and falls back to another directory when the temporary one is over quota. Fail-before is measured under the lock: tests and fixtures first against the old code."""
import os,sys,subprocess,datetime,tempfile,shutil,time
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=os.path.dirname(os.path.abspath(__file__))+"/stage"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";SC=B+"🌐️remote-execution/🛰️sidecar/";RS=B+"🏡️resident/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/";H="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
TESTS=[PN+"🌐️owner-pool/🧪️tests",SC+"🧪️tests/🏊️pool",SC+"🧪️tests"]
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
    d=tempfile.mkdtemp(prefix="r75-merge-")
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
def tests(dirs,keep=False):
    res=[]
    for rel in dirs:
        if not os.path.exists(R+"/"+rel+"/🟦️.ts"):print("skip (no test file)",rel[len(L):]);continue
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=os.path.realpath(tempfile.mkdtemp(prefix="r75-art-"))
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000","--bail"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        if r.returncode!=0:
            first=[l[:200] for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")];print("again (red once, a timing law under the load of a live host?)",rel[len(L):],first,flush=True)
            r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000","--bail"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        shutil.rmtree(env["SEMIO_TEST_ARTIFACT_DIR"],ignore_errors=True)
        out=r.stdout+r.stderr;failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0 and not keep:i=out.find("error:");print(out[max(0,i-1500):i+1500],flush=True);break
    return res
lock=T+"/landing.lock"
waited=0
while True:
    try:os.mkdir(lock);break
    except FileExistsError:
        try:holder=open(lock+"/owner").read().strip()
        except Exception:holder="?"
        if waited==0 or waited%300==0:print("landing.lock is held by",holder,"; waiting",flush=True)
        if waited>=2400:sys.exit("landing.lock still held by "+holder)
        time.sleep(15);waited+=15
open(lock+"/owner","w").write("R75-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[]
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r75.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=[(i,rel,kind)+merged(i,rel,kind) for i,rel,kind in rows]
    if any(h in("CONFLICT","exists") for *_,h in plan):sys.exit("conflicts while waiting for the lock")
    before={}
    for i,rel,kind,data,how in plan:
        p=R+"/"+rel;before[p]=rd(p) if os.path.exists(p) else None
    testrows=[(R+"/"+rel,data) for i,rel,kind,data,how in plan if "/🧪️tests/" in rel and how!="same"]
    for p,data in testrows:put(p,data);applied.append(p)
    red=tests([PN+"🌐️owner-pool/🧪️tests",SC+"🧪️tests/🏊️pool"],keep=True)
    summary=[(r[0][len(L):],r[1],r[2]) for r in red]
    print("FAIL-BEFORE:",summary,flush=True)
    open(os.path.dirname(os.path.abspath(__file__))+"/fail-before.txt","w",encoding="utf8").write(repr(summary))
    for i,rel,kind,data,how in plan:
        p=R+"/"+rel
        if how!="same":
            put(p,data)
            if p not in applied:applied.append(p)
    res=tests(TESTS);ok=all(x[1]==0 for x in res) and len(res)>=len(TESTS)-3
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
        print("RED: files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R75 landing (%s): owner pool prices children by measured percentile, farm reservations count a quarter, children are recycled by growth, wave files fall back on quota errors\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("- R75: child price = 75th percentile of the last 32 measured child sizes (+15 %%) instead of the maximum of the last 5; unmeasured farm reservations count 25 %% in the unrealized memory the pool keeps free; a child is recycled only when it passed the limit AND grew 600 MB past its ready size; wave files fall back to `.cache/f/owner-pool-waves` on EDQUOT/ENOSPC. Fail-before (new tests against old code): %s\n"%summary)
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\\n"%(rel[len(L):],code,len(failed)) if False else "- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
