#!/bin/bash
# 🌎️ One bounded slice of the hub wasm32-wasip2 workspace check (E1 verification), private E1 dirs, through the cargo slot.
# The whole cargo process group is interrupted after <seconds> so a foreground tool call never outlives its deadline;
# rerunning resumes from the finished check units. Prints the last progress line, errors, and DONE when cargo finished.
# usage: 🌎️hub-check.sh <log> [seconds=540]
REPO=/Users/ueli/Documents/semio; cd "$REPO" || exit 2
E1="$REPO/.🧬semio/🦑️repo/⚡️cache/e1/os-kernel"; LOG=$1; SECS=${2:-540}
set -m
env CARGO_TARGET_DIR="$E1/t" CARGO_BUILD_BUILD_DIR="$E1/b" CARGO_INCREMENTAL=0 CARGO_TERM_COLOR=never \
  "$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" e1-os-kernel -- \
  cargo check --manifest-path "🌎️hub/Cargo.toml" --workspace --exclude semio-hub --exclude semio-hub-auth-client --target wasm32-wasip2 --message-format short > "$LOG" 2>&1 &
PID=$!; START=$(date +%s)
while kill -0 $PID 2>/dev/null; do
  if [ $(( $(date +%s) - START )) -ge "$SECS" ]; then kill -INT -- -$PID 2>/dev/null; sleep 5; kill -TERM -- -$PID 2>/dev/null; wait $PID; echo "INTERRUPTED after ${SECS}s"; break; fi
  sleep 2
done
wait $PID 2>/dev/null; RC=$?
grep -E '^error' "$LOG" | sort | uniq -c | head -20
grep -cE '^\s+(Checking|Compiling)' "$LOG" | sed 's/^/units-this-slice: /'
grep -q 'Finished' "$LOG" && echo "DONE rc=$RC"
