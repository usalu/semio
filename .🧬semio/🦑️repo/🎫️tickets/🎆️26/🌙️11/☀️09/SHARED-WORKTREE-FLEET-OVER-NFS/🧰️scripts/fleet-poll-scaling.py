#!/usr/bin/env python3
"""usage: fleet-poll-scaling.py D1 D2 ... -- idle polling cost against origin count. Needs a built fleet with all daemons stopped. For every D adds D dummy origins homed at each of g2..g5 (stores g2..g5), lets relay-g1 import them into store g1, and samples relay-g1 and the daemon g1-s (--scan 3600) for 60 s. Prints one JSON line per step."""
import json, os, subprocess, sys, time
import fleet

fleet.GROUPS_ = None
here = os.path.dirname(os.path.abspath(__file__))
os.environ["FLEET_SCAN"] = "3600"
def cpu(pid):
    out = subprocess.run(["ps", "-o", "cputime=,rss=", "-p", str(pid)], capture_output=True, text=True).stdout.split()
    t = out[0].split(":"); s = float(t[-1]) + 60 * int(t[-2]) + (3600 * int(t[-3]) if len(t) > 2 else 0)
    return s, int(out[1]) / 1024
done = 0
fleet.POLL, fleet.SCAN = "250", "3600"
fleet.start_relay("g1"); fleet.start_daemon("g1-s")
for d in map(int, sys.argv[1:]):
    for g in ("g2", "g3", "g4", "g5"):
        subprocess.run([sys.executable, here + "/dummy-origins.py", fleet.store(g), g, str(d), "dummy" + g[1:]], check=True)
    time.sleep(45)
    pids = fleet.pids(); origins = len(os.listdir(fleet.store("g1") + "/v1/origins"))
    a = {n: cpu(pids[n]) for n in ("relay-g1", "g1-s")}; time.sleep(60); b = {n: cpu(pids[n]) for n in ("relay-g1", "g1-s")}
    print(json.dumps({"dummies_per_peer": d, "origins_in_store_g1": origins, **{n + "_cpu_pct": round((b[n][0] - a[n][0]) / 60 * 100, 2) for n in a}, **{n + "_rss_mib": round(b[n][1]) for n in a}}), flush=True)
fleet.stop(["relay-g1", "g1-s"])
