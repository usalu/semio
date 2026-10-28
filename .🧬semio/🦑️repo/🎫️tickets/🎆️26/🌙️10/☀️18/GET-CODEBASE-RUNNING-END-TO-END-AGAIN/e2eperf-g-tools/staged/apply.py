#!/usr/bin/env python3
"""Staged batch e2eperf-g: worker policy rows for the sharded course e2e (idempotent string patch; `apply.py` dry-runs, `apply.py --write` writes, `apply.py --revert --write` undoes).
Rows (🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🔣️.json): playwright workerMegabytes 600->1200 (measured: one Chromium worker with its pages p50 1.1 GB, max 2.8 GB; 3+1 workers peak tree 7.4-9.3 GB) wanted 1->4 maximum 4->6 oversubscription 2->1; e2e-stacks wanted 2->3 maximum 2->4.
Test (🧪️tests/🟦️.ts): open e2e-stacks workers 2 -> 3 and playwright 4. Verify: `bun test ./🧪️tests/🟦️.ts` in that directory (10 pass)."""
import sys
ROOT = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/"
PATCHES = [
    ("🔣️.json", '"workerMegabytes": 600, "wanted": 1, "maximum": 4, "oversubscription": 2 }', '"workerMegabytes": 1200, "wanted": 4, "maximum": 6, "oversubscription": 1 }'),
    ("🔣️.json", '"wanted": 2, "maximum": 2, "oversubscription": 1 }', '"wanted": 3, "maximum": 4, "oversubscription": 1 }'),
    ("🧪️tests/🟦️.ts", 'expect(open["e2e-stacks"].workers).toBe(2);', 'expect(open["e2e-stacks"].workers).toBe(3);expect(open["playwright"].workers).toBe(4);'),
]
revert, write = "--revert" in sys.argv, "--write" in sys.argv
for name, before, after in PATCHES:
    old, new = (after, before) if revert else (before, after)
    text = open(ROOT + name, encoding="utf8").read()
    if text.count(old) != 1:
        print(("already applied/reverted" if text.count(new) == 1 else "ANCHOR MISSING"), name, old[:50]); continue
    print("patch", name, old[:50], "->", new[:50])
    if write: open(ROOT + name, "w", encoding="utf8").write(text.replace(old, new))
