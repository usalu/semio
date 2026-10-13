#!/bin/bash
# ✅️ Type-checks the semio workspace natively (lib + lib tests), with all features or default features; prints errors and the warning count.
# usage: ✅️check-semio.sh [all|default|wasm] [extra cargo args...]
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
MODE="${1:-all}"; shift
M="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.toml"
case "$MODE" in
  all) ARGS=(--lib --tests --all-features) ;;
  default) ARGS=(--lib) ;;
  wasm) ARGS=(--lib --all-features --target wasm32-wasip2) ;;
esac
LOG="$T/🗑️generated/lane-c/check-$MODE.log"
s=$(date +%s)
bash "$T/lane-c-semio-split/🦀️cargo.sh" 580 check --manifest-path "$M" "${ARGS[@]}" "$@" --message-format short > "$LOG" 2>&1
rc=$?
echo "rc=$rc secs=$(( $(date +%s) - s )) warnings=$(grep -c ': warning' "$LOG") errors=$(grep -cE ': error' "$LOG")"
grep -E ': error|^error' "$LOG" | head -${ERRS:-40}
