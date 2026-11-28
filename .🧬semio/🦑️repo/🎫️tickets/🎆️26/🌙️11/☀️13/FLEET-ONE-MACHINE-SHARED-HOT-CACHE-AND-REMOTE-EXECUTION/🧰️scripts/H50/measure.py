#!/usr/bin/env python3
# paired local/remote wall durations of single test files, one file at a time; usage: measure.py out.jsonl file...
import json,os,subprocess,sys,time
out=sys.argv[1]; files=sys.argv[2:]
root="/Users/ueli/Documents/semio"
for f in files:
    for mode in ("never","always"):
        env=dict(os.environ,SEMIO_TEST_FILE_OFFLOAD=mode)
        load=os.getloadavg()[0]/os.cpu_count()
        t=time.time()
        p=subprocess.run(["nice","-n","10","repo","test-file","--json","--",f],cwd=root,env=env,capture_output=True,text=True,timeout=900)
        wall=time.time()-t
        try:
            doc=json.loads(p.stdout)
            rows=doc["files"]; sec=sum(r["seconds"] for r in rows); ran={r.get("ranOn") for r in rows}; st=doc["status"]
        except Exception as e:
            sec=None; ran=None; st="error:"+str(e)[:80]
        rec={"file":f,"mode":mode,"loadPerCoreAtStart":round(load,2),"wall":round(wall,1),"testSeconds":sec,"ranOn":sorted(x for x in ran if x) if ran else ran,"status":st,"at":time.strftime("%H:%M:%S")}
        open(out,"a").write(json.dumps(rec,ensure_ascii=False)+"\n"); print(rec,flush=True)
