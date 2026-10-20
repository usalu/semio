#!/usr/bin/env python3
# 📋 usage: verify-list.py <series> <group> <out.list>   the test files a group wrote or changed, plus the regression core of the build tool.
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
series, gid, out = sys.argv[1:4]
m = json.load(open(f"{HERE}/{series}/manifest.json"))
group = next(g for g in m["groups"] if g["id"] == gid)
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; BZ = L + "/🏗️build/🔧️tools/🏗️bazel"
rows = [f["rel"] for f in group["files"] if "/🧪️tests/" in f["rel"] and f["rel"].endswith((".ts", ".tsx")) and "/🧫️fixtures/" not in f["rel"]]
core = [BZ + "/🧪️tests/📥️actions/🟦️.ts", BZ + "/🧪️tests/📥️coverage/🟦️.ts", BZ + "/🧪️tests/📥️fingerprint/🟦️.ts", BZ + "/🧪️tests/📥️sources/🟦️.ts", BZ + "/🧪️tests/📥️workspace/🟦️.ts", BZ + "/🏃️execution/🧪️tests/🟦️.ts", BZ + "/💾️storage/🧪️tests/🟦️.ts", L + "/🏗️build/⚙️operations/🧪️tests/🟦️.ts", L + "/🏗️build/🧭️selection/🧪️tests/🟦️.ts"]
seen = []
for r in rows + core:
    if r not in seen and os.path.exists("/Users/ueli/Documents/semio/" + r):
        seen.append(r)
open(out, "w", encoding="utf8").write("\n".join(seen) + "\n")
print(len(seen), "files")
