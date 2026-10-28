"""⚖️ pregate-g: the cross-cutting graph-free laws, single-file `bun test ./…` per file, on a mirror root.
usage: laws.py ROOT OUT.jsonl [-j N] [--only SUBSTR]... [--list]"""
import json,os,re,subprocess,sys,time,resource
from concurrent.futures import ThreadPoolExecutor
root=os.path.abspath(sys.argv[1]);out=sys.argv[2]
jobs=int(sys.argv[sys.argv.index("-j")+1]) if "-j" in sys.argv else 3
only=[sys.argv[i+1] for i,a in enumerate(sys.argv) if a=="--only"]
F="🧰️framework";L=F+"/🛍️products/🦑️repo/🔨️modules/📚️library";P=F+"/🔨️modules/🏃️process";B=L+"/🏗️build"
dirs=[B+"/🚪️door",B+"/📥️caller/🧪️tests/🚪️door",B+"/🕸️graph/🕰️generations",B+"/🔒️leases",B+"/🌡️host-capacity",P+"/🏗️build/🌡️host-capacity",P+"/🔒️leases",
 L+"/⚡️caching/🦀️cargo/🗃️unit-cache",L+"/⚡️caching/🕸️graph/🏗️scheduling",B+"/📥️source/🔏️closure/🗓️scheduling",B+"/📥️source/🛗️zero-touch",L+"/⚖️laws/🛗️zero-touch-closure",L+"/⚖️laws/🗣️language-catalogs",P+"/🏗️build/🗣️language"]
files=[]
for d in dirs:
    for dp,dn,fn in os.walk(os.path.join(root,d)):
        if "🧪️tests" not in dp.split(os.sep) and not dp.endswith("🧪️tests"):continue
        for f in fn:
            if f.endswith(".ts") and not f.endswith(".d.ts") and (f.startswith("🟦️") or "test" in f):files.append(os.path.relpath(os.path.join(dp,f),root))
files=sorted(set(files))
if only:files=[f for f in files if any(o in f for o in only)]
if "--list" in sys.argv:print("\n".join(files),len(files));sys.exit(0)
bun=os.environ.get("PG_BUN","bun");here=os.path.dirname(os.path.abspath(__file__))
ENV_RED=re.compile(r"Original parent Script process invocation required|process invocation required|refuse[sd]? plain bun|Original (?:Script )?process")
def run(rel,envelope):
    t=time.time()
    cmd=["nice","-n","10",bun,"test","--timeout","120000","./"+rel] if not envelope else ["nice","-n","10",bun,here+"/penv.ts","file",rel]
    e=dict(os.environ,PG_ROOT=root,PG_BUN=bun,SEMIO_WORKSPACE_ROOT=root)
    try:
        r=subprocess.run(cmd,cwd=root,env=e,capture_output=True,text=True,timeout=1100)
        txt=(r.stdout or "")+(r.stderr or "");code=r.returncode
    except subprocess.TimeoutExpired as x:
        txt="TIMEOUT";code=124
    p=re.findall(r"^\s*(\d+) pass",txt,re.M);f=re.findall(r"^\s*(\d+) fail",txt,re.M)
    return code,int(p[-1]) if p else 0,int(f[-1]) if f else 0,txt,time.time()-t
def one(rel):
    code,p,f,txt,dt=run(rel,False);mode="plain"
    if code!=0 and ENV_RED.search(txt):
        code,p,f,txt,dt2=run(rel,True);mode="envelope";dt+=dt2
    rec={"file":rel,"mode":mode,"exit":code,"pass":p,"fail":f,"sec":round(dt,1)}
    if code!=0:rec["tail"]="\n".join([l for l in txt.splitlines() if re.search(r"error|fail|✗|expected|\(fail\)",l,re.I)][:12])[:2500]
    with open(out,"a") as h:h.write(json.dumps(rec,ensure_ascii=False)+"\n")
    return rec
t0=time.time()
with ThreadPoolExecutor(jobs) as ex:res=list(ex.map(one,files))
bad=[r for r in res if r["exit"]!=0]
print(f"laws: {len(res)} files, {sum(r['pass'] for r in res)} pass, {sum(r['fail'] for r in res)} fail, {len(bad)} files red, {time.time()-t0:.0f}s")
for r in bad:print("RED",r["file"].split("/")[-3:],r["mode"],r["pass"],r["fail"])
