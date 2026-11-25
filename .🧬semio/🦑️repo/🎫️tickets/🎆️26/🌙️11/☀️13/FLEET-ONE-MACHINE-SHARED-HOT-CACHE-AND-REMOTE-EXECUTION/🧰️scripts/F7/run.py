#!/usr/bin/env python3
import json,os,subprocess,sys,time,re
SC="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/F7"
os.makedirs(SC,exist_ok=True)
ROOT="/Users/ueli/Documents/semio"
lst,outn,limit=sys.argv[1],SC+"/"+sys.argv[2],float(sys.argv[3])
files=[l.strip() for l in open(lst) if l.strip()]
done=set(json.loads(l)["file"] for l in open(outn)) if os.path.exists(outn) else set()
start=time.time()
def memfree():
    try: return int(re.search(r"(\d+)%",subprocess.run(["memory_pressure"],capture_output=True,text=True).stdout.split("free percentage:")[1]).group(1))
    except Exception: return 50
for f in files:
    if f in done: continue
    while (os.getloadavg()[0]>=40 or memfree()<25) and time.time()-start<limit: time.sleep(20)
    if time.time()-start>limit: break
    try:
        r=subprocess.run(["repo","test-file","--json","--",f],capture_output=True,text=True,cwd=ROOT,timeout=max(60,590-(time.time()-start)))
        t=r.stdout;i=t.index('{\n  "version": 1');x=json.JSONDecoder(strict=False).raw_decode(t[i:])[0]["files"][0]
        row={"file":f,"status":x["status"],"pass":x.get("pass"),"fail":x.get("fail"),"seconds":x.get("seconds"),"error":(x.get("error") or "")[:300],"tail":(x.get("tail") or "")[-2000:] if x["status"]!="passed" else ""}
    except subprocess.TimeoutExpired: row={"file":f,"status":"timeout"}
    except Exception: row={"file":f,"status":"refused","error":(r.stderr or r.stdout)[-400:]}
    open(outn,"a").write(json.dumps(row,ensure_ascii=False)+"\n")
    print(row["status"],row.get("pass"),row.get("fail"),row.get("seconds"),f.split("library/")[-1][-80:],flush=True)
