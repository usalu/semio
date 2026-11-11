#!/usr/bin/env python3
"""usage: mixed-attr-run.py [NAME[=OPTIONS]...] -- on the macOS host: mount the export of g1 once per option set (user-owned mount points under $S/mnt/v-NAME), let g1 replace v1/tmp/attrprobe/h.json atomically N times (default 100, 1.37 s apart), poll it on every mount every 2 ms with stat and with open+read (nfs-attr-probe.py watch both), print how late each mount sees the new file. Named sets: def, plan (the macOS mount plan), ac1 (actimeo=1), noac, v41 (plan with vers=4.1). Environment: S (scratchpad), N, IV, MODE (stat, read or both; the replica polls with stat and opens a head only when the stat changed, so `stat` is the one that matters: an open revalidates the directory and hides a stale name cache)."""
import json, os, subprocess, sys, time
S = os.environ["S"]; HERE = os.path.join(S, "SC"); MODE = os.environ.get("MODE", "both"); N = int(os.environ.get("N", 100)); IV = float(os.environ.get("IV", 1.37))
PLAN = "vers=4,hard,nobrowse,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache,noresvport"
OPTS = {"def": "vers=4,noresvport", "plan": PLAN, "ac1": "vers=4,noresvport,actimeo=1", "noac": "vers=4,noresvport,noac", "v41": "vers=4.1," + PLAN}
names = []
for a in sys.argv[1:] or ["def", "plan", "ac1", "noac"]:
    if "=" in a: n, o = a.split("=", 1); OPTS[n] = o
    else: n = a
    names.append(n)
PROBE = "v1/tmp/attrprobe"
def g1(cmd): return subprocess.run(["limactl", "shell", "g1", "--", "sudo", "-u", "semio", "bash", "-c", cmd], capture_output=True, text=True)
g1(f"mkdir -p /srv/semio-store/{PROBE}; rm -f /srv/semio-store/{PROBE}/h.json")
mounted = []
try:
    for n in names:
        d = f"{S}/mnt/v-{n}"; os.makedirs(d, exist_ok=True)
        r = subprocess.run(["mount", "-t", "nfs", "-o", OPTS[n], "192.168.64.4:/srv/semio-store", d], capture_output=True, text=True)
        if r.returncode: print(f"mount of {n} failed: {r.stderr.strip()}"); sys.exit(1)
        mounted.append(d)
    subprocess.run(["limactl", "copy", f"{HERE}/nfs-attr-probe.py", "g1:/tmp/nfs-attr-probe.py"], capture_output=True)
    off = -json.loads(subprocess.run(["python3", f"{HERE}/nfs-clock.py", "measure", "192.168.64.4", "7777", "600"], capture_output=True, text=True).stdout)["offset_ms"]
    dur = N * IV + 15
    procs = [subprocess.Popen(["python3", f"{HERE}/nfs-attr-probe.py", "watch", MODE, f"{S}/mnt/v-{n}/{PROBE}", str(dur), f"{S}/fv/attr-{n}.log"]) for n in names]
    time.sleep(3)
    g1(f"python3 /tmp/nfs-attr-probe.py write /srv/semio-store/{PROBE} {N} {IV} /tmp/attr.write")
    for p in procs: p.wait()
    open(f"{S}/fv/attr.write", "w").write(g1("cat /tmp/attr.write").stdout)
    print(f"reader (mac) minus writer (g1) clock: {off} ms; load {os.getloadavg()}")
    for n in names:
        out = subprocess.run(["python3", f"{HERE}/nfs-attr-probe.py", "join", f"{S}/fv/attr.write", f"{S}/fv/attr-{n}.log", str(off)], capture_output=True, text=True).stdout.strip()
        print(f"{n} [{OPTS[n]}]: {out}", flush=True)
finally:
    for d in mounted: subprocess.run(["umount", d]); os.rmdir(d)
    g1(f"rm -rf /srv/semio-store/{PROBE}")
