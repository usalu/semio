#!/usr/bin/env bash
# ✅️ Last verification of every staffage suite except 🧊️model-catalog (run by final-lane.sh): verify.sh <lane 0|1> <lanes>, resumable, one capacity request per suite, results in <U>/🗑️generated/closing/verify/
set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
U="$(cd "$HERE/.." && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
S="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun 2>/dev/null | head -1)"
F="$U/🗑️generated/closing/verify"
mkdir -p "$F"
i=0
for dir in "$S"/🧪️tests/*/; do
  suite="$(basename "$dir")"
  case "$suite" in node_modules|🎚️config|🧰️support|🧊️model-catalog) continue ;; esac
  ls "$dir"🟦️*.ts >/dev/null 2>&1 || ls "$dir"*/🟦️*.ts >/dev/null 2>&1 || continue
  i=$((i + 1)); [ $((i % $2)) -eq "$1" ] || continue
  [ -f "$F/$suite.json" ] && continue
  limit=5400; memory=1200
  case "$suite" in 🧊️catalog-silhouette) limit=14400; memory=3600 ;; 🧊️three-views|🔭️multiview-coverage) memory=3600 ;; esac
  started=$(date +%s)
  (cd "$ROOT" && repo capacity run --memory=$memory -- timeout $limit "$BUN" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --reporter=default --reporter=json --outputFile.json="$F/$suite.json" --config "$S/🧪️tests/🎚️config/🟦️.ts" "/$suite/" --testTimeout=3600000 --hookTimeout=3600000) > "$F/$suite.log" 2>&1
  code=$?
  counts="$(python3 -c "import json,sys;d=json.load(open(sys.argv[1]));print(d['numPassedTests'],'passed',d['numFailedTests'],'failed')" "$F/$suite.json" 2>/dev/null || echo "no json")"
  echo "$suite exit $code $counts seconds $(( $(date +%s) - started ))" | tee -a "$F/summary.txt"
done
