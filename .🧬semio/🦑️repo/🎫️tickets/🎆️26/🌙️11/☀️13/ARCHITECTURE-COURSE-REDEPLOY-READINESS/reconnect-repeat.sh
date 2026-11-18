#!/bin/sh
# 🔂️ Runs the reconnect-storm spec once per seed (default five; ten cycles walk every kind once) against the running static stack, one report per seed. Env: RECONNECT_SEEDS RECONNECT_CYCLES
D="$(cd "$(dirname "$0")" && pwd)"
for seed in ${RECONNECT_SEEDS:-11 12 13 14 15}; do
  RECONNECT_SEED=$seed RECONNECT_CYCLES=${RECONNECT_CYCLES:-10} RECONNECT_REPORT="$D/🗑️generated/reconnect/seed-$seed.json" "$D/reconnect-run.sh" > "$D/🗑️generated/reconnect/seed-$seed.log" 2>&1
  echo "seed $seed exit $?" >> "$D/🗑️generated/reconnect/repeat.log"
done
echo finished >> "$D/🗑️generated/reconnect/repeat.log"
