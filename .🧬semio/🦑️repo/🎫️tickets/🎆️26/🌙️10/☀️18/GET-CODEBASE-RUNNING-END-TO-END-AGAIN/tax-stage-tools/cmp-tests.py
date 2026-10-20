#!/usr/bin/env python3
# 🔬 Compares two run-tests.sh outputs (live baseline vs mirror): per test file the pass/fail counts and the failing test names. usage: cmp-tests.py <live.txt> <mirror.txt>
import re, sys
def parse(path):
    out = {}; cur = None
    for line in open(path, encoding="utf8", errors="replace"):
        m = re.match(r"(live|mirror) rc=(\S+) pass=(\S+) fail=(\S+) secs=(\S+) (.*)", line.rstrip())
        if m: cur = m.group(6); out[cur] = {"pass": m.group(3), "fail": m.group(4), "names": set()}
        elif line.startswith("    (fail)") and cur: out[cur]["names"].add(line.strip())
    return out
a, b = parse(sys.argv[1]), parse(sys.argv[2])
same = 0
for k in a:
    x, y = a[k], b.get(k)
    if y is None: print("MISSING in mirror", k[-90:]); continue
    if (x["pass"], x["fail"], x["names"]) == (y["pass"], y["fail"], y["names"]): same += 1; continue
    print("DIFF", k[-90:], "live", x["pass"], "/", x["fail"], "-> mirror", y["pass"], "/", y["fail"])
    print("   only mirror:", sorted(y["names"] - x["names"])[:12]); print("   only live:", sorted(x["names"] - y["names"])[:12])
print("same:", same, "of", len(a))
