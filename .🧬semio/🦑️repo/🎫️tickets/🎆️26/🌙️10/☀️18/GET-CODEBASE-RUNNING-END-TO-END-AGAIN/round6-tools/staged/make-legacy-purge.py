#!/usr/bin/env python3
"""Copy of the legacy-purge set at the ticket root (`r6-legacy-purge-tools`, same depth) rebuilt (build/build.py) from the tree the chain has produced by then; tool-iface is already applied there, so REQUIRE_SETS is emptied and every row is based on the live text. The edits that remove the `agents` member of tickets as a "pre-schema log" are dropped: the live ticket schema defines `agents` (durable agent sessions and their plans) as a real field, which contradicts that premise (see result-legacy-purge.md, round 6)."""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/legacy-purge-tools", "r6-legacy-purge-tools", beside=True)
patch(T + "/src/edits.py", 'REQUIRE_SETS = ["tool-iface-tools"]', "REQUIRE_SETS = []")
with open(T + "/src/edits.py", "a", encoding="utf8") as handle:
    handle.write("\n\n# round 6: the ticket `agents` member is a first-class, schema-defined field now (live schema: durable agent sessions and their plans), so the edits that treat it as a pre-schema log are dropped\nEDITS = {key: [pair for pair in pairs if callable(pair) or not (\"gent\" in pair[0] or \"gent\" in pair[1])] for key, pairs in EDITS.items()}\n")
run = subprocess.run([sys.executable, T + "/build/build.py"], capture_output=True, text=True, cwd=T)
if run.returncode:
    raise SystemExit("maker: legacy-purge build.py failed: " + (run.stdout + run.stderr)[-800:])
print("legacy-purge copy:", T, run.stdout.strip()[-200:])
