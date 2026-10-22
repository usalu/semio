#!/usr/bin/env python3
"""📊️ Inclusive time (ms) of named functions in a .cpuprofile, counting each sample once per function. usage: profinc.py <file> <function>..."""
import json, sys, collections
d = json.load(open(sys.argv[1]))
nodes = {x["id"]: x for x in d["nodes"]}; parent = {}
for x in d["nodes"]:
    for c in x.get("children", []): parent[c] = x["id"]
want = set(sys.argv[2:]); inc = collections.Counter(); total = 0
for sample, delta in zip(d["samples"], d["timeDeltas"]):
    total += delta; seen = set(); nid = sample
    while nid is not None:
        name = nodes[nid]["callFrame"]["functionName"]
        if name in want and name not in seen: seen.add(name); inc[name] += delta
        nid = parent.get(nid)
print("total ms", total // 1000)
for name in sys.argv[2:]: print(f"{inc[name]//1000:8d} ms  {name}")
