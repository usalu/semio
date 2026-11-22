#!/usr/bin/env bash
# Serves a built demonstrator site folder at the root of a local static server and runs the eight pane boot checks in English and German. Usage: verify-static.sh <site-folder> [port]
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$T/../../../../../../.." && pwd)"; G="$T/🗑️generated"; mkdir -p "$G"
[ $# -ge 1 ] || { echo "usage: verify-static.sh <site-folder> [port]" >&2; exit 2; }
SITE="$1"; PORT="${2:-8765}"
[ -d "$SITE" ] || { echo "site folder missing: $SITE" >&2; exit 2; }
[ -f "$SITE/index.html" ] || { echo "index.html missing in: $SITE" >&2; exit 2; }
if python3 -c 'import socket, sys; s = socket.socket(); s.settimeout(1); sys.exit(0 if s.connect_ex(("127.0.0.1", int(sys.argv[1]))) == 0 else 1)' "$PORT"; then
  echo "port $PORT is already served by another process; refusing to verify against a stale origin" >&2; exit 1
fi
python3 "$T/static-origin.py" "$SITE" "$PORT" > "$G/static-server.log" 2>&1 & SERVER=$!
trap 'kill "$SERVER" 2>/dev/null || true' EXIT
for _ in $(seq 1 60); do
  kill -0 "$SERVER" 2>/dev/null || { echo "static server exited early:" >&2; cat "$G/static-server.log" >&2; exit 1; }
  curl -sf "http://127.0.0.1:$PORT/" >/dev/null && break
  sleep 0.5
done
curl -sf "http://127.0.0.1:$PORT/" >/dev/null || { echo "static server not ready on port $PORT after 30s" >&2; exit 1; }
DEMO="$ROOT/♻️mit-bestand/🧺️demonstrator"
cd "$DEMO" || exit 1
CLI="$(dirname "$(node -p 'require.resolve("@playwright/test/package.json")')")/cli.js"
report="$G/static-playwright.json"
rm -f "$report"
repo capacity run --memory=4096 -- env PLAYWRIGHT_JSON_OUTPUT_FILE="$report" DEMONSTRATOR_VERIFY_CDN=1 PLAYWRIGHT_BASE_URL="http://cdn.localhost:$PORT/" PLAYWRIGHT_BROWSERS_PATH="$ROOT/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright" \
  node "$CLI" test --config "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts" --grep " boot$" --output "$G/pw" --reporter=list,json 2>&1 | tee "$G/static-verify.log"
python3 "$T/verify-boot-matrix.py" "$report" "$G/static-server.log"
