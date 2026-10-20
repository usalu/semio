#!/usr/bin/env python3
"""Compares the check-tests.py records of a baseline tree and of the tree after the series. usage: check-compare.py <base.json> <series.json>
Exit 1 when the series has a failing test the baseline does not have, or a group ran fewer files than expected."""
import json, sys
base, series = (json.load(open(path, encoding="utf8"))["groups"] for path in sys.argv[1:3])
bad = 0
print(f"{'group':26} {'base pass/fail':>16} {'series pass/fail':>18}  new failures")
for name, row in series.items():
    old = base.get(name, {"pass": 0, "fail": 0, "failing": []})
    fresh = [test for test in row["failing"] if test not in old["failing"]]
    fixed = [test for test in old["failing"] if test not in row["failing"]]
    print(f"{name:26} {old['pass']:>8}/{old['fail']:<7} {row['pass']:>10}/{row['fail']:<7}  {len(fresh)} new, {len(fixed)} fixed")
    for test in fresh:
        print("    NEW  ", test[:200])
    for test in fixed:
        print("    FIXED", test[:200])
    if row.get("exit") not in (0, None) and not row["failing"]:
        print("    EXIT without a named failure:", row.get("tail", "")[-400:].replace("\n", " | "))
        fresh = fresh or ["exit"]
    bad += bool(fresh)
sys.exit(1 if bad else 0)
