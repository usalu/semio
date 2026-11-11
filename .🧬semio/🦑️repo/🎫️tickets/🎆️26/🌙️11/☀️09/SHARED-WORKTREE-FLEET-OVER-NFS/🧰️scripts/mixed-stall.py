#!/usr/bin/env python3
"""usage: mixed-stall.py DIR SECONDS -- on a macOS NFS mount: one thread replaces DIR/h.json by rename of a file from the sibling directory stalltmp (a rename across directories, like replaceText of the replica) every second (like head.json), another stats and reads it every 20 ms; prints the syscalls that took longer than 300 ms (the stall of the NFS client after a rename over a file it has read)."""
import os, sys, threading, time
d, secs = sys.argv[1], float(sys.argv[2]); os.makedirs(d, exist_ok=True); tdir = os.path.join(os.path.dirname(d), "stalltmp"); os.makedirs(tdir, exist_ok=True)
end = time.time() + secs; slow = []; n = [0]
def writer():
    i = 0
    while time.time() < end:
        t = os.path.join(tdir, "t%d" % i); i += 1
        with open(t, "w") as f: f.write("edit %d\n" % i)
        t0 = time.time(); os.rename(t, os.path.join(d, "h.json")); dt = time.time() - t0
        if dt > .3: slow.append(("rename", round(dt * 1000)))
        time.sleep(1.0)
def reader():
    while time.time() < end:
        for op in ("stat", "read"):
            t0 = time.time()
            try:
                if op == "stat": os.stat(os.path.join(d, "h.json"))
                else: open(os.path.join(d, "h.json"), "rb").read()
            except OSError: pass
            dt = time.time() - t0; n[0] += 1
            if dt > .3: slow.append((op, round(dt * 1000)))
        time.sleep(0.02)
a = threading.Thread(target=writer); b = threading.Thread(target=reader); a.start(); b.start(); a.join(); b.join()
print("ops", n[0], "slow:", slow)
