#!/usr/bin/env python3
"""Runs the plugin units suite in the overlay for one test-name pattern and prints the first error block. usage: ut.py PATTERN [LINES]"""
import re, subprocess, sys
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/testfix-real-tools/one.py"
F = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/🟦️.ts"
r = subprocess.run(["python3", T, F, "--test-name-pattern", sys.argv[1]], capture_output=True, text=True)
out = r.stdout.strip().split("\n")
print("\n".join(out[:1] + [l for l in out[1:]]))
log = out[0].split()[-1]
t = open(log, encoding="utf8").read()
n = int(sys.argv[2]) if len(sys.argv) > 2 else 30
i = t.find("\nerror")
if i >= 0: print("\n".join(l[:300] for l in t[i:].split("\n")[:n]))
