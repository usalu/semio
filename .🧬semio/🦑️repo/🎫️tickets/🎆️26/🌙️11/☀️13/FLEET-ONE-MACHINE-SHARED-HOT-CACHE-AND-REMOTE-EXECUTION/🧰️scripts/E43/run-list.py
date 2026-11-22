import subprocess,sys,os,re,time,json,glob,concurrent.futures as cf
ROOT="/Users/ueli/Documents/semio"
T=glob.glob(ROOT+"/.*semio/*repo/*tickets/*26/*11/*13/FLEET-ONE*/")[0]
OUT=glob.glob(T+"wave-1122/*generated")[0]+"/E43"
os.makedirs(OUT,exist_ok=True)
lst,resname,par,budget,per=sys.argv[1],sys.argv[2],int(sys.argv[3]),int(sys.argv[4]),int(sys.argv[5])
resf=OUT+"/"+resname
done={json.loads(l)["file"] for l in open(resf)} if os.path.exists(resf) else set()
files=[l.strip() for l in open(lst) if l.strip()]
files=[f for f in files if f not in done]
start=time.time()
def load(): return os.getloadavg()[0]
def run(f):
    while time.time()-start<budget and load()>40: time.sleep(30)
    if time.time()-start>budget: return None
    t=time.time()
    try:
        p=subprocess.run(["repo","test-file","--json",f],capture_output=True,text=True,timeout=per,cwd=ROOT)
        rc=p.returncode;out=p.stdout
        try:
            j=json.loads(out);r=j["files"][0] if "files" in j else {}
        except Exception: j={};r={}
        row={"file":f,"rc":rc,"status":r.get("status"),"pass":r.get("pass",r.get("passed")),"fail":r.get("fail",r.get("failed")),"err":(str(r.get("error") or ""))[:300],"sec":round(time.time()-t),"errs":[l for l in (r.get("tail") or "").splitlines() if l.startswith("error:") or "RepoError" in l or l.startswith("Expected") or l.startswith("Received")][:6],"raw":None if r else (out+p.stderr[-800:])[-600:]}
    except subprocess.TimeoutExpired:
        row={"file":f,"rc":"TIMEOUT","sec":round(time.time()-t)}
    open(resf,"a").write(json.dumps(row,ensure_ascii=False)+"\n")
    return row
with cf.ThreadPoolExecutor(par) as ex:
    for r in ex.map(run,files):
        if r: print(r["rc"],r.get("status"),r.get("pass"),r.get("fail"),r["sec"],r["file"].split("⚡️caching/")[-1][-70:],flush=True)
