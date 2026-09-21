#!/usr/bin/env zsh
# ♻️ Slice C5 step 9 — a mid-edit hub RESTART that reuses the already-published trusted catalog.
#
# Stops the hold whose pid is in `🗑️generated/c5-hub-pid.txt` (by pid, never by name — preamble
# rule 15), waits for the port to be released, starts a SECOND hub process from the SAME data root
# and the SAME binary, and blocks until `/readyz` answers 200. The catalog is never republished: a
# second hub on a published root is a restart, not a bootstrap (GM1 §0).
#
# `OS_HUB_CREDENTIAL_SIGN_IN=true` is mandatory or `POST /auth/sessions` answers 403 and the two
# already-signed-in browser contexts cannot re-mint anything (GM1 §4d).
# Usage: 📜️c5-hub-restart.sh [port] [dataRoot] [binary]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7621}
DATA=${2:-"$ROOT/.🧬semio/🌐hub/jc1-boot"}
BIN=${3:-"$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-jc1/debug/os-hub"}
LOG="$GEN/c5-hub-restart.txt"
export OS_HUB_CREDENTIAL_SIGN_IN=true
: > "$LOG"

OLD=$(cat "$GEN/c5-hub-pid.txt" 2>/dev/null || echo "")
echo "=== stop hold pid=${OLD:-<none>} at $(date -Iseconds) ===" >> "$LOG"
if [ -n "$OLD" ]; then
  kill "$OLD" 2>/dev/null
  for _ in $(seq 1 30); do kill -0 "$OLD" 2>/dev/null || break; sleep 1; done
  kill -9 "$OLD" 2>/dev/null
fi
for _ in $(seq 1 60); do
  lsof -nP -iTCP:"$PORT" -sTCP:LISTEN >/dev/null 2>&1 || break
  sleep 1
done
echo "port $PORT free=$(lsof -nP -iTCP:$PORT -sTCP:LISTEN >/dev/null 2>&1 && echo no || echo yes) at $(date -Iseconds)" >> "$LOG"

( cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$LOG" 2>&1 &
NEW=$!
echo "$NEW" > "$GEN/c5-hub-pid.txt"
echo "=== new hold pid=$NEW at $(date -Iseconds) ===" >> "$LOG"
code=000
for _ in $(seq 1 120); do
  code=$(curl -s -m 5 -o /dev/null -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  sleep 2
done
echo "=== readyz http=$code at $(date -Iseconds) ===" >> "$LOG"
echo "C5-HUB-RESTART old=${OLD:-<none>} new=$NEW readyz=$code"
[ "$code" = "200" ] || exit 1
disown
