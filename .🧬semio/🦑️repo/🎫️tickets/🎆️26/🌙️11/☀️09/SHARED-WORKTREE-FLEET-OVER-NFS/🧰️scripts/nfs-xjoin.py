#!/usr/bin/env python3
"""usage: nfs-xjoin.py WRITER_LOG READER_LOG OFFSET_MS -- latency per edit = reader epoch - writer epoch_before_write - offset (offset = reader clock minus writer clock); prints n, misses, median, p95, max in ms and the sorted worst five."""
import json, statistics, sys

w = {}
for line in open(sys.argv[1]):
    i, t0, t1 = line.split(); w[int(i)] = float(t0)
r = {}
for line in open(sys.argv[2]):
    i, t = line.split(); r[int(i)] = float(t)
off = float(sys.argv[3]) / 1000
lat = sorted((r[i] - w[i] - off) * 1000 for i in w if i in r)
miss = [i for i in w if i not in r]
q = lambda p: lat[min(len(lat) - 1, int(p * len(lat)))]
print(json.dumps({"edits": len(w), "seen": len(lat), "missed": len(miss), "median_ms": round(statistics.median(lat)), "p95_ms": round(q(0.95)), "p99_ms": round(q(0.99)), "min_ms": round(lat[0]), "max_ms": round(lat[-1]), "worst5_ms": [round(x) for x in lat[-5:]]}))
