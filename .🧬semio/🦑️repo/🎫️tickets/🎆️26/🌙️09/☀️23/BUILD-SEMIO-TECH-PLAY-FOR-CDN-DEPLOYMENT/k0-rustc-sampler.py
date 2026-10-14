#!/usr/bin/env python3
"""Samples, per root pid, live rustc processes in its tree (all, and only admitted compile units whose parent is the memory-admitting wrapper) until every root exits; prints per-root maxima as JSON."""
import json, subprocess, sys, time
roots = [int(pid) for pid in sys.argv[1:]]
peak = {pid: 0 for pid in roots}
units = {pid: 0 for pid in roots}
while True:
    rows = [line.split(None, 2) for line in subprocess.run(["ps", "-axo", "pid=,ppid=,comm="], capture_output=True, text=True).stdout.splitlines()]
    parent = {int(r[0]): int(r[1]) for r in rows if len(r) == 3}
    name = {int(r[0]): r[2] for r in rows if len(r) == 3}
    alive = [pid for pid in roots if pid in parent]
    if not alive:
        break
    for root in alive:
        count = 0
        admitted = 0
        for pid, command in name.items():
            if not command.endswith("rustc"):
                continue
            cursor = pid
            while cursor in parent and cursor not in (0, 1, root):
                cursor = parent[cursor]
            count += cursor == root
            admitted += cursor == root and name.get(parent.get(pid, 0), "").endswith("semio-build-scheduler")
        peak[root] = max(peak[root], count)
        units[root] = max(units[root], admitted)
    time.sleep(0.2)
print(json.dumps({"peakRustcPerLauncher": peak, "peakAdmittedUnitsPerLauncher": units}))
