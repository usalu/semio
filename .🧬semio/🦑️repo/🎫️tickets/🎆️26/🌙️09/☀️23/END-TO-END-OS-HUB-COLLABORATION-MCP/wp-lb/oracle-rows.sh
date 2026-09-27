#!/bin/zsh
# 🐍 LB: one case's oracle role, then its non-passed rows from the platform's latest report. oracle-rows.sh <impl> <capture> <case>
cd /Users/ueli/Documents/semio || exit 2
impl="$1"; out=".tmp-ticket/wp-lb/generated/$2.txt"; c="$3"
export SEMIO_TEST_LEVEL=exhaustive NX_DAEMON=false PYTHONDONTWRITEBYTECODE=1 CARGO_INCREMENTAL=0
bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx run @semio-tech/repo-test-domain:test-oracle --outputStyle=stream -- --implementation "$impl" --case "$c" 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | /usr/bin/grep -E '^\S*\[test\]|executed=' | tail -1 > "$out"
python3 - ".🧬semio/🦑️repo/⚡️cache/tests/reports/latest/📤️results.jsonl" >> "$out" <<'PY'
import json, sys
for line in open(sys.argv[1], encoding="utf-8"):
    row = json.loads(line) if line.strip() else None
    if row and row.get("status") != "passed":
        print(row.get("status"), row.get("scenario"), json.dumps(row.get("diagnostics"), ensure_ascii=False)[:600])
PY
