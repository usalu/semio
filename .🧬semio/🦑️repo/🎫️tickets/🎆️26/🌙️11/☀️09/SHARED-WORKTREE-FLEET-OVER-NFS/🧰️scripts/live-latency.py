#!/usr/bin/env python3
"""usage: live-latency.py WRITER_TREE N TREE... -- edit one file in WRITER_TREE N times, poll every 50 ms until each TREE holds the same bytes, print per-tree latencies (ms), median and p95."""
import json, os, statistics, sys, time

writer, n, trees = sys.argv[1], int(sys.argv[2]), sys.argv[3:]
rel = "rehearsal-live/latency.txt"
os.makedirs(os.path.join(writer, os.path.dirname(rel)), exist_ok=True)
results = {t: [] for t in trees}
for i in range(n):
    data = ("edit %d %f %s\n" % (i, time.time(), os.urandom(8).hex())).encode()
    t0 = time.monotonic()
    with open(os.path.join(writer, rel), "wb") as f:
        f.write(data)
    pending = set(trees)
    while pending and time.monotonic() - t0 < 30:
        for t in list(pending):
            try:
                with open(os.path.join(t, rel), "rb") as f:
                    if f.read() == data:
                        results[t].append((time.monotonic() - t0) * 1000)
                        pending.discard(t)
            except OSError:
                pass
        time.sleep(0.05)
    for t in pending:
        results[t].append(None)
    time.sleep(1.0)
out = {}
for t, v in results.items():
    ok = sorted(x for x in v if x is not None)
    out[os.path.basename(t)] = {"n": len(v), "timeouts": len(v) - len(ok), "median_ms": round(statistics.median(ok)) if ok else None, "p95_ms": round(ok[min(len(ok) - 1, int(0.95 * len(ok)))]) if ok else None, "max_ms": round(max(ok)) if ok else None, "all_ms": [round(x) if x else None for x in v]}
print(json.dumps(out))
