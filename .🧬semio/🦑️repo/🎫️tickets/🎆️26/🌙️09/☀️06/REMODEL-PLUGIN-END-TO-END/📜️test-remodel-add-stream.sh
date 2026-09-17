#!/bin/zsh
# 🧪 Focused remodel tests: add-stream + set-active-example (ticket 26/09/06/REMODEL-PLUGIN-END-TO-END).
# ♻️ Retries while the host's swap exhaustion SIGKILLs (137) the cargo process mid-wait; -j 4 keeps our footprint small.
cd /Users/ueli/Documents/semio || exit 1
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🗑️generated/test-remodel-add-stream.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
for attempt in 1 2 3 4 5 6 7 8; do
  echo "attempt=$attempt $(date -u +%FT%TZ)" >> "$LOG"
  cargo test -p semio-s-artifact-remodel-remodeling --lib -j 4 -- add_stream::tests set_active_example::tests --test-threads=2 >> "$LOG" 2>&1
  code=$?
  echo "attempt-exit=$code $(date -u +%FT%TZ)" >> "$LOG"
  [ "$code" -ne 137 ] && break
  sleep 90
done
echo "exit=$code $(date -u +%FT%TZ)" >> "$LOG"
