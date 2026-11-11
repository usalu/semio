#!/usr/bin/env python3
"""usage: nfs-rpc-cost.py MOUNTPOINT SECONDS STORE_REL -- emulate the replica poll (stat of every origins/*/head.json every 250 ms) on one NFS mount and print the NFS RPCs per second it caused (from /proc/self/mountstats, per operation)."""
import glob, json, os, sys, time

mount, seconds, rel = sys.argv[1], float(sys.argv[2]), sys.argv[3]

def ops():
    out = {}; on = False
    for line in open("/proc/self/mountstats"):
        if line.startswith("device "): on = (" mounted on %s with " % mount) in line; continue
        if on and line.startswith("\t") and ":" in line:
            name, rest = line.strip().split(":", 1)
            if name.replace("_", "").isupper() and rest.split()[0].isdigit(): out[name] = int(rest.split()[0])
    return out

heads = sorted(glob.glob(os.path.join(mount, rel, "origins", "*", "head.json")))
for h in heads: os.stat(h)
before = ops(); end = time.time() + seconds; polls = 0
while time.time() < end:
    for h in heads: os.stat(h)
    polls += 1; time.sleep(0.25)
after = ops()
diff = {k: after.get(k, 0) - before.get(k, 0) for k in after if after.get(k, 0) - before.get(k, 0)}
print(json.dumps({"origins": len(heads), "seconds": seconds, "total_rpc_per_s": round(sum(diff.values()) / seconds, 2), "per_op_per_s": {k: round(v / seconds, 2) for k, v in sorted(diff.items(), key=lambda x: -x[1])}}))
