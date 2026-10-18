import sys,subprocess,re,json,os,hashlib,concurrent.futures as cf
root="/Users/ueli/Documents/semio/"
listf,outdir,jobs=sys.argv[1],sys.argv[2],int(sys.argv[3])
os.makedirs(outdir,exist_ok=True)
files=[l.strip() for l in open(listf) if l.strip()]
def run(p):
    h=hashlib.sha1(p.encode()).hexdigest()[:10]
    log=f"{outdir}/{h}.log"
    done=f"{outdir}/{h}.json"
    if os.path.exists(done): return json.load(open(done))
    for attempt in (1,2):
        try:
            r=subprocess.run(["nice","-n","10","perl","-e","alarm 900; exec @ARGV","--","bun","test"]+(["--timeout",os.environ["TSP_TIMEOUT"]] if os.environ.get("TSP_TIMEOUT") else [])+["./"+p],cwd=root,capture_output=True,text=True,errors="replace",env=dict(os.environ,SEMIO_TEST_ARTIFACT_DIR=root+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/tsp-artifacts"))
            out=r.stdout+r.stderr; rc=r.returncode
        except Exception as e:
            out=str(e); rc=-99
        seg=bool(re.search(r"(?i)segmentation|panic|Bun has crashed|SIGSEGV",out)) or rc in (-11,139,-6,134,142,-14)
        if seg and attempt==1: continue
        break
    m=re.search(r"(\d+) pass",out); n=re.search(r"(\d+) fail",out)
    pa=int(m.group(1)) if m else 0; fa=int(n.group(1)) if n else 0
    errs=int(re.search(r"(\d+) errors?",out).group(1)) if re.search(r"(\d+) errors?",out) else 0
    status="pass" if rc==0 and pa>0 and fa==0 else ("noTests" if rc==0 and pa==0 else ("crash" if seg else "fail"))
    open(log,"w").write(out)
    d=dict(path=p,status=status,rc=rc,pass_=pa,fail=fa,errors=errs,log=log,retried=attempt==2)
    json.dump(d,open(done,"w"),ensure_ascii=False)
    return d
res=[]
with cf.ThreadPoolExecutor(jobs) as ex:
    for i,r in enumerate(ex.map(run,files)):
        res.append(r)
        if i%25==0: print(i,len(files),flush=True)
json.dump(res,open(outdir+"/results.json","w"),ensure_ascii=False,indent=0)
import collections
print(collections.Counter(r["status"] for r in res))
