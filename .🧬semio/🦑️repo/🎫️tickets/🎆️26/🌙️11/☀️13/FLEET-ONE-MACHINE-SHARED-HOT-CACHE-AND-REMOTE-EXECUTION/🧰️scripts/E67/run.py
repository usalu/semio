#!/usr/bin/env python3
import json,os,subprocess,sys,time,re
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/3a99932f-dbc8-44a3-a665-189b4162729c/scratchpad/E67"
ROOT="/Users/ueli/Documents/semio"
mode=sys.argv[1]; out=SC+"/"+sys.argv[2]; limit=float(sys.argv[3]) if len(sys.argv)>3 else 500
T=os.path.dirname(os.path.abspath(__file__))
if mode=="open":
    latest={}
    for p in (T+"/../E31/results-E31.jsonl",):
        for l in open(p):
            r=json.loads(l); latest[r["file"]]=r
    files=[f for f,r in latest.items() if r["status"]!="passed"]
else:
    files=[l.strip() for l in open(sys.argv[4]) if l.strip()]
done=set()
if os.path.exists(out):
    for l in open(out): done.add(json.loads(l)["file"])
todo=[f for f in files if f not in done]
start=time.time()
def memfree():
    try: return int(re.search(r"(\d+)%",subprocess.run(["memory_pressure"],capture_output=True,text=True).stdout.split("free percentage:")[1]).group(1))
    except Exception: return 50
for f in todo:
    if time.time()-start>limit: break
    while (os.getloadavg()[0]>=90 or memfree()<25) and time.time()-start<limit: time.sleep(30)
    if time.time()-start>limit: break
    r=subprocess.run(["repo","test-file","--json","--",f],capture_output=True,text=True,cwd=ROOT)
    try:
        t=r.stdout;i=t.index('{\n  "version": 1');d=json.JSONDecoder(strict=False).raw_decode(t[i:])[0]
        x=d["files"][0]
        row={"file":f,"status":x["status"],"pass":x.get("pass"),"fail":x.get("fail"),"seconds":x.get("seconds"),"error":(x.get("error") or "")[:300],"tail":(x.get("tail") or "")[-2500:] if x["status"]!="passed" else ""}
    except Exception:
        row={"file":f,"status":"refused","error":(r.stderr or r.stdout)[-400:]}
    open(out,"a").write(json.dumps(row)+"\n")
    print(row["status"],row.get("pass"),row.get("fail"),f.split("build/")[-1],flush=True)
print("left",len([f for f in todo if f not in set(json.loads(l)["file"] for l in open(out))]))
