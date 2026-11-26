#!/usr/bin/env bash
# 🪁️ Typecheck of the React project and the site project one after the other in one grant. Usage: repo capacity run --memory=2200 -- typecheck-both.sh <name>
set -uo pipefail
source "$(dirname "$0")/env.sh"
NAME="$1"; mkdir -p "$OUT/$NAME"; cd "$ROOT"
for project in react site; do
  case "$project" in react) P="$S/🎯️targets/⚛️react/📦️packages/🟦️typescript" ;; site) P="$S/🌐️site/📦️packages/🟦️typescript" ;; esac
  node --max-old-space-size=2000 "$ROOT/node_modules/typescript/lib/tsc.js" --noEmit --incremental false --pretty false -p "$P/tsconfig.json" > "$OUT/$NAME/$project.txt" 2>&1
  echo "$project exit $? errors $(grep -c 'error TS' "$OUT/$NAME/$project.txt")" | tee -a "$OUT/$NAME/summary.txt"
done
