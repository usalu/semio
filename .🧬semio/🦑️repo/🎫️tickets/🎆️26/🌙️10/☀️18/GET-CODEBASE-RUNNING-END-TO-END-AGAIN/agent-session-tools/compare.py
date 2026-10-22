#!/usr/bin/env python3
"""Compares two battery result files (jsonl from battery.py): per test file the pass and fail counts of the base and the new overlay, and every test that fails only in the new one. Exit code 1 when one exists."""
import json, sys
load = lambda path: {json.loads(l)["file"]: json.loads(l) for l in open(path, encoding="utf8") if l.startswith("{")}
base, new = load(sys.argv[1]), load(sys.argv[2])
broken = 0
for file, row in new.items():
    old = base.get(file, {})
    extra = sorted(set(row.get("failing", [])) - set(old.get("failing", [])))
    print(file.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/", "M/").replace("🧰️framework/🔨️modules/🏃️process/", "F/")[-72:], "base", old.get("pass", "-"), old.get("fail", "-"), "new", row.get("pass"), row.get("fail"), "NEWLY FAILING " + str(extra) if extra else "")
    broken += bool(extra)
sys.exit(1 if broken else 0)
