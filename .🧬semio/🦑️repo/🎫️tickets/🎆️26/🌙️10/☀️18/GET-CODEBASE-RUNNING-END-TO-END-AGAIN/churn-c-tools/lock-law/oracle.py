"""🐍️ Independent oracle of the build-dir exclusion law: reads the interval log the build script of every cargo appended and asserts that no two executions of the same unit overlap and that the final binary names the last writer only.
usage: oracle.py <log> <expected-runs> [<binary-output>]"""
import json, sys

rows = [line.split() for line in open(sys.argv[1], encoding="utf-8") if line.strip()]
open_runs, intervals = {}, []
for what, pid, variant, ns in rows:
    key = (pid, variant)
    if what == "start":
        open_runs[key] = int(ns)
    else:
        intervals.append((open_runs.pop(key), int(ns), variant))
intervals.sort()
overlaps = [(a, b) for a, b in zip(intervals, intervals[1:]) if b[0] < a[1]]
result = {"runs": len(intervals), "overlaps": len(overlaps), "unfinished": len(open_runs), "variants": [row[2] for row in intervals]}
if len(sys.argv) > 3:
    text = open(sys.argv[3], encoding="utf-8").read().split()
    result["finalVariants"] = sorted({token.split(":")[0] for token in text})
    result["finalParts"] = len(text)
print(json.dumps(result))
ok = not overlaps and not open_runs and result["runs"] >= int(sys.argv[2])
sys.exit(0 if ok else 1)
