#!/usr/bin/env python3
"""Summarises a .cpuprofile (V8 format): self time by function (top N) and by url prefix. usage: cpuprof.py file [N]"""
import json, sys, collections
p = json.load(open(sys.argv[1])); n = int(sys.argv[2]) if len(sys.argv) > 2 else 25
nodes = {x["id"]: x for x in p["nodes"]}
dt = p.get("timeDeltas", []); samples = p.get("samples", [])
self_us = collections.Counter()
for s, d in zip(samples, dt):
    self_us[s] += d
total = sum(self_us.values())
byfn = collections.Counter(); byurl = collections.Counter()
for nid, us in self_us.items():
    cf = nodes[nid]["callFrame"]; url = cf["url"].replace("/Users/ueli/Documents/semio/", "")[-90:]
    byfn[(cf["functionName"] or "(anon)", url, cf["lineNumber"])] += us
    byurl[url] += us
print(f"total {total/1000:.0f} ms over {len(samples)} samples")
for (f, u, l), us in byfn.most_common(n): print(f"{us/1000:8.1f} ms  {f}  {u}:{l}")
print("-- by url")
for u, us in byurl.most_common(12): print(f"{us/1000:8.1f} ms  {u}")
