#!/bin/zsh
# 🔁️ FP13: N consecutive serial runs of the copied lib unittests binary; one capture per round.
set -u
ROOT="/Users/ueli/Documents/semio"
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
OUT="$TICKET/🗑️generated"
BIN="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-fp13/bin/fp13-lib"
export RUST_MIN_STACK=67108864
FIRST=${1:-1}
LAST=${2:-10}
cd "$ROOT" || exit 1
for round in $(seq "$FIRST" "$LAST"); do
  echo "-- round $round start $(date +%H:%M:%S) load=$(uptime | sed 's/.*averages: //')"
  "$BIN" --test-threads=1 2>&1 | grep -vE "^\[DEBUG\]" | tail -60 > "$OUT/fp13-round$round-serial.txt"
  grep -E "^test result:" "$OUT/fp13-round$round-serial.txt"
  grep -E "^---- " "$OUT/fp13-round$round-serial.txt" | head -5
done
