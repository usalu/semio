#!/usr/bin/env bash
# 🔁️ Keeps a puzzle React dev serve answering — Vite vanishes silently between probe runs (no error in
# its log) and wedges after host-edit bursts; this recycles the listener by pid whenever the port stops
# answering. Usage: `🔁️serve-supervisor.sh [variant] [port]`, default `puzzle5d 6014`
# (`puzzle3d 6013`, `puzzle2d 6012`). Activation must already be done by the coordinator:
# `bun nx run @semio-tech/framework-os-dev:activate-puzzle5d-react-dev`.
# nohup+disown friendly: `nohup bash 🔁️serve-supervisor.sh puzzle5d 6014 >/dev/null 2>&1 & disown`.
set -u
VARIANT="${1:-${SEMIO_PLAYGROUND_VARIANT:-puzzle5d}}"
PORT="${2:-${S_OS_PORT:-6014}}"
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
TK="$(cd "$(dirname "$0")" && pwd)"
LOG="$TK/🗑️generated/serve-$PORT-supervised.txt"
EVENTS="$TK/🗑️generated/serve-$PORT-supervised-events.txt"
DEV="$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
mkdir -p "$TK/🗑️generated"
alive() { [ "$(curl -s -o /dev/null -w '%{http_code}' --max-time 15 "http://127.0.0.1:$PORT/" 2>/dev/null)" = "200" ]; }
start() {
  for p in $(lsof -nP -iTCP:"$PORT" -sTCP:LISTEN -t 2>/dev/null); do kill -TERM "$p" 2>/dev/null; sleep 3; kill -0 "$p" 2>/dev/null && kill -KILL "$p"; done
  (cd "$DEV" && NODE_OPTIONS= S_OS_PORT="$PORT" SEMIO_RENDERER=react nohup bun ./📜️script.ts serve "$VARIANT" react dev >> "$LOG" 2>&1 < /dev/null &)
  echo "$(date '+%H:%M:%S') started serve $VARIANT react dev on :$PORT" >> "$EVENTS"
}
echo "$(date '+%H:%M:%S') supervising $VARIANT on :$PORT" >> "$EVENTS"
alive || start
while true; do
  sleep 20
  if ! alive; then sleep 10; alive || { echo "$(date '+%H:%M:%S') unresponsive → recycle" >> "$EVENTS"; start; }; fi
done
