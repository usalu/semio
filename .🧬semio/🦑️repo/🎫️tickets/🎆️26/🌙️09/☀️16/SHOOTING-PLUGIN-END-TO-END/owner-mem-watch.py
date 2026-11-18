import json,time,sys
out=sys.argv[1];last=None
while True:
    try:
        d=json.load(open("/Users/ueli/Documents/semio/.cache/f/slice-memory.json"))
    except Exception: time.sleep(5);continue
    if d["atMs"]!=last:
        last=d["atMs"]
        top=sorted(d["collections"],key=lambda c:-c["estimatedBytes"])[:6]
        o=d["sources"].get("owners",{}).get("ledger",{})
        line=json.dumps({"t":d["atMs"],"pid":d["pid"],"phase":d["phase"],"stage":d["stage"],"heapMB":d["heap"]["heapMegabytes"],"extraMB":d["heap"]["extraMegabytes"],"objs":d["heap"]["objectCount"],"done":o.get("done"),"top":[(c["name"][:40],c["entries"],round(c["estimatedBytes"]/1048576)) for c in top]})
        open(out,"a").write("[DEBUG] "+line+"\n")
    time.sleep(10)
