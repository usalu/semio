#!/usr/bin/env python3
"""🧭️ Lists every test module whose test-only references point at a crate that (transitively) depends on the test's
own crate, per `🕸️family-graph.py` output (`--edges N`): those tests must become `[[test]]` targets of their owner.

usage: 🧭️upward-tests.py <family-graph output>
"""
import collections, re, sys
lib, test, files = collections.defaultdict(set), collections.defaultdict(set), collections.defaultdict(set)
edge = None
for line in open(sys.argv[1], encoding="utf-8"):
    m = re.match(r"(\S+) -> (\S+): lib (\d+) test (\d+)", line)
    if m:
        edge = (m.group(1), m.group(2))
        if int(m.group(3)):
            lib[edge[0]].add(edge[1])
        continue
    m = re.match(r"\s+test (\S+) :: ", line)
    if m and edge:
        files[edge].add(m.group(1))
        test[edge[0]].add(edge[1])
def reach(a):
    seen, stack = set(), [a]
    while stack:
        for y in lib[stack.pop()]:
            if y not in seen:
                seen.add(y)
                stack.append(y)
    return seen
for a in sorted(test):
    for b in sorted(test[a]):
        if not a.startswith("[[test]]") and a in reach(b):
            for f in sorted(files[(a, b)]):
                print(f"{a}\t{b}\t{f}")
