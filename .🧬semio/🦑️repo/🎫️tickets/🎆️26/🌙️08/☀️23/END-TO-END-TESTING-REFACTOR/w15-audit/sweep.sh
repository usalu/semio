#!/bin/bash
# 🧹 Per-owner `oracle exhaustive` sweep. One owner per invocation, so a single case's
# 900 s timeout loses that owner rather than the whole run.
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w15-audit/oracle-sweep"
mkdir -p "$OUT"
while IFS= read -r owner; do
  [ -z "$owner" ] && continue
  slug=$(echo "$owner" | tr '/' '-')
  bun ./📜️script.ts oracle exhaustive --owner "$owner" > "$OUT/$slug.txt" 2>&1
  echo "EXIT=$?" >> "$OUT/$slug.txt"
done
