#!/usr/bin/env zsh
# ♻️ Slice C8 — supervise the 7671 hold until the machine lets the hub finish booting.
#
# Under fleet load (measured: load average 227–270, 47 cargo / 19 rustc) `os-hub` needs longer than
# both readiness waits allow — `🐍️ds1-hub-hold.ts` gives up on a 30 s stall bound and
# `🐍️pr1-hub-hold.ts` on a 300 s absolute one — and the holder's throw tears the child down with it.
# This restarts the patient hold until `/readyz` answers 200, then stays out of the way.
# Kill by the pid in `🗑️generated/c8-hub-supervisor-pid.txt`.
# Usage: 📜️c8-hub-hold-retry.sh <port> <absoluteDataDir> <absoluteBinaryPath>
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT="$1"; DATA="$2"; BIN="$3"
export OS_HUB_CREDENTIAL_SIGN_IN=true
attempt=0
while [ "$attempt" -lt 40 ]; do
  attempt=$((attempt + 1))
  code=$(curl -s -m 5 -o /dev/null -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  if [ "$code" = "200" ]; then
    sleep 30
    continue
  fi
  echo "=== attempt $attempt at $(date -Iseconds) load=$(uptime | sed 's/.*averages: //') ===" >> "$GEN/c8-hub-supervisor.txt"
  ( cd "$ROOT" && bun "$TICKET/🐍️pr1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$GEN/c8-hub-7671.txt" 2>&1
  sleep 20
done
