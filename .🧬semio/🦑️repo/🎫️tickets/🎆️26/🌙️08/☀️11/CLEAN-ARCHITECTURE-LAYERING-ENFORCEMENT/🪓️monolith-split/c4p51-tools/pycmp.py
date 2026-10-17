"""⚖️ Compares the per-scenario rows of two pyrun result sets (before versus after), temp directory names normalised.

usage: pycmp.py <before.jsonl> <after.jsonl>...   (pairs: b1 a1 b2 a2 ...)
Prints one line per pair: scenario counts, status multiset and the number of scenarios whose full row differs.
"""
import collections, json, re, sys

NORM = re.compile(r"c4p51-py-[A-Za-z0-9_]+")


def rows(path):
    out = {}
    for line in open(path, encoding="utf-8"):
        if line.strip():
            row = json.loads(line)
            row.pop("durationMs", None)
            for d in row.get("diagnostics", []):
                detail = d.get("detail")
                if isinstance(detail, str) and "Traceback" in detail:
                    d["detail"] = "TRACEBACK " + [x for x in detail.splitlines() if x.strip()][-1]
            out[row["scenario"]] = NORM.sub("TMP", json.dumps(row, sort_keys=True, ensure_ascii=False))
    return out


args = sys.argv[1:]
bad = 0
for i in range(0, len(args), 2):
    b, a = rows(args[i]), rows(args[i + 1])
    status = lambda d: collections.Counter(json.loads(v)["status"] for v in d.values())
    diff = [k for k in sorted(set(b) | set(a)) if b.get(k) != a.get(k)]
    bad += len(diff)
    print("%-60s before %4d after %4d status-before %s status-after %s DIFFER %d" % (args[i].split("c4p51-py-")[-1][:58], len(b), len(a), dict(status(b)), dict(status(a)), len(diff)))
    for k in diff[:5]:
        print("   ", k)
print("TOTAL DIFFER", bad)
sys.exit(1 if bad else 0)
