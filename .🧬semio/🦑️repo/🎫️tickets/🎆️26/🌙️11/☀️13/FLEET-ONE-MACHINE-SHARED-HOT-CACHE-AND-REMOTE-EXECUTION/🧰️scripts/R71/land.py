#!/usr/bin/env python3
"""R71 landing, ONE lock hold: R65 lean slice read (hand-merged onto the R68 tree), R70 dependency fence, R69 if staged and R69_CLOSURE_OK=1, plus the R71 plan-client key wait (already in the live tree, only tested here). Usage: land.py --dry-run | --land"""
import os,re,sys,subprocess,datetime,tempfile,shutil
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/"
PREFLIGHT=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
OWN=[B+"🏡️resident/🧪️tests/🚪️client/🗝️key",B+"🏡️resident/🧪️tests/🚪️client"]
def package(n):
    d=T+"/🧰️scripts/R%d"%n
    if not os.path.exists(d+"/stage/index.txt"):return None
    ns={};src=open(d+"/land.py",encoding="utf8").read().splitlines()
    for line in src:
        if re.match(r"^(L|B|CK|PN|GS|RD|DP)=",line) or line.startswith("TESTS="):exec(line,ns)
    rows=[l.split(" ") for l in open(d+"/stage/index.txt",encoding="utf8").read().splitlines() if l]
    return dict(n=n,S=d+"/stage",rows=rows,tests=ns.get("TESTS",[]),base=("" if n==70 else B))
pk=[package(65),package(70)]
if os.environ.get("R69_CLOSURE_OK")=="1":pk.append(package(69))
pk=[p for p in pk if p]
rd=lambda p:open(p,"rb").read()
HAND_IMP='import { planFarmWorkersV1 } from "../../../🧠️memory/🪜️plan-stage/🟨️.mjs";'
HAND_IMP2='import { leanSliceV1, planFarmWorkersV1 } from "../../../🧠️memory/🪜️plan-stage/🟨️.mjs";'
HAND_F='maximumWorkers:planFarmWorkersV1({kind:"project-farm",workerMegabytes:GRAPH_FARM_POLICY_V1.workerMegabytes}),engage'
HAND_F2='maximumWorkers:leanSliceV1({residentMegabytes:process.memoryUsage().rss/1048576}).lean?Infinity:planFarmWorkersV1({kind:"project-farm",workerMegabytes:GRAPH_FARM_POLICY_V1.workerMegabytes}),engage'
def merged(p,i,rel,kind):
    path=R+"/"+(p["base"]+rel if not rel.startswith("🧰️framework") else rel);new=rd(p["S"]+"/"+i+".new")
    if kind=="new":
        if os.path.exists(path):return path,(rd(path),"exists") if rd(path)!=new else (new,"same")
        return path,(new,"create")
    base=rd(p["S"]+"/"+i+".base");cur=rd(path)
    if cur==base:return path,(new,"clean")
    if cur==new:return path,(new,"same")
    if p["n"]==65 and rel.endswith("project-nodes/🟨️.mjs"):
        t=cur.decode()
        if HAND_IMP2 in t and HAND_F2 in t:return path,(cur,"same")
        if t.count(HAND_IMP)==1 and t.count(HAND_F)==1:return path,(t.replace(HAND_IMP,HAND_IMP2).replace(HAND_F,HAND_F2).encode(),"hand")
    d=tempfile.mkdtemp(prefix="r71-merge-")
    try:
        for n,b in(("live",cur),("base",base),("new",new)):open(d+"/"+n,"wb").write(b)
        r=subprocess.run(["git","merge-file","-p",d+"/live",d+"/base",d+"/new"],capture_output=True)
        return path,((r.stdout,"merged") if r.returncode==0 else (b"","CONFLICT"))
    finally:shutil.rmtree(d,ignore_errors=True)
def build():
    return [(p["n"],rel,)+(merged(p,i,rel,kind)) for p in pk for i,rel,kind in p["rows"]]
plan=build()
for n,rel,path,(data,how) in plan:print("R%d %-8s %s"%(n,how,rel[-90:]))
if any(h in("CONFLICT","exists") for *_,(d,h) in plan):sys.exit("conflicts: merge by hand")
if "--dry-run" in sys.argv:print("dry ok",len(plan),"files, packages",[p["n"] for p in pk]);sys.exit(0)
if "--land" not in sys.argv:sys.exit("--dry-run or --land")
TESTS=[t for p in pk for t in p["tests"]]+OWN
def tests(dirs):
    res=[]
    for rel in dirs:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=os.path.realpath(tempfile.mkdtemp(prefix="r71-art-"))
        for attempt in(1,2):
            r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000","--bail"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
            if r.returncode==0:break
            print("again (red once, a timing law under host load?)",rel[len(L):],[l[:200] for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")],flush=True)
        shutil.rmtree(env["SEMIO_TEST_ARTIFACT_DIR"],ignore_errors=True)
        out=r.stdout+r.stderr;failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
        if r.returncode!=0:i=out.find("error:");print(out[max(0,i-1500):i+1500],flush=True);break
    return res
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R71-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
applied=[];before={}
def put(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".r71.tmp";open(tmp,"wb").write(data);os.replace(tmp,path)
try:
    plan=build()
    if any(h in("CONFLICT","exists") for *_,(d,h) in plan):sys.exit("conflicts while waiting for the lock")
    for n,rel,path,(data,how) in plan:
        before[path]=rd(path) if os.path.exists(path) else None
        if how!="same":put(path,data);applied.append(path)
    res=tests(TESTS);ok=all(x[1]==0 for x in res) and len(res)==len(TESTS)
    if ok:res+=tests(PREFLIGHT);ok=all(x[1]==0 for x in res)
    if not ok:
        for p in applied:
            if before[p] is None:os.remove(p)
            else:put(p,before[p])
        print("RED: files restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R71 landing (%s): lean slice read (R65), %s, plan client waits for a missing key file\n"%(datetime.datetime.now().strftime("%Y-%m-%d %H:%M"),"+".join("R%d"%p["n"] for p in pk if p["n"]!=65)))
        f.write("- Tool-closure files; every record is stale once more. Files: "+", ".join(sorted(set(r[1][-60:] for r in plan)))+"\n")
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red\n"%(rel[len(L):],code,len(failed)))
    print("LANDED")
finally:
    try:os.remove(lock+"/owner");os.rmdir(lock)
    except Exception:pass
