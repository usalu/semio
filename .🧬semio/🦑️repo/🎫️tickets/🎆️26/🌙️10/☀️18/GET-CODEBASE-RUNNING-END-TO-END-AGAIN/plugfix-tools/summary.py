#!/usr/bin/env python3
"""📊 plugfix summary: per crate, the distinct findings of one kind from classify.py output. usage: summary.py <classified.txt> KIND..."""
import collections, re, sys
cur = None
agg = collections.defaultdict(lambda: collections.defaultdict(lambda: [0, set(), set()]))
kinds = set(sys.argv[2:])
for l in open(sys.argv[1], encoding="utf-8"):
    if l.startswith("## "):
        cur = l[3:].strip()
        continue
    m = re.match(r"\s+(\w+)\s+(E\d+)\s+(.*?) x(\d+) \[(\w+)\] @ (.*)$", l)
    if m and m.group(1) in kinds:
        k, code, msg, n, cfg, where = m.groups()
        a = agg[cur][(k, code, msg[:100])]
        a[0] += int(n)
        a[1].add(cfg)
        a[2].add(where[-70:])
for c, d in agg.items():
    print("##", c[-90:])
    for (k, code, msg), (n, cfgs, ws) in sorted(d.items()):
        print(f"   {k} {code} {msg} x{n} {sorted(cfgs)} {sorted(ws)[:2]}")
