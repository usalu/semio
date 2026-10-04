#!/usr/bin/env python3
"""🔓️ Cargo build-dir deadlock breaker: every 60 s, a childless cargo with < 3 s CPU that sits in `prebuild_lock_exclusive → flock`
(sampled) is killed per pid (its owner re-runs) when it is older than 15 min AND no cargo anywhere has a compiling rustc child (a global
flock cycle), or older than 25 min regardless (a partial cycle); a waiter queued behind progressing builds is left alone."""
import os, signal, subprocess, time
LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "coord", "deadlock-breaker.txt")
os.makedirs(os.path.dirname(LOG), exist_ok=True)
def seconds(text):
    days, _, rest = text.strip().rpartition("-")
    parts = [float(p) for p in rest.split(":")]
    while len(parts) < 3: parts.insert(0, 0.0)
    return (int(days) if days else 0) * 86400 + parts[0] * 3600 + parts[1] * 60 + parts[2]
def run(*cmd):
    return subprocess.run(cmd, capture_output=True, text=True).stdout
while True:
    cargos = run("pgrep", "-x", "cargo").split()
    rustc_parents = {line.strip() for line in run("ps", "-Ao", "ppid=,comm=").splitlines() if line.strip().endswith("rustc")}
    active = any(any(entry.split()[0] == pid for entry in rustc_parents) for pid in cargos)
    for pid in cargos:
        if run("pgrep", "-P", pid).strip(): continue
        fields = run("ps", "-o", "etime=,time=,command=", "-p", pid).split(None, 2)
        if len(fields) < 3: continue
        age, cpu, command = seconds(fields[0]), seconds(fields[1]), fields[2]
        if cpu >= 3 or age < 900 or (active and age < 1500): continue
        if "prebuild_lock_exclusive" not in run("sample", pid, "1"): continue
        try:
            os.kill(int(pid), signal.SIGKILL)
            with open(LOG, "a") as out: out.write(f"{time.strftime('%F %T')} killed {pid} age {int(age)}s cpu {cpu}s: {command[:180]}\n")
        except OSError: pass
    time.sleep(60)
