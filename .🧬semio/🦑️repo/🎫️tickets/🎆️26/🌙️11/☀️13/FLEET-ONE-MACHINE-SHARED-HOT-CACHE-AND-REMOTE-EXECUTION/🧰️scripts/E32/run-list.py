import subprocess,sys,os,re,time,json,concurrent.futures as cf
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
OUT=T+"/wave-1122/🗑️generated/E32"
lst,par,budget,per=sys.argv[1],int(sys.argv[2]),int(sys.argv[3]),int(sys.argv[4])
resf=OUT+"/results.jsonl"
done={json.loads(l)["file"] for l in open(resf)} if os.path.exists(resf) else set()
files=[l.split(" || ")[0].strip() for l in open(lst) if l.strip()]
files=[f for f in files if f not in done]
start=time.time()
def run(f):
    if time.time()-start>budget: return None
    t=time.time()
    try:
        p=subprocess.run(["repo","test-file",f],capture_output=True,text=True,timeout=per,cwd="/Users/ueli/Documents/semio")
        out=p.stdout+p.stderr;rc=p.returncode
    except subprocess.TimeoutExpired as e:
        out=(e.stdout.decode(errors="replace") if isinstance(e.stdout,bytes) else (e.stdout or ""));rc="TIMEOUT"
    m=re.search(r"^\s*(\d+) pass",out,re.M);k=re.search(r"^\s*(\d+) fail",out,re.M)
    err=[e for e in re.findall(r"^error: (.*)$",out,re.M)]
    row={"file":f,"rc":rc,"pass":m.group(1) if m else None,"fail":k.group(1) if k else None,"sec":round(time.time()-t),"err":(err[0] if err else "")[:200]}
    open(resf,"a").write(json.dumps(row,ensure_ascii=False)+"\n")
    return row
with cf.ThreadPoolExecutor(par) as ex:
    for r in ex.map(run,files):
        if r: print(r["rc"],r["pass"],r["fail"],r["sec"],r["file"].split("⚡️caching/")[-1][-70:],"::",r["err"][:100],flush=True)
