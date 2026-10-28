"""[gram3-g] compares two ROWS_DIGEST outputs of full.mjs (owner -> "sha256:count" of the observation rows in order) and two harness result files. usage: cmp-rows.py <before.rows.json> <after.rows.json> [<before.json> <after.json>]"""
import json, sys
a, b = (json.load(open(p, encoding="utf8")) for p in sys.argv[1:3])
same = sorted(a) == sorted(b) and all(a[k] == b[k] for k in a)
print("owners", len(a), len(b), "rows digests equal:", same)
if not same:
    for k in sorted(set(a) | set(b)):
        if a.get(k) != b.get(k):
            print("  differs:", k, a.get(k), b.get(k))
if len(sys.argv) > 4:
    x, y = (json.load(open(p, encoding="utf8")) for p in sys.argv[3:5])
    for name, d in (("before", x), ("after", y)):
        s = d["series"]
        end = [r for r in s if r["tag"] == "afterGC"][-1]
        print(name, "seconds", d["seconds"], "peakRssMB", d["peakRssMB"], "live after GC MB", end["heap"] + end["extra"], "projects", d["projects"])
