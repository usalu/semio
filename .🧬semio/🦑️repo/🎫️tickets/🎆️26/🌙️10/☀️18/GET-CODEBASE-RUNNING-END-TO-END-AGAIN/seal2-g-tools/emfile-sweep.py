#!/usr/bin/env python3
"""🎲️ Sweeps RLIMIT_NOFILE for the sealed door (run INSIDE a mirror: mm run <n> -- python3 -I emfile-sweep.py <bun> <mode> [low high repeats]). Prints exit code and output size per limit, the first output lines of every run that ends with exit 2 or without output."""
import os, resource, subprocess, sys
PB, MODE = sys.argv[1], sys.argv[2]
LOW, HIGH, REPEATS = (int(x) for x in sys.argv[3:6]) if len(sys.argv) > 5 else (5, 40, 3)
DOOR = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts"
seen = {}
for limit in range(LOW, HIGH + 1):
    for _ in range(REPEATS):
        def cap(): resource.setrlimit(resource.RLIMIT_NOFILE, (limit, limit))
        p = subprocess.Popen([PB, DOOR, "repo", "--help"], env=dict(os.environ, SEMIO_REPO_IMPLEMENTATION=MODE, SEMIO_LANGUAGE="en"), stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, preexec_fn=cap)
        out = p.communicate()[0]
        key = (limit, p.returncode, "empty" if not out else "usage" if b"Usage:" in out else "text")
        seen[key] = seen.get(key, 0) + 1
        if p.returncode == 2 or (p.returncode != 0 and not out):
            print("BAD limit=%d exit=%d bytes=%d head=%r" % (limit, p.returncode, len(out), out[:160]), flush=True)
for key in sorted(seen): print(key, seen[key])
