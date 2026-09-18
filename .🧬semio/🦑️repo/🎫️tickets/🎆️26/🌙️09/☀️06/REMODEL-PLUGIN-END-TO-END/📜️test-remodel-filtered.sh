#!/bin/zsh
# 🧪 Filtered native tests of the remodeling artifact crate with SIGKILL (137) retries while the host
# swaps under peer builds (ticket 26/09/06/REMODEL-PLUGIN-END-TO-END).
# Usage: 📜️test-remodel-filtered.sh <log-name> [--ignored] <test filters…>
cd /Users/ueli/Documents/semio || exit 1
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🗑️generated/$1.txt"; shift
echo "start $(date -u +%FT%TZ)" > "$LOG"
for attempt in 1 2 3 4 5 6 7 8 9 10; do
  echo "attempt=$attempt $(date -u +%FT%TZ)" >> "$LOG"
  cargo test -p semio-s-artifact-remodel-remodeling --lib -j 2 -- --test-threads=4 --nocapture "$@" >> "$LOG" 2>&1
  code=$?
  echo "attempt-exit=$code $(date -u +%FT%TZ)" >> "$LOG"
  [ "$code" -ne 137 ] && break
  sleep 120
done
echo "exit=$code $(date -u +%FT%TZ)" >> "$LOG"
