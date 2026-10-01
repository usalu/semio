#!/usr/bin/env python3
"""🚀️ Starts a command in its own session (survives agent/turn teardown): detach.py <log> <cwd> <cmd...>; prints the pid."""
import os, subprocess, sys
log, cwd, cmd = sys.argv[1], sys.argv[2], sys.argv[3:]
os.makedirs(os.path.dirname(os.path.abspath(log)), exist_ok=True)
with open(log, "ab", buffering=0) as out:
    child = subprocess.Popen(cmd, cwd=cwd, stdout=out, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True,
                             env={**os.environ, "NODE_OPTIONS": ""})
print(child.pid)
