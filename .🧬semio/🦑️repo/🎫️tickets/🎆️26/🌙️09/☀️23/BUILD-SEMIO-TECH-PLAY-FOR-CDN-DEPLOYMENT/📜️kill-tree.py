#!/usr/bin/env python3
"""🪓️ SIGKILL every descendant of the given root pids (and the roots), then report survivors. usage: kill-tree.py <pid…>"""
import os, signal, subprocess, sys, time

rows = [line.split() for line in subprocess.run(["ps", "-axo", "pid=,ppid="], capture_output=True, text=True).stdout.splitlines()]
children: dict[int, list[int]] = {}
for pid, ppid in rows: children.setdefault(int(ppid), []).append(int(pid))
tree, stack = [], [int(p) for p in sys.argv[1:]]
while stack:
    pid = stack.pop(); tree.append(pid); stack.extend(children.get(pid, []))
for pid in tree:
    try: os.kill(pid, signal.SIGKILL)
    except ProcessLookupError: pass
time.sleep(1)
alive = [p for p in tree if subprocess.run(["kill", "-0", str(p)], capture_output=True).returncode == 0]
print(f"killed {len(tree)} pids; alive: {alive}")
