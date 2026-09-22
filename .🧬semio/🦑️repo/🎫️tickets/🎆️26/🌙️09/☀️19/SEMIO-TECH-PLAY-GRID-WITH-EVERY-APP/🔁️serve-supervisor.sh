#!/usr/bin/env bash
# 🔁️ Keeps the play dev server on :6033 — restarts a missing listener at once, recycles a live one only after ten minutes without a single answer (Vite wedge), never for being slow under fleet load.
set -u
ROOT="/Users/ueli/Documents/semio"
TK="$(cd "$(dirname "$0")" && pwd)"
PORT="${SEMIO_TECH_PLAY_PORT:-6033}"
LOG="${SEMIO_TECH_PLAY_SUPERVISOR_LOG:-$TK/🗑️generated/serve-$PORT-supervised.txt}"
EVENTS="$LOG.events.txt"
alive() { [ "$(curl -s -o /dev/null -w '%{http_code}' --max-time 120 "http://127.0.0.1:$PORT/" 2>/dev/null)" = "200" ]; }
listening() { [ -n "$(lsof -nP -iTCP:"$PORT" -sTCP:LISTEN -t 2>/dev/null)" ]; }
start() {
  for p in $(lsof -nP -iTCP:"$PORT" -sTCP:LISTEN -t 2>/dev/null); do kill -TERM "$p" 2>/dev/null; sleep 3; kill -0 "$p" 2>/dev/null && kill -KILL "$p"; done
  (cd "$ROOT/🏢️semio-tech/🎡️play" && SEMIO_TECH_PLAY_PORT="$PORT" SEMIO_TECH_PLAY_FROZEN="${SEMIO_TECH_PLAY_FROZEN:-true}" DEVELOPER_DIR=/Library/Developer/CommandLineTools NODE_OPTIONS= nohup bun ./🔨️modules/🧩️runtime/📜️script.ts serve >> "$LOG" 2>&1 < /dev/null &)
  echo "$(date '+%H:%M:%S') started serve" >> "$EVENTS"
  sleep 45
}
last_ok=$(date +%s)
while true; do
  if [ -f "$TK/🗑️generated/serve-restart.request" ]; then
    rm -f "$TK/🗑️generated/serve-restart.request"; echo "$(date '+%H:%M:%S') restart requested -> recycle" >> "$EVENTS"; start; last_ok=$(date +%s)
  elif ! listening; then
    echo "$(date '+%H:%M:%S') no listener -> start" >> "$EVENTS"; start; last_ok=$(date +%s)
  elif alive; then
    last_ok=$(date +%s)
  elif [ $(( $(date +%s) - last_ok )) -gt 600 ]; then
    echo "$(date '+%H:%M:%S') listener silent for 10 min -> recycle" >> "$EVENTS"; start; last_ok=$(date +%s)
  else
    echo "$(date '+%H:%M:%S') slow ($(( $(date +%s) - last_ok )) s since last answer)" >> "$EVENTS"
  fi
  sleep 30
done
