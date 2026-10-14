#!/usr/bin/env python3
"""🔁️ Reads `🏗️rehost.py plan` outputs and reports normal-edge cycles and test edges that close a cycle (dev-dependency loops).

usage: 🔁️plan-cycles.py <plan.txt>...
"""
import re, sys, collections
for path in sys.argv[1:]:
    text = open(path, encoding="utf-8").read()
    normal = re.findall(r"^  (\S+) -> (\S+): \d+$", text.split("## crate edges (test)")[0], re.M)
    test = re.findall(r"^  (\S+) -> (\S+): \d+(?:\s+e\.g\. (.*))?$", text.split("## crate edges (test)")[1].split("## dropped")[0], re.M) if "## crate edges (test)" in text else []
    g = collections.defaultdict(set)
    for a, b in normal: g[a].add(b)
    def reach(a, b, seen=None):
        seen = seen or set()
        if a == b: return True
        seen.add(a)
        return any(reach(n, b, seen) for n in g[a] if n not in seen)
    cyc = [(a, b) for a, b in normal if reach(b, a)]
    tcyc = [(a, b, e) for a, b, e in test if reach(b, a)]
    print(f"{path}: normal cycles {cyc}  test cycles {[(a, b, e[-120:]) for a, b, e in tcyc]}")
    for line in re.findall(r"^   .*$", text, re.M)[:6]:
        print("   ", line.strip()[-160:])
