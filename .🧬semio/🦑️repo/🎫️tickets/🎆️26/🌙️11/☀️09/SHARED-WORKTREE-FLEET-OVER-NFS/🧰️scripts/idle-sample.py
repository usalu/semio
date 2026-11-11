#!/usr/bin/env python3
"""usage: idle-sample.py SECONDS NAME=PID... -- sample cputime and rss of the given pids every 0.5 s; print per pid the mean CPU % of one core over the window, the rss (MiB) at start/end/max, and the CPU bursts (consecutive samples above 50 %) with their duration and CPU seconds."""
import json, subprocess, sys, time

def sample(pid):
    out = subprocess.run(["ps", "-o", "cputime=,rss=", "-p", str(pid)], capture_output=True, text=True).stdout.split()
    if len(out) < 2: return None
    t = out[0]; parts = t.split(":")
    secs = float(parts[-1]) + 60 * int(parts[-2]) + (3600 * int(parts[-3]) if len(parts) > 2 else 0)
    return secs, int(out[1]) / 1024

seconds = float(sys.argv[1]); procs = dict(a.split("=") for a in sys.argv[2:])
rows = {n: [] for n in procs}
t0 = time.monotonic()
while time.monotonic() - t0 < seconds:
    now = time.monotonic() - t0
    for n, p in procs.items():
        s = sample(p)
        if s: rows[n].append((now, s[0], s[1]))
    time.sleep(0.5)
res = {}
for n, r in rows.items():
    cpu = r[-1][1] - r[0][1]; wall = r[-1][0] - r[0][0]
    bursts = []; cur = None
    for i in range(1, len(r)):
        rate = (r[i][1] - r[i-1][1]) / max(1e-6, r[i][0] - r[i-1][0])
        if rate > 0.5:
            if cur is None: cur = [r[i-1][0], r[i][1] - r[i-1][1]]
            else: cur[1] += r[i][1] - r[i-1][1]
            cur_end = r[i][0]
        elif cur is not None:
            bursts.append({"at_s": round(cur[0]), "wall_s": round(cur_end - cur[0], 1), "cpu_s": round(cur[1], 1)}); cur = None
    if cur is not None: bursts.append({"at_s": round(cur[0]), "wall_s": round(cur_end - cur[0], 1), "cpu_s": round(cur[1], 1)})
    res[n] = {"cpu_percent_of_one_core": round(100 * cpu / wall, 1), "cpu_s": round(cpu, 1), "rss_mib_start": round(r[0][2]), "rss_mib_end": round(r[-1][2]), "rss_mib_max": round(max(x[2] for x in r)), "bursts": bursts}
print(json.dumps(res))
