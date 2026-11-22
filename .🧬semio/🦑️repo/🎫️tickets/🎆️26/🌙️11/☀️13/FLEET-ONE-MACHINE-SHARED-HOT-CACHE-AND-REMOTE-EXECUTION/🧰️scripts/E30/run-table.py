#!/usr/bin/env python3
import json,os,subprocess,sys,time
S="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-kfaIfm/ccee8eb5-3363-4a84-af9e-53e38d236627/scratchpad/E30"
ROOT="/Users/ueli/Documents/semio"
limit=float(sys.argv[1]) if len(sys.argv)>1 else 480
size=int(sys.argv[2]) if len(sys.argv)>2 else 10
from_index=int(sys.argv[3]) if len(sys.argv)>3 else 0
name=sys.argv[4] if len(sys.argv)>4 else "results"
files=[f for f in open(S+"/all.txt").read().split("\n") if f][from_index:]
done=set()
out=S+"/"+name+".jsonl"
for n in os.listdir(S):
    if n.endswith(".jsonl"):
        for line in open(S+"/"+n):
            done.add(json.loads(line)["file"])
skip=set(l.strip() for l in open(S+"/skip.txt")) if os.path.exists(S+"/skip.txt") else set()
todo=[f for f in files if f not in done and f not in skip]
start=time.time()
while todo and time.time()-start<limit:
    while os.getloadavg()[0]>45 and time.time()-start<limit: time.sleep(20)
    if time.time()-start>=limit: break
    batch,todo=todo[:size],todo[size:]
    r=subprocess.run(["repo","test-file","--json","--"]+batch,capture_output=True,text=True,cwd=ROOT)
    try: d=json.loads(r.stdout)
    except Exception: 
        with open(out,"a") as o:
            for f in batch: o.write(json.dumps({"file":f,"status":"refused","error":(r.stderr or r.stdout)[-400:]})+"\n")
        continue
    with open(out,"a") as o:
        for f in d["files"]:
            o.write(json.dumps({"file":f["file"],"suite":f.get("suite"),"status":f["status"],"pass":f.get("pass"),"fail":f.get("fail"),"seconds":f.get("seconds"),"error":(f.get("error") or "")[:300],"tail":(f.get("tail") or "")[-1500:] if f["status"]=="failed" else ""})+"\n")
print("remaining",len(todo),"load",os.getloadavg()[0])
