#!/usr/bin/env python3
"""usage: nfs-xwrite.py FILE N INTERVAL_S LOGFILE -- (machine A) write edit i (content "edit i <hex>") to FILE N times, INTERVAL_S apart; LOGFILE gets one line "i epoch_before_write epoch_after_close"."""
import os, sys, time

path, n, interval, log = sys.argv[1], int(sys.argv[2]), float(sys.argv[3]), sys.argv[4]
os.makedirs(os.path.dirname(path), exist_ok=True)
with open(log, "w") as lf:
    for i in range(n):
        data = ("edit %d %s\n" % (i, os.urandom(8).hex())).encode()
        t0 = time.time()
        with open(path, "wb") as f:
            f.write(data)
        t1 = time.time()
        lf.write("%d %.6f %.6f\n" % (i, t0, t1)); lf.flush()
        time.sleep(interval)
