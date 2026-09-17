#!/bin/zsh
# 📸 Restage the remodel guest wasm + materialize the react dev module (ticket 26/09/06/REMODEL-PLUGIN-END-TO-END).
# ♻️ Retries while the host's swap exhaustion SIGKILLs the wasm cargo mid-build (nx reports exit 1 with a
# "killed by signal SIGKILL" line); a completed nx run is cached, so a retry only redoes what was lost.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react S_OS_PORT=6063
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🗑️generated/activate-remodel-react.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
for attempt in 1 2 3 4 5 6; do
  echo "attempt=$attempt $(date -u +%FT%TZ)" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-remodel-react-dev >> "$LOG" 2>&1
  code=$?
  echo "attempt-exit=$code $(date -u +%FT%TZ)" >> "$LOG"
  [ "$code" -eq 0 ] && break
  grep -q "SIGKILL\|signal" "$LOG" || break
  sleep 90
done
echo "exit=$code $(date -u +%FT%TZ)" >> "$LOG"
