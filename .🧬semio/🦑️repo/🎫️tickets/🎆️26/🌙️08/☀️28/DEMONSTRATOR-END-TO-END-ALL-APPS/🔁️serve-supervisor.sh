#!/usr/bin/env bash
# 🔁️ Keeps the demonstrator dev server answering on :6029 — recycles the listener when Vite wedges after host-edit bursts / registry regeneration.
set -u
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
TK="$(cd "$(dirname "$0")" && pwd)"
PORT="${MIT_BESTAND_DEMONSTRATOR_PORT:-6029}"
LOG="$TK/🗑️generated/serve-$PORT-supervised.txt"
alive() { [ "$(curl -s -o /dev/null -w '%{http_code}' --max-time 15 "http://127.0.0.1:$PORT/" 2>/dev/null)" = "200" ]; }
start() {
  for p in $(lsof -nP -iTCP:"$PORT" -sTCP:LISTEN -t 2>/dev/null); do kill -TERM "$p" 2>/dev/null; sleep 3; kill -0 "$p" 2>/dev/null && kill -KILL "$p"; done
  (cd "$ROOT/♻️mit-bestand/🧺️demonstrator" && MIT_BESTAND_DEMONSTRATOR_PORT="$PORT" DEVELOPER_DIR=/Library/Developer/CommandLineTools NODE_OPTIONS= nohup bun ./🔨️modules/🧩️runtime/📜️script.ts serve >> "$LOG" 2>&1 < /dev/null &)
  echo "$(date '+%H:%M:%S') started serve" >> "$LOG.events"
}
alive || start
while true; do
  sleep 30
  if ! alive; then sleep 10; alive || { echo "$(date '+%H:%M:%S') unresponsive → recycle" >> "$LOG.events"; start; }; fi
done
