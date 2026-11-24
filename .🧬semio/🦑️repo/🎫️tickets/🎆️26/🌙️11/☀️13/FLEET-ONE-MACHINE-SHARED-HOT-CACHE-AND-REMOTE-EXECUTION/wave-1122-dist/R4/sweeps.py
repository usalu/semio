# -*- coding: utf-8 -*-
import json,datetime
S="/Users/ueli/Documents/semio/.cache/b/p/m/fleet/cache-server/v1/.retention/"
rows=[]
for f in ("sweeps.jsonl.1","sweeps.jsonl"):
    try:
        for l in open(S+f,errors="replace"):
            try:rows.append(json.loads(l))
            except:pass
    except FileNotFoundError:pass
print(len(rows),"sweeps; retired total",sum(r.get("retired",0) for r in rows),"modes",set(r["mode"] for r in rows))
t=lambda ms:datetime.datetime.fromtimestamp(ms/1000).strftime("%H:%M")
prev=None
for r in rows:
    h=t(r["sweptAt"])[:4]
    if h!=prev and h[3] in "03":
        print(t(r["sweptAt"]),"entries",r["entries"],"declared",r["declared"],"live",r["live"],"unreached",r["unreached"],"refused",r.get("refused"))
    prev=h
print("--- windows")
for lo,hi in (("20:25","21:45"),("21:50","22:50")):
    last=None
    for r in rows:
        tt=t(r["sweptAt"])
        if lo<=tt<=hi:
            d="" if last is None else r["entries"]-last
            if last is None or abs(d)>600: print(tt,"entries",r["entries"],"delta",d)
            last=r["entries"]
