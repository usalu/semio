"""🧾 rii: test-name multiset of the files this slice touched: pre-edit copy (scratchpad rii/pre) versus the live file; new files are listed. usage: names.py"""
import os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/../rtb-tools")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import testnames
from ed import REPO, PRE, TOUCHED
tot_b = tot_a = 0
diff = []
new = []
for rel in sorted({l.strip() for l in open(TOUCHED, encoding="utf8") if l.strip().endswith(".rs")}):
    live = REPO + "/" + rel
    pre = PRE + "/" + rel
    a = collections.Counter(testnames.names_of(open(live, encoding="utf8").read())) if os.path.exists(live) else collections.Counter()
    if not os.path.exists(pre):
        new.append((rel[-60:], sum(a.values())))
        continue
    b = collections.Counter(testnames.names_of(open(pre, encoding="utf8").read()))
    tot_b += sum(b.values()); tot_a += sum(a.values())
    if a != b:
        diff.append((rel, dict(b - a), dict(a - b)))
print("touched files with a pre-edit copy: tests before", tot_b, "after", tot_a)
print("files whose test-name multiset differs:", len(diff))
for d in diff: print("  ", d)
print("new files (no pre-edit copy):", new)
print("IDENTICAL" if not diff else "DIFFERENT")
