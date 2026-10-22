#!/usr/bin/env python3
"""📊️ Summarizes a .cpuprofile: self time by function (top N) and by file. usage: profsum.py <file> [N]"""
import json, sys, collections
d = json.load(open(sys.argv[1]))
n = int(sys.argv[2]) if len(sys.argv) > 2 else 30
nodes = {x["id"]: x for x in d["nodes"]}
self_us = collections.Counter()
deltas = d["timeDeltas"]
for sample, delta in zip(d["samples"], deltas):
    self_us[sample] += delta
by_fn = collections.Counter(); by_file = collections.Counter()
total = sum(self_us.values())
for nid, us in self_us.items():
    cf = nodes[nid]["callFrame"]
    name = cf["functionName"] or "(anon)"
    url = cf["url"].split("/")[-1] if cf["url"] else "(native)"
    by_fn[(name, url, cf.get("lineNumber"))] += us
    by_file[url] += us
print("total ms", total // 1000)
for (name, url, line), us in by_fn.most_common(n):
    print(f"{us//1000:8d} ms {100*us/total:5.1f}%  {name} {url}:{line}")
print("--- by file")
for url, us in by_file.most_common(12):
    print(f"{us//1000:8d} ms {100*us/total:5.1f}%  {url}")
if len(sys.argv) > 3:
    parent = {}
    for x in d["nodes"]:
        for c in x.get("children", []): parent[c] = x["id"]
    def chain(nid, depth=6):
        out = []
        while nid in parent and len(out) < depth:
            nid = parent[nid]; cf = nodes[nid]["callFrame"]
            out.append((cf["functionName"] or "(anon)") + ":" + (cf["url"].split("/")[-1] if cf["url"] else "") + ":" + str(cf.get("lineNumber")))
        return out
    for target in sys.argv[3:]:
        agg = collections.Counter()
        for nid, us in self_us.items():
            if nodes[nid]["callFrame"]["functionName"] == target:
                agg[" < ".join(chain(nid, 5))] += us
        print("=== callers of", target)
        for key, us in agg.most_common(6): print(f"{us//1000:7d} ms  {key[:260]}")
