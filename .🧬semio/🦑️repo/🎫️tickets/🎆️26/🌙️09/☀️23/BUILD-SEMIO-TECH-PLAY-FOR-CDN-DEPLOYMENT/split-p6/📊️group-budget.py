#!/usr/bin/env python3
"""📊️ Folds the lib and test unit file lists of `📏️unit-files.ts --files` onto groups: per group lib files/lines/bytes
and test-only files/lines/bytes (test unit minus lib unit, active lines under cfg(test)).

usage: 📊️group-budget.py <artifact-dir> <groups.json> <lib.tsv> <test.tsv>
"""
import json, os, re, sys, collections
art, groups, lib, test = sys.argv[1:5]
rules = [(re.compile(r), g) for r, g in json.load(open(groups))]
REPO = "/Users/ueli/Documents/semio/"
def rows(path):
    out = {}
    for line in open(path):
        if not line.startswith("F\t"): continue
        _, lines, nbytes, kind, p = line.rstrip("\n").split("\t")
        out[p] = (int(lines), int(nbytes))
    return out
L, T = rows(lib), rows(test)
prefix = os.path.relpath(art, REPO) + "/"
def group(p):
    r = p[len(prefix):] if p.startswith(prefix) else "../" + p
    for rule, g in rules:
        if rule.search(r): return g
    return "?" + r
agg = collections.defaultdict(lambda: [0, 0, 0, 0, 0, 0])
for p, (l, b) in L.items():
    a = agg[group(p)]; a[0] += 1; a[1] += l; a[2] += b
for p, (l, b) in T.items():
    a = agg[group(p)]
    if p in L:
        a[4] += max(0, l - L[p][0])
    else:
        a[3] += 1; a[4] += l; a[5] += b
print(f"{'group':28s} {'libF':>5s} {'libL':>6s} {'libB':>8s} | {'tstF':>5s} {'tstL':>6s} {'tstB':>8s}")
for g in sorted(agg):
    a = agg[g]; print(f"{g[:60]:28s} {a[0]:5d} {a[1]:6d} {a[2]:8d} | {a[3]:5d} {a[4]:6d} {a[5]:8d}")
tot = [sum(a[i] for a in agg.values()) for i in range(6)]
print(f"{'TOTAL':28s} {tot[0]:5d} {tot[1]:6d} {tot[2]:8d} | {tot[3]:5d} {tot[4]:6d} {tot[5]:8d}")
