#!/usr/bin/env python3
"""usage: mixed-stages.py head.json DURATION LOG -- poll a store head.json every 5 ms and log "epoch events" whenever `events` changes (stage timing of the relay path)."""
import json, sys, time
p, dur, log = sys.argv[1], float(sys.argv[2]), sys.argv[3]
last = None; end = time.time() + dur
with open(log, "w") as lf:
    while time.time() < end:
        try:
            e = json.load(open(p))["events"]; now = time.time()
            if e != last: last = e; lf.write("%.6f %d\n" % (now, e)); lf.flush()
        except (OSError, ValueError, KeyError): pass
        time.sleep(0.005)
