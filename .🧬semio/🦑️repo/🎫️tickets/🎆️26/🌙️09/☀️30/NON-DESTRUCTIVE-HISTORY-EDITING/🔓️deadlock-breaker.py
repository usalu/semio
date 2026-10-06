#!/usr/bin/env python3
"""🔓️ Cargo build-dir deadlock breaker: every 30 s it looks at the cargos on the SHARED build dir only (a cargo with its own
`CARGO_BUILD_BUILD_DIR` cannot be part of a cycle there, and its compiling rustc must not hide one). A cargo is stalled while it is
childless and its CPU time does not move; a stalled cargo that samples inside `prebuild_lock_exclusive` is a lock waiter. Two or more waiters stalled for
7 min are a cycle (or queue behind one long compile — the re-run is cheap either way): the YOUNGEST is killed, one per pass. A lone
waiter goes after 15 min. Kills are logged in `🗑️generated/coord/deadlock-breaker.txt`; the owner re-runs its command."""
import os, signal, subprocess, time
LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "coord", "deadlock-breaker.txt")
os.makedirs(os.path.dirname(LOG), exist_ok=True)
CYCLE, LONE = 420, 900
def seconds(text):
    days, _, rest = text.strip().rpartition("-")
    parts = [float(p) for p in rest.split(":")]
    while len(parts) < 3: parts.insert(0, 0.0)
    return (int(days) if days else 0) * 86400 + parts[0] * 3600 + parts[1] * 60 + parts[2]
def run(*cmd):
    return subprocess.run(cmd, capture_output=True, text=True).stdout
def kill(pid, why, age, command):
    try:
        os.kill(int(pid), signal.SIGKILL)
        with open(LOG, "a") as out: out.write(f"{time.strftime('%F %T')} killed {pid} ({why}) age {int(age)}s: {command[:180]}\n")
    except OSError: pass
stalled = {}
while True:
    now, seen = time.time(), {}
    for pid in run("pgrep", "-x", "cargo").split():
        if "CARGO_BUILD_BUILD_DIR=" in run("ps", "eww", "-o", "command=", "-p", pid): continue
        fields = run("ps", "-o", "etime=,time=,command=", "-p", pid).split(None, 2)
        if len(fields) < 3: continue
        age, cpu, command = seconds(fields[0]), seconds(fields[1]), fields[2]
        since, last = stalled.get(pid, (now, cpu))
        if run("pgrep", "-P", pid).strip() or cpu != last: since = now
        seen[pid] = (since, cpu)
        stalled[pid] = (since, cpu)
    stalled = dict(seen)
    waiters = []
    for pid, (since, _) in seen.items():
        if now - since < CYCLE: continue
        if "prebuild_lock_exclusive" not in run("sample", pid, "1"): continue
        fields = run("ps", "-o", "etime=,command=", "-p", pid).split(None, 1)
        if len(fields) == 2: waiters.append((seconds(fields[0]), pid, now - since, fields[1]))
    waiters.sort()
    if len(waiters) >= 2:
        age, pid, _, command = waiters[0]
        kill(pid, f"cycle of {len(waiters)} waiters", age, command)
        stalled.pop(pid, None)
    elif waiters and waiters[0][2] >= LONE:
        age, pid, _, command = waiters[0]
        kill(pid, "lone waiter", age, command)
        stalled.pop(pid, None)
    time.sleep(30)
