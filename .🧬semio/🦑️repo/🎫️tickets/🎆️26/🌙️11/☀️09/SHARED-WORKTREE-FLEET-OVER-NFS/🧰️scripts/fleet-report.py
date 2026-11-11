#!/usr/bin/env python3
"""usage: fleet-report.py RUNDIR [T0 T1] -- per-process CPU (% of one core) and RSS over the window from samples.csv, probe latency percentiles, store growth, event counts of the logs."""
import collections, csv, glob, json, os, statistics, sys
run = sys.argv[1]; t0 = float(sys.argv[2]) if len(sys.argv) > 3 else 0; t1 = float(sys.argv[3]) if len(sys.argv) > 3 else 1e9
rows = collections.defaultdict(list)
for r in csv.DictReader(open(run + "/sample/samples.csv")):
    t = float(r["t"])
    if t0 <= t <= t1: rows[r["name"]].append((t, int(r["rss_kb"]) / 1024, float(r["cpu_s"])))
def pct(v, p): v = sorted(v); return v[min(len(v) - 1, int(p * len(v)))]
cpu = {}; rss = {}
for n, r in rows.items():
    if len(r) > 2: cpu[n] = 100 * (r[-1][2] - r[0][2]) / max(1, r[-1][0] - r[0][0]); rss[n] = (statistics.mean(x[1] for x in r), max(x[1] for x in r))
out = {}
for label, sel in (("daemon", lambda n: "-" in n and not n.startswith("relay")), ("relay", lambda n: n.startswith("relay"))):
    ns = [n for n in cpu if sel(n)]
    if ns:
        c = [cpu[n] for n in ns]; m = [rss[n][1] for n in ns]; a = [rss[n][0] for n in ns]
        out[label] = {"n": len(ns), "cpu_pct_median": round(statistics.median(c), 1), "cpu_pct_p95": round(pct(c, .95), 1), "cpu_pct_max": round(max(c), 1), "cpu_pct_sum": round(sum(c)), "rss_mib_mean": round(statistics.mean(a)), "rss_mib_max": round(max(m)), "rss_mib_sum_max": round(sum(m))}
print(json.dumps(out))
rs = [json.loads(l) for l in open(run + "/probe-results.jsonl")]
for scope in ("in", "cross"):
    for big in (False, True):
        sel = [r for r in rs if r["scope"] == scope and r["big"] == big]
        v = sorted(r["ms"] for r in sel if r["ms"] is not None)
        if v: print(scope, "big" if big else "small", json.dumps({"n": len(v), "timeouts": len(sel) - len(v), "median": v[len(v) // 2], "p95": pct(v, .95), "p99": pct(v, .99), "max": v[-1]}))
