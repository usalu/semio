#!/usr/bin/env bash
# usage: rowstore-budgets.sh <MiB>...   settles the 6,707 real closures once per reach memo budget, a log with the post-chunk heap and the seconds per budget
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/INCREMENTAL-MULTI-AGENT-BUILD-GRAPH-SERVICE"; D="$PWD/$T/🗑️generated/rowstore"
for mb in "$@"; do
  start=$(date +%s)
  SEMIO_REACH_MEMO_BYTES=$((mb*1048576)) ROWSTORE_CHUNKS=1 repo tools exec --light bun -- run "./$T/🧰️scripts/rowstore-measure.ts" 6707 "b$mb" "$D" > "$D/b$mb.txt" 2>&1
  echo "SECONDS $(( $(date +%s)-start ))" >> "$D/b$mb.txt"
done
echo finished > "$D/budgets.done"
