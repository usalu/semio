#!/usr/bin/env bash
# 🔎️ Starts the development server, runs one probe script against it and stops the server again. Usage: dev-probe.sh <probe.mjs> <output-directory> [probe arguments...]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
PROBE="$1" OUT="$2"; shift 2
PORT="${DEV_PORT:-6272}"
mkdir -p "$OUT"
"$HERE/dev.sh" "$PORT" > "$OUT/dev-server.log" 2>&1 &
SERVER=$!
trap 'kill "$SERVER" 2>/dev/null || true' EXIT
for _ in $(seq 1 120); do curl -s -o /dev/null "http://127.0.0.1:$PORT/" && break; sleep 1; done
node "$HERE/$PROBE" "http://127.0.0.1:$PORT/" "$OUT" "$@"
