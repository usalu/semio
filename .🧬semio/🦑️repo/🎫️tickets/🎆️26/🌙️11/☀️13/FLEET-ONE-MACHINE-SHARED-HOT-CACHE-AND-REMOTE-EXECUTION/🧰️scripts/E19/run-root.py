# coding: utf-8
import json,os,subprocess,sys,time
HERE=os.path.dirname(os.path.abspath(__file__))
G=HERE+"/../../wave-1122/🗑️generated/E19/"
ROOT="/Users/ueli/Documents/semio"
BUN="/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
budget=float(sys.argv[1]); out=sys.argv[2]; lst=sys.argv[3]
files=[l.strip() for l in open(G+lst) if l.strip()]
done=set(json.loads(l)["file"] for l in open(G+out)) if os.path.exists(G+out) else set()
drop=("SEMIO_FLEET_GROUP","SEMIO_FLEET_NFS_HOST","SEMIO_FLEET_NETWORK","SEMIO_REMOTE_CACHE_ENDPOINT","SEMIO_REMOTE_CACHE_ACCESS","SEMIO_REMOTE_CACHE_INSTANCE","SEMIO_REMOTE_CACHE_TOKEN","SEMIO_TEST_ARTIFACT_DIR")
env={k:v for k,v in os.environ.items() if k not in drop}
env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"; env["SEMIO_TEST_PYTHON_BINARY"]="/usr/bin/python3"
t0=time.time()
for f in files:
    if f in done: continue
    if time.time()-t0>budget: break
    while os.getloadavg()[0]>40 and time.time()-t0<budget: time.sleep(20)
    s=time.time()
    try:
        p=subprocess.run([BUN,"test","./"+f,"--timeout","180000"],cwd=ROOT,env=env,capture_output=True,text=True,errors="replace",timeout=400); rc=p.returncode; txt=p.stdout+p.stderr
    except subprocess.TimeoutExpired:
        rc,txt=-9,"WALL-TIMEOUT"
    open(G+"rootlogs.txt","a").write("=== %s rc=%s %.1fs\n%s\n"%(f,rc,time.time()-s,txt[-3500:] if rc else txt[-300:]))
    open(G+out,"a").write(json.dumps({"file":f,"rc":rc,"sec":round(time.time()-s,1)},ensure_ascii=False)+"\n")
