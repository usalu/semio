#!/usr/bin/env python3
"""🏛️ Promotion contention check (run INSIDE a mirror): drifts one closure file, starts N heavy commands at once, reports exit codes, wall and who promoted.
usage: mm run <n> -- python3 -I concurrent.py [N] [command words]"""
import os, subprocess, sys, time
N = int(sys.argv[1]) if len(sys.argv) > 1 else 3
WORDS = sys.argv[2:] or ["cache", "status"]
FILE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🏃️main/🟦️.ts"
original = open(FILE, "rb").read()
open(FILE, "wb").write(original + b"\n")
env = dict(os.environ, SEMIO_REPO_IMPLEMENTATION="sealed", SEMIO_LANGUAGE="en")
started = time.perf_counter()
try:
    procs = [subprocess.Popen(["bun", "repo"] + WORDS, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True) for _ in range(N)]
    for index, proc in enumerate(procs):
        err = proc.communicate(timeout=600)[1]
        notes = [line[:150] for line in err.splitlines() if line.startswith("[implementation]") and "Promoted" in line or "Waiting for the promotion" in line or "not promoted" in line]
        print("proc %d exit %s wall %.1f s  %s" % (index, proc.returncode, time.perf_counter() - started, notes[:2]), flush=True)
finally:
    open(FILE, "wb").write(original)
