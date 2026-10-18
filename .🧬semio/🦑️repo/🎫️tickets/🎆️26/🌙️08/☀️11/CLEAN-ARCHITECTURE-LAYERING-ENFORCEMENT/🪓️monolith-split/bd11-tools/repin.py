#!/usr/bin/env python3
"""usage: repin.py <test.ts> <fixture-dir> [--apply] : for every failing toBe(expected) with an exact string or number mismatch, replaces the expected literal by the received one inside the JSON fixtures of <fixture-dir> (every occurrence of the same literal)."""
import re, sys, os, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import needles
T = os.path.abspath(sys.argv[1]); D = os.path.abspath(sys.argv[2]); apply = "--apply" in sys.argv
src, sites, out = needles.run(T)
pairs = {}
for ln, col, msg in sites:
    m = re.search(r'Expected: "([^"]+)" \| Received: "([^"]+)"', msg)
    if m: pairs[m.group(1)] = m.group(2); continue
    m = re.search(r'Expected: (\d+) \| Received: (\d+)', msg)
    if m and '--numbers' in sys.argv: pairs[m.group(1)] = m.group(2)
files = glob.glob(D + "/*.json")
for old, new in pairs.items():
    hits = [(f, open(f).read().count(old)) for f in files]
    hits = [(f, c) for f, c in hits if c]
    print(old[:16], "->", new[:16], [(os.path.basename(f), c) for f, c in hits])
    if apply and hits:
        for f, c in hits:
            s = open(f).read(); open(f, "w").write(s.replace(old, new))
