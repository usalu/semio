#!/usr/bin/env python3
"""usage: mixed-kill-tmp.py TMPDIR UNIT MIN TIMEOUT_S -- poll TMPDIR (v1/tmp of a store) every 2 ms; when it holds MIN more entries than at the start (a blob or head write in flight), SIGKILL the main pid of the systemd UNIT (read once from systemctl, then killed by that PID) and print the pid and the entries seen; "no-write" after TIMEOUT_S."""
import json, os, signal, subprocess, sys, time

tmp, unit, need, timeout = sys.argv[1], sys.argv[2], int(sys.argv[3]), float(sys.argv[4])
base = len(os.listdir(tmp)); end = time.time() + timeout
while time.time() < end:
    names = os.listdir(tmp)
    if len(names) >= base + need:
        pid = int(subprocess.run(["systemctl", "show", unit, "-p", "MainPID", "--value"], capture_output=True, text=True).stdout.strip())
        os.kill(pid, signal.SIGKILL); print(json.dumps({"killed_pid": pid, "tmp_entries": len(names), "base": base, "at": time.time()})); sys.exit(0)
    time.sleep(0.002)
print("no-write")
