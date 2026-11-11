#!/usr/bin/env python3
"""usage: mixed-rpc.py NAME=OPTIONS... -- on the macOS host: for every option set mount the export of g1 (192.168.64.4:/srv/semio-store) under $S/mnt/r-NAME, poll the head.json of the four origins with stat four times a second for 20 s (the follow of the replica daemon) and print the NFSv4 COMPOUND calls per second from `nfsstat -c` (a counter of the whole host: run with the daemon stopped)."""
import os, re, subprocess, sys, time
S = os.environ["S"]
def compounds(): return int(re.search(r"Compound\s*\n\s*\d+\s+(\d+)", subprocess.run(["nfsstat", "-c"], capture_output=True, text=True).stdout).group(1))
for arg in sys.argv[1:]:
    name, opts = arg.split("=", 1); d = f"{S}/mnt/r-{name}"; os.makedirs(d, exist_ok=True)
    subprocess.run(["mount", "-t", "nfs", "-o", opts, "192.168.64.4:/srv/semio-store", d], check=True)
    try:
        heads = [f"{d}/v1/origins/{o}/head.json" for o in ("g1-server", "g2-server", "g2-w1", "mac1")]
        for h in heads: os.stat(h)
        time.sleep(3); c0 = compounds(); t0 = time.time(); n = 0
        while time.time() - t0 < 20:
            for h in heads: os.stat(h); n += 1
            time.sleep(0.25)
        c1 = compounds(); dt = time.time() - t0
        print(f"{name} [{opts}]: {(c1 - c0) / dt:.1f} COMPOUND/s for {n / dt:.0f} stat/s", flush=True)
    finally:
        subprocess.run(["umount", d]); os.rmdir(d)
