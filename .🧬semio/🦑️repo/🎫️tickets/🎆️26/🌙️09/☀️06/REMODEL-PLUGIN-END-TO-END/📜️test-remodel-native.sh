#!/bin/zsh
# 🧪 Native unit tests of the remodeling artifact crate + plugin crate (ticket 26/09/06/REMODEL-PLUGIN-END-TO-END).
# ♻️ Retries while the host's swap exhaustion SIGKILLs (137) the cargo process mid-wait.
cd /Users/ueli/Documents/semio || exit 1
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🗑️generated/test-remodel-native.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
for attempt in 1 2 3 4 5 6; do
  echo "attempt=$attempt $(date -u +%FT%TZ)" >> "$LOG"
  cargo test -p semio-s-artifact-remodel-remodeling --lib -j 4 -- --test-threads=4 >> "$LOG" 2>&1
  code=$?
  echo "artifact-attempt-exit=$code $(date -u +%FT%TZ)" >> "$LOG"
  [ "$code" -ne 137 ] && break
  sleep 90
done
echo "artifact-exit=$code $(date -u +%FT%TZ)" >> "$LOG"
cargo test -p semio-s-plugin-remodel --lib -j 4 >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
