import glob,json,os,sys,time
since=float(sys.argv[1]) if len(sys.argv)>1 else 0
pat=sys.argv[2:] or ["sourcing"]
rows=[]
for f in glob.glob(os.path.join(os.path.dirname(__file__),"../../../../../../../.cache/b/r/m/u/x/*.jsonl")):
    for line in open(f,errors="replace"):
        try:r=json.loads(line)
        except ValueError:continue
        if r.get("kind")=="miss" and r.get("peakRssBytes") and r.get("atMs",0)/1000>=since and any(p in r.get("crate","") for p in pat):
            rows.append((r["peakRssBytes"]//1048576,r["crate"],r.get("milliseconds",0)//1000,time.strftime("%H:%M:%S",time.localtime(r["atMs"]/1000)),r.get("key","")[:60]))
for r in sorted(rows,reverse=True):print(*r)
