import subprocess,sys,json,time,os,tempfile
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R89"
HERE=os.path.dirname(os.path.abspath(__file__))
K=int(sys.argv[1]);N=int(sys.argv[2]);WARM=sys.argv[3]
part=tempfile.mkdtemp(prefix="warm-parts-",dir=S)
t=time.time();ps=[subprocess.Popen([BUN,HERE+"/real-warm.ts","/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R80/wave",str(i),str(K),str(N//K),part,WARM],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,cwd="/Users/ueli/Documents/semio") for i in range(K)]
res=[]
for p in ps:
    o,e=p.communicate()
    r=[l for l in o.splitlines() if l.startswith("RESULT")]
    print(r[0] if r else ("FAIL "+e[-600:]),flush=True)
    if r:res.append(json.loads(r[0][7:]))
print("wall incl boot",round(time.time()-t,1),"max loop seconds",max(x["seconds"] for x in res),"owners",sum(x["owners"] for x in res))
rows=[json.loads(l) for f in os.listdir(part) for l in open(part+"/"+f) if l.strip()]
print("durable rows",len(rows),"distinct",len({r["n"] for r in rows}))
