#!/usr/bin/env python3
"""usage: fleet-summary.py RUNDIR... -- one JSON line per run: ops, probe latency (small files, ms: median/p95/p99/max) in-group and cross-group before the first fault and over the whole run, big-file latency, daemon/relay CPU and RSS, store growth, ops/s, event timeline."""
import csv, json, os, statistics, sys
pct = lambda v, p: v[min(len(v) - 1, int(p * len(v)))]
for run in sys.argv[1:]:
    ev = json.load(open(run + "/events.json")) if os.path.exists(run + "/events.json") else []
    ops = json.load(open(run + "/ops.json")) if os.path.exists(run + "/ops.json") else {"total": None}
    rs = [json.loads(l) for l in open(run + "/probe-results.jsonl")]
    t0 = min(r["t"] for r in rs) - 3
    first_fault = min([e["t"] for e in ev if e["what"].startswith(("kill9", "store-away"))] or [168.1]) - 5
    out = {"run": os.path.basename(run.rstrip("/")), "ops": ops["total"], "first_fault_s": first_fault}
    for label, sel in (("pre_fault", lambda r: r["t"] - t0 < first_fault), ("whole", lambda r: True)):
        for scope in ("in", "cross"):
            v = sorted(r["ms"] for r in rs if r["scope"] == scope and not r["big"] and sel(r) and r["ms"] is not None)
            miss = sum(1 for r in rs if r["scope"] == scope and not r["big"] and sel(r) and r["ms"] is None)
            if v: out[f"{label}_{scope}"] = {"n": len(v), "timeouts": miss, "med": pct(v, .5), "p95": pct(v, .95), "p99": pct(v, .99), "max": v[-1]}
    for scope in ("in", "cross"):
        v = sorted(r["ms"] for r in rs if r["scope"] == scope and r["big"] and r["ms"] is not None)
        if v: out[f"big_{scope}"] = {"n": len(v), "med": pct(v, .5), "max": v[-1], "sizes_mb": sorted({r["n"] >> 20 for r in rs if r["big"]})}
    print(json.dumps(out))
