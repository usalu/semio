#!/usr/bin/env python3
"""usage: nfs-kill-when.py HEAD_JSON UNIT DELTA TIMEOUT_S -- poll HEAD_JSON every 5 ms; when its `events` grew by DELTA since the start, SIGKILL the main pid of the systemd UNIT and print the head that was seen, the pid and the epoch; prints "no-publish" after TIMEOUT_S."""
import json, os, signal, subprocess, sys, time

head, unit, delta, timeout = sys.argv[1], sys.argv[2], int(sys.argv[3]), float(sys.argv[4])
def events():
    try: return json.load(open(head))["events"], json.load(open(head))
    except (OSError, ValueError, KeyError): return None, None
base, _ = events(); end = time.time() + timeout
while time.time() < end:
    e, h = events()
    if e is not None and (base is None or e >= base + delta):
        pid = int(subprocess.run(["systemctl", "show", unit, "-p", "MainPID", "--value"], capture_output=True, text=True).stdout.strip())
        os.kill(pid, signal.SIGKILL); print(json.dumps({"killed_pid": pid, "head": h, "base_events": base, "at": time.time()})); sys.exit(0)
    time.sleep(0.005)
print("no-publish")
