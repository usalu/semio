#!/bin/bash
# 🌎️ Checks the hub workspace (minus the native host and auth client) natively or at wasm32-wasip2; prints an error summary.
# usage: 🌎️hub-check.sh [native|wasm] [extra cargo args...]
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
MODE="${1:-wasm}"; shift; EXTRA=("$@"); [ "$MODE" = wasm ] && EXTRA+=(--target wasm32-wasip2)
mkdir -p "$T/🗑️generated/l1"; LOG="$T/🗑️generated/l1/hub-$MODE.log"
s=$(date +%s)
bash "$T/l1-library-xor-component/🦀️cargo.sh" 585 check --keep-going --manifest-path 🌎️hub/Cargo.toml --workspace --exclude semio-hub --exclude semio-hub-auth-client "${EXTRA[@]}" --message-format short > "$LOG" 2>&1
rc=$?
echo "rc=$rc secs=$(( $(date +%s) - s )) units=$(grep -c '^    Checking\|^   Compiling' "$LOG") warnings=$(grep -c ': warning' "$LOG") errors=$(grep -cE ': error' "$LOG")"
grep -E '^error: could not compile|^error: failed|^error\[|^error: ' "$LOG" | sort | uniq -c | head -20
grep -E ': error' "$LOG" | sed -E 's/:[0-9]+:[0-9]+: /: /' | sort | uniq -c | sort -rn | head -${ERRS:-30} | cut -c1-260
