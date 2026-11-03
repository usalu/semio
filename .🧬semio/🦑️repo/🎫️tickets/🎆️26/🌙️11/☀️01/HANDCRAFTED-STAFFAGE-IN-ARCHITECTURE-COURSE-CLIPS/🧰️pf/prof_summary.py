"""📊 Summarise a V8/JSC `.cpuprofile`: self and inclusive time per function (top N).

@see ../pf-report.md — the performance report
"""

import json
import sys
from collections import defaultdict

profile = json.load(open(sys.argv[1], encoding="utf-8"))
top = int(sys.argv[2]) if len(sys.argv) > 2 else 25
nodes = {n["id"]: n for n in profile["nodes"]}
parent = {}
for n in profile["nodes"]:
    for c in n.get("children", []):
        parent[c] = n["id"]
delta = profile["timeDeltas"]
self_us = defaultdict(float)
for sample, d in zip(profile["samples"], delta):
    self_us[sample] += d
key = lambda n: f'{n["callFrame"]["functionName"] or "(anon)"} {n["callFrame"]["url"].split("/")[-1][-40:]}:{n["callFrame"]["lineNumber"]}'
by_self = defaultdict(float)
by_incl = defaultdict(float)
for nid, us in self_us.items():
    by_self[key(nodes[nid])] += us
    seen = set()
    cur = nid
    while cur is not None:
        k = key(nodes[cur])
        if k not in seen:
            by_incl[k] += us
            seen.add(k)
        cur = parent.get(cur)
total = sum(self_us.values())
print(f"total {total / 1e6:.1f}s")
print("-- self")
for k, v in sorted(by_self.items(), key=lambda x: -x[1])[:top]:
    print(f"{v / 1e6:8.2f}s {k}")
print("-- inclusive")
for k, v in sorted(by_incl.items(), key=lambda x: -x[1])[:top]:
    print(f"{v / 1e6:8.2f}s {k}")
