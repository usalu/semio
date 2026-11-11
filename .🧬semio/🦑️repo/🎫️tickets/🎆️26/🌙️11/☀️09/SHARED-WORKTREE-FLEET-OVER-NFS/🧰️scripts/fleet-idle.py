#!/usr/bin/env python3
"""usage: fleet-idle.py SAMPLEDIR -- CPU (% of one core) and RSS of daemons and relays over the window of a fleet-sample.py directory."""
import csv, collections, statistics, sys
rows = collections.defaultdict(list)
for r in csv.DictReader(open(sys.argv[1] + "/samples.csv")): rows[r["name"]].append((float(r["t"]), int(r["rss_kb"]) / 1024, float(r["cpu_s"])))
d, rl, rs = [], [], []
for n, v in rows.items():
    if len(v) < 3: continue
    c = 100 * (v[-1][2] - v[0][2]) / (v[-1][0] - v[0][0])
    (rl if n.startswith("relay") else d).append(c)
    if not n.startswith("relay") and n[0] == "g": rs.append(v[-1][1])
d.sort()
print("daemons", len(d), "cpu median %.2f p95 %.2f max %.2f sum %.0f" % (statistics.median(d), d[int(.95 * len(d))], max(d), sum(d)), "rss mean %.0f max %.0f sum %.0f" % (statistics.mean(rs), max(rs), sum(rs)))
print("relays", [round(x, 2) for x in rl])
