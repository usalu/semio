#!/usr/bin/env python3
"""🔎️ Read-only: prints the exact mount text (attribute lines + mod line) of a shared helper at every mounting site listed in a modowners scan.

usage: mountlines.py <scan.tsv> <needle>
"""
import sys, re, collections
REPO = "/Users/ueli/Documents/semio"
scan, needle = sys.argv[1], sys.argv[2]
shapes = collections.Counter()
rows = []
for line in open(scan, encoding="utf-8"):
    f, n, pk, sites = line.rstrip("\n").split("\t")
    if needle not in f:
        continue
    for s in sites.split(" ; "):
        pkg, _, ms = s.partition("=")
        for m in ms.split(","):
            m = re.sub(r":\d+.*$", "", m.strip())
            if not m.endswith(".rs"):
                continue
            rows.append(m)
for m in sorted(set(rows)):
    text = open(f"{REPO}/{m}", encoding="utf-8").read()
    found = False
    for mt in re.finditer(r'((?:[ \t]*#\[[^\n]*\]\s*)*)[ \t]*((?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;)', text):
        pm = re.search(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]', mt.group(1))
        if pm and needle in pm.group(1):
            found = True
            attrs = re.sub(r'"[^"]*' + re.escape(needle) + r'[^"]*"', '"<P>"', mt.group(1)).strip()
            shapes[(attrs + " | " + mt.group(2)).replace("\n", "\\n")] += 1
    if not found:
        print("NO-MOUNT-FOUND", m)
for k, v in shapes.most_common():
    print(v, k)
