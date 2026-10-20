#!/usr/bin/env python3
import subprocess, re, sys, os
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out = subprocess.run(["git", "ls-files", "-z", "-co", "--exclude-standard", "--", "*.ts", "*.tsx", "*.js", "*.mjs"], cwd=ROOT, capture_output=True).stdout.decode("utf-8", "replace").split("\0")
rx = re.compile(rb"""from\s*["']bun:test["']""")
res = []
for f in out:
    if not f or f.startswith(".🧬semio/🦑️repo/🎫️tickets/") or "/node_modules/" in f: continue
    p = os.path.join(ROOT, f)
    try:
        if os.path.getsize(p) > 5_000_000: continue
        if rx.search(open(p, "rb").read()): res.append(f)
    except OSError: pass
open(sys.argv[1], "w", encoding="utf-8").write("\n".join(sorted(res)) + "\n")
print(len(res))
