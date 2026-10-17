#!/usr/bin/env python3
import json, sys, re
t = open(sys.argv[1], errors="replace").read()
i = t.index("\n{\n")
r = json.loads(t[i:])
print("keys", list(r))
def n(x):
    return len(x) if hasattr(x, "__len__") else x
for k, v in r.items():
    if k == "census":
        bad = [row for row in v if not row.get("withinHardBudget")]
        print("census rows", len(v), "over", len(bad))
        for row in bad[:20]: print("  OVER", {a: b for a, b in row.items() if a != "diagnostics"})
    elif isinstance(v, dict):
        print(k, {a: (n(b) if not isinstance(b, dict) else {c: n(d) for c, d in b.items()}) for a, b in v.items()})
    elif isinstance(v, list): print(k, len(v), (v[:3] if v else ""))
    else: print(k, v)
