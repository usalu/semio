#!/usr/bin/env python3
"""✂️ Cut-point advisor for rsfn phases. usage: cuts.py <file> <fn> [--nth N] [--block LINE] [--target 150]
Prints the live-variable crossing of every statement boundary and a greedy proposal of phase ranges (from,to start lines) of about `target` lines that minimises the crossing count."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn
a = sys.argv[1:]
file, fname = a[0], a[1]
opt = a[2:]
nth = int(opt[opt.index("--nth") + 1]) if "--nth" in opt else 0
blk = int(opt[opt.index("--block") + 1]) if "--block" in opt else None
target = int(opt[opt.index("--target") + 1]) if "--target" in opt else 150
text = open(file, encoding="utf-8").read()
if blk:
    toks, k, op, cl, params, rows = rsfn.analyse_block(text, fname, nth, blk, {})
else:
    toks, k, op, cl, params, rows = rsfn.analyse(text, fname, nth)
n = len(rows)
declared_at = {p: -1 for p in params}
for i, r in enumerate(rows):
    for d in r["decl"]: declared_at[d] = i
cross = []
for i in range(n - 1):
    live = set()
    for j in range(i + 1, n):
        for name in rsfn.used_in(toks, rows[j]["s"], rows[j]["e"], {x for x, di in declared_at.items() if di <= i and di >= 0}):
            live.add(name)
    cross.append(sorted(live))
def size(a, b): return rows[b]["b"] - rows[a]["a"] + 1
print("stmts", n, "lines", rows[-1]["b"] - rows[0]["a"] + 1)
i = 0
phases = []
while i < n:
    best = None
    j = i
    while j < n and size(i, j) <= target * 1.6:
        if j == n - 1 or size(i, j) >= target * 0.6:
            c = len(cross[j]) if j < n - 1 else 0
            if best is None or c < best[0] or (c == best[0] and abs(size(i, j) - target) < abs(size(i, best[1]) - target)): best = (c, j)
        j += 1
    if best is None: best = (0, n - 1)
    phases.append((i, best[1], best[0]))
    i = best[1] + 1
for a_, b_, c in phases:
    print("phase %d-%d lines %d crossing-out %d %s" % (rows[a_]["a"], rows[b_]["a"], size(a_, b_), c, ",".join(cross[b_]) if b_ < n - 1 else ""))

if "--all" in opt:
    print("boundaries (after statement start line: crossing)")
    for i in range(n - 1):
        if len(cross[i]) <= 3:
            print("  after %d (next %d): %d %s" % (rows[i]["a"], rows[i + 1]["a"], len(cross[i]), ",".join(cross[i])))
