#!/usr/bin/env python3
"""usage: nfs-sample.py SECONDS NAME=PID... -- Linux /proc sampler: every 0.5 s utime+stime and VmRSS of each pid; prints per pid mean CPU % of one core, rss MiB at start/end/max and VmHWM."""
import json, os, sys, time

TICK = os.sysconf("SC_CLK_TCK")

def sample(pid):
    try:
        with open("/proc/%s/stat" % pid) as f:
            raw = f.read()
        fields = raw[raw.rindex(")") + 2:].split()
        ticks = int(fields[11]) + int(fields[12])
        rss = hwm = 0
        with open("/proc/%s/status" % pid) as f:
            for line in f:
                if line.startswith("VmRSS:"): rss = int(line.split()[1])
                elif line.startswith("VmHWM:"): hwm = int(line.split()[1])
        return ticks / TICK, rss / 1024, hwm / 1024
    except OSError:
        return None

seconds = float(sys.argv[1]); procs = dict(a.split("=") for a in sys.argv[2:])
rows = {n: [] for n in procs}
t0 = time.monotonic()
while time.monotonic() - t0 < seconds:
    now = time.monotonic() - t0
    for n, p in procs.items():
        s = sample(p)
        if s: rows[n].append((now,) + s)
    time.sleep(0.5)
out = {}
for n, r in rows.items():
    if len(r) < 2: out[n] = None; continue
    cpu = r[-1][1] - r[0][1]; wall = r[-1][0] - r[0][0]
    out[n] = {"cpu_percent_of_one_core": round(100 * cpu / wall, 2), "cpu_s": round(cpu, 2), "rss_mib_start": round(r[0][2]), "rss_mib_end": round(r[-1][2]), "rss_mib_max": round(max(x[2] for x in r)), "hwm_mib": round(r[-1][3])}
print(json.dumps(out))
