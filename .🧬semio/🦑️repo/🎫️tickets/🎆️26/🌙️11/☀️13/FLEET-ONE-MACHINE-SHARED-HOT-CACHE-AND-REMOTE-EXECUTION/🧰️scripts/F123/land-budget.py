#!/usr/bin/env python3
"""Stage for after 19:00 (quiet window): declare the transaction-v2 test file in the test file budgets, atomically. Take landing.lock.process only around the run; the table must stay in path order."""
import json, os, sys
P = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/📏️file-budgets/🔣️.json"
F = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts"
d = json.load(open(P, encoding="utf8"))
if any(r["path"] == F for r in d["files"]): sys.exit(0)
d["files"].append({"path": F, "megabytes": 4600, "reason": "measured process-tree peak 3411 MB and 3641 MB (loaded host): 62 concurrent cases, spawned crash-driver children and fixture git trees; 4600 MB = 3641 MB plus 25 percent headroom"})
d["files"].sort(key=lambda r: r["path"].encode("utf8"))
t = P + ".tmp123"
open(t, "w", encoding="utf8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
os.replace(t, P)
