#!/bin/zsh
# ⚖️ LB: test-parity (Rust subject + reference oracle) of cases, ONE native-lane hold, build-fleet-b, nice 15. parity-fleet.sh <capture> <case…>
cd /Users/ueli/Documents/semio || exit 2
out=".tmp-ticket/wp-lb/generated/$1.txt"; shift
export SEMIO_TEST_LEVEL=exhaustive NX_DAEMON=false PYTHONDONTWRITEBYTECODE=1 CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb/target"
echo "QUEUED $(date '+%H:%M:%S')" > "$out"
zsh .tmp-ticket/📜️fleet-mutex.sh native lb -- zsh -c '
out="$1"; shift
for c in "$@"; do
  s=$(date +%s)
  nice -n 15 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx run @semio-tech/repo-test-domain:test-parity --outputStyle=stream -- --case "$c" 2>&1 | sed "s/\x1b\[[0-9;]*m//g" > "$out.case.tmp"
  echo "$c $(( $(date +%s) - s ))s $(/usr/bin/grep -o "executed=[0-9]* passed=[0-9]* failed=[0-9]* errored=[0-9]* parity=[0-9/]*" "$out.case.tmp" | tail -1)" >> "$out"
  python3 - ".🧬semio/🦑️repo/⚡️cache/tests/reports/latest/📤️results.jsonl" >> "$out" <<PY
import json, sys
for line in open(sys.argv[1], encoding="utf-8"):
    row = json.loads(line) if line.strip() else None
    if row and row.get("status") != "passed":
        print("  ", row.get("status"), row.get("implementation"), row.get("role"), row.get("scenario"), json.dumps(row.get("diagnostics"), ensure_ascii=False)[:400])
PY
  /usr/bin/grep -E "^error(\[|:)" "$out.case.tmp" | head -10 >> "$out"
done
rm -f "$out.case.tmp"
echo "DONE $(date "+%H:%M:%S")" >> "$out"
' lb "$out" "$@"
