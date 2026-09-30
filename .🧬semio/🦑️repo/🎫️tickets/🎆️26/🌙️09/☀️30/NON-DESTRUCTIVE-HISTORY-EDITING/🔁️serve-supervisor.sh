#!/usr/bin/env bash
# 🔁️ Keeps the puzzle 2d React dev serve answering on :6012 — Vite vanishes silently between probe runs
# (no error in its log) and wedges after host-edit bursts; this recycles the listener whenever the port
# stops answering. Activation (component/materialize/prepare/activate) must already be done:
# `bun nx run @semio-tech/framework-os-dev:activate-puzzle2d-react-dev`.
set -u
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
TK="$(cd "$(dirname "$0")" && pwd)"
PORT="${S_OS_PORT:-6012}"
LOG="$TK/🗑️generated/serve-$PORT-supervised.txt"
DEV="$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
alive() { [ "$(curl -s -o /dev/null -w '%{http_code}' --max-time 15 "http://127.0.0.1:$PORT/" 2>/dev/null)" = "200" ]; }
start() {
  for p in $(lsof -nP -iTCP:"$PORT" -sTCP:LISTEN -t 2>/dev/null); do kill -TERM "$p" 2>/dev/null; sleep 3; kill -0 "$p" 2>/dev/null && kill -KILL "$p"; done
  (cd "$DEV" && NODE_OPTIONS= nohup bun ./📜️script.ts serve puzzle2d react dev >> "$LOG" 2>&1 < /dev/null &)
  echo "$(date '+%H:%M:%S') started serve" >> "$LOG.events"
}
alive || start
while true; do
  sleep 20
  if ! alive; then sleep 10; alive || { echo "$(date '+%H:%M:%S') unresponsive → recycle" >> "$LOG.events"; start; }; fi
done
