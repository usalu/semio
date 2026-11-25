#!/bin/zsh
# 🚦️ Starts one stage of `nstage.sh` (or, with STAGE_SCRIPT=stage.sh, of the proof stages) through the machine-wide memory queue from the repo root and waits for it: nq.sh <MB> <timeout-s> <stage> <id>…   Reserves: 7000 matrix, 5000 check, 2500 vitest and measure, 1000 gate, scan, build, apply, sheets.
BATCH=MA source "${0:A:h}/../MR/env.sh"
mb="$1"; t="$2"; shift 2; mkdir -p "$GEN/proof"
out="$GEN/proof/nq-$1-$$.out"
cd "$R" && repo capacity run --memory="$mb" -- timeout "$t" "$W/${STAGE_SCRIPT:-nstage.sh}" "$@" > "$out" 2>&1
echo "[$1] exit $?"; grep -v '^\[capacity\] waiting' "$out" | cut -c1-${CUT:-400}; rm -f "$out"
