#!/usr/bin/env python3
"""cpuprof-top.py file.cpuprofile [N] : self time per source file and per function from a Chrome/Bun cpuprofile."""
import json,sys,collections
p=json.load(open(sys.argv[1]));n=int(sys.argv[2]) if len(sys.argv)>2 else 20
nodes={x["id"]:x for x in p["nodes"]};self_us=collections.Counter()
dt=p["timeDeltas"];samples=p["samples"]
for s,d in zip(samples,dt):self_us[s]+=d
byurl=collections.Counter();byfn=collections.Counter()
for i,us in self_us.items():
    cf=nodes[i]["callFrame"];u=cf["url"] or "(native/"+cf["functionName"]+")";byurl[u]+=us;byfn[(cf["functionName"] or "(anon)",u.split("/")[-2] if "/" in u else u,cf.get("lineNumber"))]+=us
tot=sum(self_us.values());print(f"samples={len(samples)} total={tot/1000:.1f}ms")
print("by file");[print(f"{us/1000:8.1f} ms  {'/'.join(u.split('/')[-3:])}") for u,us in byurl.most_common(n)]
print("by function");[print(f"{us/1000:8.1f} ms  {k[0]} {k[1]}:{k[2]}") for k,us in byfn.most_common(n)]
