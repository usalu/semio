#!/usr/bin/env bash
# 🧪️ Runs suite files of the React target or the site one by one through the s1 runner scripts (each run under its own capacity grant) and appends one line per file to `🗑️generated/linux-s6/s6-<suite>-summary.tsv`: case folder, exit code, passed, failed, skipped, seconds. Usage: [SEMIO_TEST_LEVEL=<level>] [TEST_TIMEOUT_S=<s>] suite.sh <react|site> <case folder>...
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
OUT="$HERE/../../🗑️generated/linux-s6"
SUITE="$1"; shift
mkdir -p "$OUT"
for CASE in "$@"; do
  NAME="${CASE//[^a-z0-9-]/}"
  JSON="$OUT/s6-$SUITE-$NAME.json"
  rm -f "$JSON"
  STARTED=$(date +%s)
  "$HERE/../s1/$SUITE-test.sh" "/$CASE/" --reporter=default --reporter=json --outputFile.json="$JSON" > "$OUT/s6-$SUITE-$NAME.log" 2>&1
  CODE=$?
  COUNTS=$(node -e 'try{const r=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8"));console.log([r.numPassedTests,r.numFailedTests,r.numPendingTests+(r.numTodoTests??0)].join("\t"))}catch{console.log("?\t?\t?")}' "$JSON")
  printf '%s\t%s\t%s\t%s\t%s\n' "$NAME" "$CODE" "$COUNTS" "$(( $(date +%s) - STARTED ))" "$(date +%H:%M:%S)" >> "$OUT/s6-$SUITE-summary.tsv"
done
