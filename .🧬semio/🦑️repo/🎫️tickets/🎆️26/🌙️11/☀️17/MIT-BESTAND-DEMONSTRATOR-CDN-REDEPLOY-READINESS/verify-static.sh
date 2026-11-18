#!/usr/bin/env bash
# Serves a built demonstrator site folder at the root of a local static server and runs the eight pane boot checks in English and German. Usage: verify-static.sh <site-folder> [port]
set -euo pipefail
ROOT=/Users/ueli/Documents/semio
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; mkdir -p "$G"
[ $# -ge 1 ] || { echo "usage: verify-static.sh <site-folder> [port]" >&2; exit 2; }
SITE="$1"; PORT="${2:-8765}"
[ -d "$SITE" ] || { echo "site folder missing: $SITE" >&2; exit 2; }
[ -f "$SITE/index.html" ] || { echo "index.html missing in: $SITE" >&2; exit 2; }
python3 -m http.server "$PORT" --bind 127.0.0.1 --directory "$SITE" > "$G/static-server.log" 2>&1 & SERVER=$!
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
PLAYWRIGHT_BASE_URL="http://127.0.0.1:$PORT/" PLAYWRIGHT_BROWSERS_PATH="$ROOT/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright" \
  node "$CLI" test --config "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts" --grep " boot$" --output "$G/pw" --reporter=list 2>&1 | tee "$G/static-verify.log" || true
exit "${PIPESTATUS[0]}"
