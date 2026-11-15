#!/usr/bin/env python3
"""keyparts.py: for the miss events of this machine, groups the keys of one crate by their source digest; keys of equal sources that differ say WHICH key component differs (toolchain, args, externs, environment)."""
import json,glob,collections,re
U="/Users/Shared/semio/.cache/b/r/m/u/x"
by=collections.defaultdict(dict)
n=0
for f in glob.glob(U+"/*.jsonl"):
    for l in open(f,errors="ignore"):
        try:r=json.loads(l)
        except ValueError:continue
        if r["kind"]!="miss" or not r.get("reason"):continue
        m=re.match(r"toolchain:(\w+) args:(\w+) source:(\w+) externs:(\w+) environment:(\w+)",r["reason"])
        if not m:continue
        n+=1;by[(r["crate"],m.group(3))][r["key"]]=m.groups()
print("miss events with parts:",n,"crate/source groups:",len(by))
same=[g for g,k in by.items() if len(k)>1]
print("groups where equal source gave more than one key:",len(same))
diff=collections.Counter()
names=["toolchain","args","source","externs","environment"]
for g in same:
    rows=list(by[g].values())
    d=tuple(names[i] for i in range(5) if len({r[i] for r in rows})>1)
    diff[d]+=1
for d,v in diff.most_common():print(v,"differs in",d)
print("crates whose misses repeat with equal source:",len({g[0] for g in same}))
