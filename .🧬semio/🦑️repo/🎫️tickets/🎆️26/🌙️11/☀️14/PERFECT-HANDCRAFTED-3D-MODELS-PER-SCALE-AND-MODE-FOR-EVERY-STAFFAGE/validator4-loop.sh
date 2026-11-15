#!/bin/sh
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
mkdir -p 🗑️generated/validator4
date > 🗑️generated/validator4/loop.started
for i in 1 2 3 4 5 6; do
  start=$(date +%s)
  nice -n 10 "$BUN" schema-catalog-check.ts --workers 4 --out validator4-catalog-check.md > "🗑️generated/validator4/catalog-run-$i.log" 2>&1
  end=$(date +%s)
  python3 validator4-lists.py $((end - start)) >> "🗑️generated/validator4/lists.log" 2>&1
  [ "$i" -lt 6 ] && sleep 900
done
date > 🗑️generated/validator4/loop.done
