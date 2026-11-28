#!/bin/bash
U="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
BUN="$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
LOG="$U/🗑️generated/matrix/CX-SU"
unset SEMIO_STAFFAGE_MODEL_ASSETS
: > "$LOG/timing.tsv"
cd /Users/ueli/Documents/semio || exit 1
for suite in 🧊️catalog-silhouette 🧊️three-views 🧊️volumes-thermal 🧊️volumes-flood 🧊️volumes-connected 🧊️volume-schema 🧊️model-looks 🧊️model-batch; do
  start=$(date +%s)
  repo capacity run --memory=3000 -- timeout 5400 "$BUN" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" /$suite/ --testTimeout=3600000 --hookTimeout=3600000 > "$LOG/$suite.log" 2>&1
  code=$?
  end=$(date +%s)
  printf '%s\t%s\t%s\n' "$suite" "$code" "$((end-start))" >> "$LOG/timing.tsv"
done
echo DONE >> "$LOG/timing.tsv"
