#!/usr/bin/env python3
"""Independent model of the Nx put slot schedule: N workers arrive in order at S slots, each holds its slot for H ms; prints the start order (first in, first out) and the peak concurrency. usage: oracle.py SLOTS WORKERS"""
import heapq, json, sys

slots, workers = int(sys.argv[1]), int(sys.argv[2])
free = [0] * slots
heapq.heapify(free)
starts, peak, active = [], 0, []
for worker in range(workers):
    start = heapq.heappop(free)
    starts.append(worker)
    heapq.heappush(free, start + 1)
    active.append((start, start + 1))
for time in range(workers + 1):
    peak = max(peak, sum(1 for begin, end in active if begin <= time < end))
print(json.dumps({"order": starts, "peak": peak}))
