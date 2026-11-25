#!/usr/bin/env bash
# 🔎️ Runs `idle-probe.mjs` against the development server of the live tree (s1's `dev.sh`), server and browser under one capacity grant; samples, screenshots and logs go to `🗑️generated/linux-s7/<name>`. Usage: idle-probe.sh <name> [seconds] [route] [port]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
if [ "${1:-}" = "--inside" ]; then
  NAME="$2" SECONDS_IDLE="$3" ROUTE="$4" PORT="$5"
  OUT="$HERE/../../🗑️generated/linux-s7/$NAME"
  mkdir -p "$OUT"
  "$HERE/../s1/dev.sh" "$PORT" > "$OUT/s7-dev-server.log" 2>&1 &
  SERVER=$!
  trap 'kill "$SERVER" 2>/dev/null || true' EXIT
  for _ in $(seq 1 180); do curl -s -o /dev/null "http://127.0.0.1:$PORT/" && break; sleep 1; done
  node "$HERE/idle-probe.mjs" "http://127.0.0.1:$PORT/" "$OUT" "$ROUTE" "$SECONDS_IDLE"
  exit $?
fi
cd "$ROOT"
repo capacity run --memory="${PROBE_MEMORY_MB:-4500}" -- timeout "${PROBE_TIMEOUT_S:-900}" "$HERE/idle-probe.sh" --inside "$1" "${2:-160}" "${3:-}" "${4:-6277}"
