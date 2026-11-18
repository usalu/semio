import json,glob,os,sys,time
O="/Users/Shared/semio-store/v1/origins"
def read():
    out={}
    for f in sorted(glob.glob(O+"/*/slice-memory.json")):
        o=f.split("/")[-2]
        try:d=json.load(open(f))
        except Exception:continue
        s=d.get("sources",{}).get("owners",{}) or {}
        w=s.get("warm") or {}
        m=d.get("memory",{}) or {}
        out[o]=dict(age=round(time.time()-os.path.getmtime(f)),pid=d.get("pid"),up=d.get("uptimeSeconds"),stage=(d.get("stage") or {}).get("stage") if isinstance(d.get("stage"),dict) else d.get("stage"),taken=s.get("taken"),jobs=s.get("jobs"),warm=(w.get("done"),w.get("total"),w.get("settled"),w.get("durable"),w.get("failed"),w.get("lanes"),w.get("peak")) if w else None,tree=d.get("treeFootprintMegabytes"),rss=(d.get("stage") or {}).get("residentMegabytes") if isinstance(d.get("stage"),dict) else None)
    return out
n=int(sys.argv[1]) if len(sys.argv)>1 else 1;gap=float(sys.argv[2]) if len(sys.argv)>2 else 30
prev=None
for i in range(n):
    cur=read();t=time.time()
    for o,r in cur.items():
        rate=""
        if prev and o in prev[0] and prev[0][o]["pid"]==r["pid"]:
            dt=t-prev[1];p=prev[0][o]
            if r["warm"] and p["warm"] and r["warm"][0] is not None and p["warm"][0] is not None:rate+=" warm/s=%.2f"%((r["warm"][0]-p["warm"][0])/dt)
            if r["taken"] is not None and p["taken"] is not None:rate+=" taken/s=%.2f"%((r["taken"]-p["taken"])/dt)
        print(time.strftime("%H:%M:%S"),o,r,rate,flush=True)
    prev=(cur,t)
    if i<n-1:time.sleep(gap)
