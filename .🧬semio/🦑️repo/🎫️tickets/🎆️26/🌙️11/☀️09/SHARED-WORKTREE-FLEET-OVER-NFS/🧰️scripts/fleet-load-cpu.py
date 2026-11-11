#!/usr/bin/env python3
"""usage: fleet-load-cpu.py RUNDIR T0 T1 -- CPU (% of one core) per daemon and per relay between sample times T0 and T1 (the load window), RSS mean/max, applied events from the log counts are not included."""
import csv, collections, json, statistics, sys
run, t0, t1 = sys.argv[1], float(sys.argv[2]), float(sys.argv[3])
rows = collections.defaultdict(list)
for r in csv.DictReader(open(run + "/sample/samples.csv")):
    t = float(r["t"])
    if t0 <= t <= t1: rows[r["name"]].append((t, int(r["rss_kb"]) / 1024, float(r["cpu_s"])))
out = {}
for label, sel in (("daemon", lambda n: n[0] == "g" and "-" in n), ("relay", lambda n: n.startswith("relay"))):
    c = []; m = []; mx = []
    for n, v in rows.items():
        if sel(n) and len(v) > 3 and v[-1][0] - v[0][0] > 0.5 * (t1 - t0): c.append(100 * (v[-1][2] - v[0][2]) / (v[-1][0] - v[0][0])); m.append(statistics.mean(x[1] for x in v)); mx.append(max(x[1] for x in v))
    if not c: continue
    c.sort(); out[label] = {"n": len(c), "cpu_median": round(statistics.median(c), 1), "cpu_p95": round(c[int(.95 * len(c))], 1), "cpu_max": round(c[-1], 1), "cpu_total_cores": round(sum(c) / 100, 2), "rss_mean_mib": round(statistics.mean(m)), "rss_max_mib": round(max(mx)), "rss_sum_of_means_gib": round(sum(m) / 1024, 2)}
print(json.dumps(out))
