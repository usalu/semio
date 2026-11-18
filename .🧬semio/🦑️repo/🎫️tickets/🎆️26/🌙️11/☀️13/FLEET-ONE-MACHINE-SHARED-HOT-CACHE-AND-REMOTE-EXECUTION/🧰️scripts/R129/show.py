import json,sys
for l in open(sys.argv[1]):
    if l.startswith("{"):
        d=json.loads(l); print(d["label"][:16].ljust(16),d.get("kind","")[-6:].ljust(6),"s",str(d.get("seconds","")).ljust(5),"rss",d.get("rss"),"fp",d.get("footprint"),"shrink",d.get("afterShrinkRss"),"heap",d.get("heapUsed"),"ext",d.get("external"),"ab",d.get("arrayBuffers"),"trim",d.get("trimmed"),(d.get("caches") or {}).get("totalMb"))
    elif "peak memory footprint" in l or "maximum resident" in l: print(l.strip())
