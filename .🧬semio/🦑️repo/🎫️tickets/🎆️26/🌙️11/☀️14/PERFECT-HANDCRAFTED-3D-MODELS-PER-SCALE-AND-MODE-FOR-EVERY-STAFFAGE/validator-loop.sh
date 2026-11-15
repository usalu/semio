#!/bin/sh
# runs the catalog check every 30 minutes for three hours (7 runs), niced; then the trend and ready sections of validator.md
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for i in 1 2 3 4 5 6 7; do
  start=$(date +%s)
  nice -n 15 "$BUN" schema-catalog-check.ts --workers 3 > "🗑️generated/validator/catalog-run-$i.log" 2>&1
  end=$(date +%s)
  python3 validator-trend.py $((end - start)) >> "🗑️generated/validator/trend.log" 2>&1
  [ "$i" -lt 7 ] && sleep $((1800 - (end - start) > 60 ? 1800 - (end - start) : 60))
done
