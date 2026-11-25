#!/bin/zsh
# 🚦️ Runs one script of this folder in one grant of the machine-wide memory queue and waits for it: run.sh <MB> <timeout-s> <log-name> <script> <args…>   (log `$GEN/runs/<log-name>.out`)
BATCH=MA source "${0:A:h}/../MR/env.sh"
mb="$1"; t="$2"; name="$3"; script="$4"; shift 4; mkdir -p "$GEN/runs"
cd "$R" && SEMIO_LANGUAGE=en repo capacity run --memory="$mb" -- timeout "$t" "$W/$script" "$@" > "$GEN/runs/$name.out" 2>&1
e=$?; grep -v 'waiting' "$GEN/runs/$name.out" | cut -c1-${CUT:-300}; exit $e
