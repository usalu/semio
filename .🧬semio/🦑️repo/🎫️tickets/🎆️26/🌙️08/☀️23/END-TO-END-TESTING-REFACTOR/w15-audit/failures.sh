#!/bin/bash
# 🩺 Re-run each owner that reported failures and capture the failing scenarios before the
# shared reports/latest can be overwritten by another session.
ROOT="/Users/ueli/Documents/semio"
OUT="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w15-audit/failures"
mkdir -p "$OUT"
cd "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
while IFS= read -r owner; do
  [ -z "$owner" ] && continue
  slug=$(echo "$owner" | tr '/' '-')
  bun ./📜️script.ts oracle exhaustive --owner "$owner" > "$OUT/$slug.log" 2>&1
  echo "EXIT=$?" >> "$OUT/$slug.log"
  python3 - "$ROOT" "$OUT/$slug.jsonl" <<'PY'
import json,sys,os
root,out=sys.argv[1],sys.argv[2]
p=os.path.join(root,".🧬semio/🦑️repo/⚡️cache/tests/reports/latest/📤️results.jsonl")
rows=[]
if os.path.exists(p):
    for line in open(p,encoding="utf-8"):
        r=json.loads(line)
        if r.get("status")!="passed": rows.append(r)
open(out,"w",encoding="utf-8").write("\n".join(json.dumps(r,ensure_ascii=False) for r in rows)+("\n" if rows else ""))
print(out, len(rows), "failing records")
PY
done
