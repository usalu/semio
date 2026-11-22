#!/bin/sh
# 🏃 Author 12: draws the 2D shapes of a candidate tree and validates the whole candidate document: rework-12-run.sh <slug> [--volumes]; logs in 🗑️generated/rework-12/<slug>/.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug=$1; shift
D="🗑️generated/rework-12/$slug"
"$BUN" rework-12-draw2d.ts "$D/pre.json" "$D/plan.json" "$D/🔣️.json" > "$D/draw.log" 2>&1 || { echo "DRAW FAILED"; tail -5 "$D/draw.log"; exit 1; }
start=$(date +%s)
"$BUN" r3-validate.ts --file "$D/🔣️.json" "$@" > "$D/validate.log" 2>&1
echo "[DEBUG] validated $slug in $(( $(date +%s) - start )) s"
tail -1 "$D/validate.log" | cut -c1-300
grep -v "^VALID\|^INVALID" "$D/validate.log" | awk '{k=$1" "$2; c[k]++; if (c[k]<=2) print substr($0,1,460)} END {for (k in c) print c[k], k}' | head -60
