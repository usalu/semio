#!/usr/bin/env python3
"""usage: fleet-timeline.py RUNDIR [WINDOW_S] -- probe latency (small files) per time window since the first probe: n, median, p95, max, timeouts, split in-group / cross-group."""
import json, sys
run = sys.argv[1]; w = int(sys.argv[2]) if len(sys.argv) > 2 else 30
rs = [json.loads(l) for l in open(run + "/probe-results.jsonl")]
t0 = min(r["t"] for r in rs) - 5
pct = lambda v, p: v[min(len(v) - 1, int(p * len(v)))]
print("window  in: n med p95 max to | cross: n med p95 max to")
for lo in range(0, int(max(r["t"] for r in rs) - t0) + 1, w):
    row = []
    for sc in ("in", "cross"):
        sel = [r for r in rs if r["scope"] == sc and not r["big"] and lo <= r["t"] - t0 < lo + w]
        v = sorted(r["ms"] for r in sel if r["ms"] is not None)
        row.append(f"{len(v):4d} {pct(v, .5):6d} {pct(v, .95):6d} {v[-1]:6d} {len(sel) - len(v):3d}" if v else "  -")
    print(f"{lo:5d}  {row[0]} | {row[1]}")
