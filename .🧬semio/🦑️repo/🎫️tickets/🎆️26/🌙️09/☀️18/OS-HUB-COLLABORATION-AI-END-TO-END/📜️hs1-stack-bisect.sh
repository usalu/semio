#!/usr/bin/env zsh
# 📏️ HS1 — measures the document-socket path's real stack requirement WITHOUT a rebuild.
#
# `WorkerPool::new` spawns its workers through `thread::Builder` with no `stack_size`, so their
# budget is `std::thread::min_stack()` — i.e. `RUST_MIN_STACK`, defaulting to 2 MiB. Booting the same
# binary under a raised `RUST_MIN_STACK` and running 🐍️hs1-socket-repro.ts therefore prices the path
# in bytes: the smallest value at which the socket session survives IS the requirement.
#
# Usage: 📜️hs1-stack-bisect.sh <bytes> [binary] [port]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
BYTES="${1:?stack bytes}"
BIN="${2:-$ROOT/.🧬semio/🦑️repo/⚡️cache/hs1/os-hub-gm1}"
PORT="${3:-7631}"
SPACE=01a0c00f-4f3c-7834-a7e6-2ccf9de925db
DOCUMENT=artifact-2fb248125b8b2b4d56de25933d30ed21
LOG="$TICKET/🗑️generated/hs1-stack-$BYTES.txt"
cd "$ROOT" || exit 1
export RUST_MIN_STACK="$BYTES" HS1_HUB_LOG="$LOG"
for attempt in 1 2 3 4; do
  nohup zsh "$TICKET/📜️hs1-hub-hold.sh" "$PORT" "$BIN" > /dev/null 2>&1 &
  disown
  for _ in $(seq 1 30); do
    sleep 5
    curl -s --max-time 3 "http://127.0.0.1:$PORT/readyz" > /dev/null 2>&1 && break
  done
  curl -s --max-time 5 "http://127.0.0.1:$PORT/readyz" > /dev/null 2>&1 && break
  echo "BOOT-RETRY $attempt (artifact-authority 30 s deadline under fleet load)"
done
PID=$(lsof -nP -iTCP:"$PORT" -sTCP:LISTEN -t 2>/dev/null | head -1)
echo "RUST_MIN_STACK=$BYTES hubPid=${PID:-none}"
[ -z "${PID:-}" ] && { echo "VERDICT $BYTES BOOT-FAILED"; exit 2; }
bun "$TICKET/🐍️hs1-socket-repro.ts" "http://127.0.0.1:$PORT" "$SPACE" "$DOCUMENT" "s.gis.gismap@1/*#editor" "${HS1_HOLD_MS:-20000}" "${HS1_EDITS:-0}"
REPRO=$?
OVERFLOW=$(grep -c "overflowed its stack" "$LOG" 2>/dev/null || echo 0)
ALIVE=$(lsof -nP -iTCP:"$PORT" -sTCP:LISTEN -t 2>/dev/null | head -1)
echo "VERDICT $BYTES reproExit=$REPRO overflowLines=$OVERFLOW hubStillListening=${ALIVE:-none}"
