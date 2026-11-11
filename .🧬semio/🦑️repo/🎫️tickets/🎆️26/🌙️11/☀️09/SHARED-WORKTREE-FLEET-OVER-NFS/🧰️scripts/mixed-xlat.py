#!/usr/bin/env python3
"""usage: mixed-xlat.py NAME WRITER:TREE READER:TREE N INTERVAL_S [SUBDIR] -- end-to-end latency of one file edited 'N' times on node WRITER (g1, g2 = Lima VMs through limactl, mac = this host) and polled every 10 ms on node READER; nodes' clock offsets to this host are measured (nfs-clock.py serving on port 7777 of the VMs) before and after; prints the nfs-xjoin.py JSON. LIMA_HOME must be set; the nfs-*.py helpers are in /tmp of the VMs."""
import json, os, subprocess, sys, tempfile, threading, time
HERE = os.path.dirname(os.path.abspath(__file__))
IPS = {"g1": "192.168.64.4", "g2": "192.168.64.5"}
name, w, r, n, iv = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4]), float(sys.argv[5])
sub = sys.argv[6] if len(sys.argv) > 6 else "rehearsal-live"
wn, wt = w.split(":", 1); rn, rt = r.split(":", 1)
def sh(node, cmd, **kw):
    if node == "mac": return subprocess.run(["bash", "-c", cmd], capture_output=True, text=True, **kw)
    return subprocess.run(["limactl", "shell", node, "--", "sudo", "-u", "semio", "bash", "-c", cmd], capture_output=True, text=True, **kw)
def offset(node):
    if node == "mac": return 0.0
    out = subprocess.run(["python3", os.path.join(HERE, "nfs-clock.py"), "measure", IPS[node], "7777", "600"], capture_output=True, text=True).stdout
    return json.loads(out)["offset_ms"]
o1 = {x: offset(x) for x in {wn, rn}}
dur = n * iv + 25
helpers = lambda node: HERE if node == "mac" else "/tmp"
path = lambda t: f"{t}/{sub}/{name}.txt"
th = threading.Thread(target=lambda: sh(rn, f"python3 {helpers(rn)}/nfs-xread.py {path(rt)} {dur} /tmp/{name}.read"))
th.start(); time.sleep(4)
sh(wn, f"python3 {helpers(wn)}/nfs-xwrite.py {path(wt)} {n} {iv} /tmp/{name}.write")
th.join()
o2 = {x: offset(x) for x in {wn, rn}}
d = tempfile.mkdtemp()
for node, kind in ((wn, "write"), (rn, "read")):
    open(f"{d}/{kind}", "w").write(sh(node, f"cat /tmp/{name}.{kind}").stdout)
off = ((o2[rn] + o1[rn]) - (o2[wn] + o1[wn])) / 2
print(f"{name}: {w} -> {r}; offsets before {o1} after {o2}; reader-minus-writer {off:.2f} ms", flush=True)
print(subprocess.run(["python3", os.path.join(HERE, "nfs-xjoin.py"), f"{d}/write", f"{d}/read", str(off)], capture_output=True, text=True).stdout.strip())
