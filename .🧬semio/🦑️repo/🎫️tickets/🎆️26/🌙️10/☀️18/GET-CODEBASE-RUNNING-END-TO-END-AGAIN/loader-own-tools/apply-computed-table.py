#!/usr/bin/env python3
"""apply-computed-table.py: runs computed-table.ts and rewrites ComputedImportSites rows (roots kept)."""
import json, subprocess, sys
root = "/Users/ueli/Documents/semio/"
C = root + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/"
p = root + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json"
out = subprocess.run(["nice", "-n", "10", "bun", C + "loader-own-tools/computed-table.ts"], capture_output=True, text=True, cwd=root)
sys.stderr.write(out.stderr)
rows = json.loads(out.stdout)
t = open(p, encoding="utf8").read(); d = json.loads(t)
assert json.dumps(d, ensure_ascii=False, indent=2) + "\n" == t
d["$defs"]["ComputedImportSites"]["const"] = rows
open(p, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
print("rows", len(rows))
