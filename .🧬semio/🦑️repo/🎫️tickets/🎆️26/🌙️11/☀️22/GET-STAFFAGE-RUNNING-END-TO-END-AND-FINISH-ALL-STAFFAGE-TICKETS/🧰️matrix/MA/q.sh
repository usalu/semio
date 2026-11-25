#!/bin/zsh
# 🚦️ Starts one stage of `stage.sh` through the machine-wide memory queue from the repo root and waits for it (the wait in the queue is part of the timeout): q.sh <MB> <timeout-s> <stage> <id>…   Reserves: 7000 matrix, 5000 check, 2500 silhouette / three-views / physics / any vitest run and measure, 1000 gate, scan, apply, sheets.
BATCH=MA source "${0:A:h}/../MR/env.sh"
mb="$1"; t="$2"; shift 2; mkdir -p "$GEN/proof"
cd "$R" && repo capacity run --memory="$mb" -- timeout "$t" "$W/stage.sh" "$@" > "$GEN/proof/q-$1.out" 2>&1
echo "[$1] exit $?"; cat "$GEN/proof/q-$1.out" | cut -c1-300
