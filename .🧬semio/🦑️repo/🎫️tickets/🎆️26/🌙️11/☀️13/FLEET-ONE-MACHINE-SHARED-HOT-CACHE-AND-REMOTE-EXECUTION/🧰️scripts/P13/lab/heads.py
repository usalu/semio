#!/usr/bin/env python3
"""Read-only: lists the generation heads stored in the production cache (generation, owners, who wrote it, state time, stale count, upload time and age). usage: heads.py [cache-server-v1-dir]"""
import os,sys,re,json,time
base=sys.argv[1] if len(sys.argv)>1 else "/Users/Shared/semio/.cache/f/cache-server/v1"
rows=[]
for inst in sorted(os.listdir(base)):
    root=f"{base}/{inst}/ac"
    if not os.path.isdir(root) or inst.startswith("."):continue
    for d,_,fs in os.walk(root):
        for f in fs:
            try:b=open(os.path.join(d,f),"rb").read()
            except OSError:continue
            m=re.search(rb'\{"version":1,"generation":\d+.*?\}(?=\Z|[^\w"])',b)
            if not m or b'"signature"' not in m.group(0):continue
            try:n=json.loads(m.group(0))
            except ValueError:continue
            rows.append((n["publishedMs"],inst,f[:10],n))
now=time.time()*1000
for pub,inst,key,n in sorted(rows):
    state=n.get("stateMs");print(time.strftime("%m-%d %H:%M:%SZ",time.gmtime(pub/1000)),inst,key,"gen",n["generation"],"owners",n["owners"],"origin",n.get("origin","-"),"state_age_min",("%.1f"%((now-state)/60000) if state else "-"),"stale",n.get("stale","unmeasured" if state else "-"),"upload_age_min %.1f"%((now-pub)/60000))
