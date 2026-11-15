#!/bin/sh
# waits for the previous loop (validator-loop.sh), then runs the catalog check every 30 minutes for three hours (6 runs), niced; after each run the Trend, Ready, Thermal only and Geometry lists of validator.md
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
while pgrep -f validator-loop.sh >/dev/null; do sleep 30; done
date > 🗑️generated/validator3/loop.started
for i in 1 2 3 4 5 6; do
  start=$(date +%s)
  nice -n 15 "$BUN" schema-catalog-check.ts --workers 3 > "🗑️generated/validator3/catalog-run-$i.log" 2>&1
  end=$(date +%s)
  python3 validator3-lists.py $((end - start)) >> "🗑️generated/validator3/lists.log" 2>&1
  [ "$i" -lt 6 ] && sleep $((1800 - (end - start) > 60 ? 1800 - (end - start) : 60))
done
date > 🗑️generated/validator3/loop.done
