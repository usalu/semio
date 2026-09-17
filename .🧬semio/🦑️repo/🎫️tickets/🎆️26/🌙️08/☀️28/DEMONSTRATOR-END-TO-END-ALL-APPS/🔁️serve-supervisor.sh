#!/usr/bin/env bash
# 🔁️ Keeps the demonstrator dev server answering on :6029 — recycles the listener when Vite wedges after host-edit bursts / registry regeneration.
set -u
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
TK="$(cd "$(dirname "$0")" && pwd)"
PORT="${MIT_BESTAND_DEMONSTRATOR_PORT:-6029}"
LOG="$TK/🗑️generated/serve-$PORT-supervised.txt"
# ⏱️ 2026-09-17: a landing boot streams several hundred MB of wasm (8 panes incl. energy 111 MB + fem 81 MB); under load the root fetch
# legitimately takes >15 s, and recycling then kills every in-flight pane boot (shells flip to error, page force-reloads). Be tolerant:
# a 45 s budget per probe and THREE consecutive misses (~2 min) before recycling.
alive() { [ "$(curl -s -o /dev/null -w '%{http_code}' --max-time 45 "http://127.0.0.1:$PORT/" 2>/dev/null)" = "200" ]; }
start() {
  for p in $(lsof -nP -iTCP:"$PORT" -sTCP:LISTEN -t 2>/dev/null); do kill -TERM "$p" 2>/dev/null; sleep 3; kill -0 "$p" 2>/dev/null && kill -KILL "$p"; done
  (cd "$ROOT/♻️mit-bestand/🧺️demonstrator" && MIT_BESTAND_DEMONSTRATOR_PORT="$PORT" DEVELOPER_DIR=/Library/Developer/CommandLineTools NODE_OPTIONS= nohup bun ./🔨️modules/🧩️runtime/📜️script.ts serve >> "$LOG" 2>&1 < /dev/null &)
  echo "$(date '+%H:%M:%S') started serve" >> "$LOG.events"
}
alive || start
while true; do
  sleep 30
  if ! alive; then
    echo "$(date '+%H:%M:%S') miss 1" >> "$LOG.events"; sleep 20
    if ! alive; then
      echo "$(date '+%H:%M:%S') miss 2" >> "$LOG.events"; sleep 20
      alive || { echo "$(date '+%H:%M:%S') unresponsive ×3 → recycle" >> "$LOG.events"; start; }
    fi
  fi
done
