import json,os,subprocess,sys,re
L="/Users/ueli/Documents/semio/"
S=os.environ["R164_SCRATCH"]
BUN="/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
tests=json.load(open(os.path.dirname(os.path.abspath(__file__))+"/tests.json"))
mode=sys.argv[1];lo,hi=int(sys.argv[2]),int(sys.argv[3])
env={k:v for k,v in os.environ.items() if not k.startswith("SEMIO_REMOTE_CACHE") and not k.startswith("SEMIO_FLEET_")}
os.makedirs(S+"/out-"+mode,exist_ok=True)
for i in range(lo,min(hi,len(tests))):
    if mode=="live": cwd=os.path.dirname(L+tests[i]);cmd=[BUN,"test","./🟦️.ts","--timeout","180000"]
    else: cwd=S+"/stage/ex";cmd=[BUN,"test","./%02d.test.ts"%i,"--timeout","180000"]
    r=subprocess.run(cmd,cwd=cwd,env=env,capture_output=True,text=True,stdin=subprocess.DEVNULL,timeout=900)
    o=r.stdout+r.stderr;open(S+"/out-%s/%02d.txt"%(mode,i),"w").write(o)
    ps=re.findall(r"^\s*(\d+) pass",o,re.M);fs=re.findall(r"^\s*(\d+) fail",o,re.M)
    print(mode,i,tests[i].split("/")[-2],"pass",ps[-1] if ps else "?","fail",fs[-1] if fs else "?","|"," ; ".join(re.findall(r"^\(fail\) (.{0,70})",o,re.M)[:3]),flush=True)
