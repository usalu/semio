#!/usr/bin/env python3
"""usage: nfs-xread.py FILE SECONDS LOGFILE -- (machine B) poll FILE every 10 ms for SECONDS and log "i epoch" the first time each edit index appears."""
import sys, time

path, seconds, log = sys.argv[1], float(sys.argv[2]), sys.argv[3]
seen = set()
end = time.time() + seconds
with open(log, "w") as lf:
    while time.time() < end:
        try:
            with open(path, "rb") as f:
                text = f.read()
            now = time.time()
            parts = text.split()
            if len(parts) == 3 and parts[0] == b"edit":
                i = int(parts[1])
                if i not in seen:
                    seen.add(i); lf.write("%d %.6f\n" % (i, now)); lf.flush()
        except (OSError, ValueError):
            pass
        time.sleep(0.01)
