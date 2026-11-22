#!/usr/bin/env python3
import json,os,subprocess,sys,time
SC="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-kfaIfm/ccee8eb5-3363-4a84-af9e-53e38d236627/scratchpad"
S=SC+"/E31"
T=os.path.dirname(os.path.abspath(__file__))
ROOT="/Users/ueli/Documents/semio"
limit=float(sys.argv[1]) if len(sys.argv)>1 else 480
size=int(sys.argv[2]) if len(sys.argv)>2 else 6
name=sys.argv[3] if len(sys.argv)>3 else "table"
recent=float(os.environ.get("RECENT_MINUTES","60"))
files=[f for f in open(T+"/../E30/all-test-files.txt").read().split("\n") if f]
done=set()
for d in (SC+"/E30",S):
    for n in os.listdir(d):
        if n.endswith(".jsonl"):
            for line in open(d+"/"+n):
                try:
                    r=json.loads(line)
                    if r.get("status")!="refused": done.add(r["file"])
                except Exception: pass
def runnable(row): return row.get("status")!="refused"
for line in open(T+"/../E30/results-part1.jsonl"):
    done.add(json.loads(line)["file"])
out=S+"/"+name+".jsonl"
def fresh(f):
    try: return time.time()-os.stat(ROOT+"/"+f).st_mtime<recent*60
    except OSError: return True
todo=[f for f in files if f not in done and not fresh(f)]
if os.environ.get("REVERSE"): todo.reverse()
skipped=[f for f in files if f not in done and fresh(f)]
open(S+"/skipped-recent.txt","w").write("\n".join(skipped)+"\n")
start=time.time()
while todo and time.time()-start<limit:
    while os.getloadavg()[0]>45 and time.time()-start<limit: time.sleep(20)
    if time.time()-start>=limit: break
    batch,todo=todo[:size],todo[size:]
    r=subprocess.run(["repo","test-file","--json","--"]+batch,capture_output=True,text=True,cwd=ROOT)
    try:
        t=r.stdout;i=t.index('{\n  "version": 1');d=json.JSONDecoder(strict=False).raw_decode(t[i:])[0]
    except Exception:
        with open(out,"a") as o:
            for f in batch: o.write(json.dumps({"file":f,"status":"refused","error":(r.stderr or r.stdout)[-400:]})+"\n")
        continue
    with open(out,"a") as o:
        for f in d["files"]:
            o.write(json.dumps({"file":f["file"],"suite":f.get("suite"),"status":f["status"],"pass":f.get("pass"),"fail":f.get("fail"),"seconds":f.get("seconds"),"error":(f.get("error") or "")[:300],"tail":(f.get("tail") or "")[-1500:] if f["status"]=="failed" else ""})+"\n")
print("remaining",len(todo),"skipped-recent",len(skipped),"load",os.getloadavg()[0])
