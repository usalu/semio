#!/usr/bin/env python3
"""⏱️ T14 lane-hold deadline (macOS has no `timeout`): runs a command in its own process group and, when the absolute
deadline passes, SIGTERMs then SIGKILLs the whole group (cargo + every rustc child, so no orphan keeps a lock), exit 124.
usage: deadline.py <epoch-seconds> <command…>"""
import os
import signal
import subprocess
import sys
import time

deadline = float(sys.argv[1])
if time.time() >= deadline:
    print(f"DEADLINE-SKIP {' '.join(sys.argv[2:5])}", file=sys.stderr)
    sys.exit(124)
process = subprocess.Popen(sys.argv[2:], start_new_session=True)
try:
    sys.exit(process.wait(timeout=max(1.0, deadline - time.time())))
except subprocess.TimeoutExpired:
    os.killpg(process.pid, signal.SIGTERM)
    try:
        process.wait(timeout=15)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        process.wait()
    print(f"DEADLINE-KILLED {' '.join(sys.argv[2:5])}", file=sys.stderr)
    sys.exit(124)
