#!/usr/bin/env python3
"""📋 Writes registry.json: every new test file of lawcov-build, live or staged, library-relative with their real variation selectors, with its patch group."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
installed = json.load(open(os.path.join(HERE, "..", "installed.json"), encoding="utf8"))
manifest = json.load(open(os.path.join(HERE, "manifest.json"), encoding="utf8"))
FE = "️"
tests = {k: "main" for k in installed if k.endswith("🟦" + FE + ".ts")}
for row in manifest.values():
    for f, d in row["files"].items():
        if f == "test.ts" and not row.get("overwrite"):
            tests[d] = "fixes" if row.get("stage") == "fixes" else row.get("group", "main")
json.dump({"version": 1, "tests": [{"path": p, "group": g} for p, g in sorted(tests.items())]}, open(os.path.join(HERE, "registry.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(tests))
