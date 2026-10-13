#!/bin/bash
# 🌎️ Checks the hub workspace (minus the native host and auth client) natively or at wasm32-wasip2; prints error summary.
# usage: 🌎️hub-check.sh [native|wasm]
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
MODE="${1:-wasm}"; EXTRA=(); [ "$MODE" = wasm ] && EXTRA=(--target wasm32-wasip2)
LOG="$T/🗑️generated/lane-c/hub-$MODE.log"
s=$(date +%s)
bash "$T/lane-c-semio-split/🦀️cargo.sh" 590 check --manifest-path 🌎️hub/Cargo.toml --workspace --exclude semio-hub --exclude semio-hub-auth-client "${EXTRA[@]}" --message-format short > "$LOG" 2>&1
rc=$?
echo "rc=$rc secs=$(( $(date +%s) - s )) units=$(grep -c '^    Checking\|^   Compiling' "$LOG") warnings=$(grep -c ': warning' "$LOG") errors=$(grep -cE ': error' "$LOG")"
grep -E '^error: could not compile' "$LOG" | head -20
grep -E ': error' "$LOG" | sed -E 's/:[0-9]+:[0-9]+: /: /' | sort | uniq -c | sort -rn | head -${ERRS:-30} | cut -c1-260
