#!/usr/bin/env bash
# Dry run of the static acceptance boot for selected panes and locales, one browser at a time. Usage: early-static-matrix.sh <site-folder> <port> "<panes>" "<locales>"
set -uo pipefail
T="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$T/../../../../../../.." && pwd)"; G="$T/🗑️generated"; mkdir -p "$G"
SITE="$1"; PORT="$2"; PANES="$3"; LOCALES="$4"
python3 -c 'import socket, sys; s = socket.socket(); s.settimeout(1); sys.exit(0 if s.connect_ex(("127.0.0.1", int(sys.argv[1]))) == 0 else 1)' "$PORT" && { echo "port $PORT busy" >&2; exit 1; }
LOG="$G/early-static-server.log"
python3 "$T/static-origin.py" "$SITE" "$PORT" > "$LOG" 2>&1 & SERVER=$!
trap 'kill "$SERVER" 2>/dev/null || true' EXIT
for _ in $(seq 1 60); do curl -sf "http://127.0.0.1:$PORT/" >/dev/null && break; sleep 0.5; done
cd "$ROOT/♻️mit-bestand/🧺️demonstrator" || exit 1
CLI="$(dirname "$(node -p 'require.resolve("@playwright/test/package.json")')")/cli.js"
for pane in $PANES; do for loc in $LOCALES; do
  before=$(wc -l < "$LOG"); start=$(date +%s)
  repo capacity run --memory="${MEM:-4096}" -- env DEMONSTRATOR_VERIFY_CDN=1 PLAYWRIGHT_BASE_URL="http://cdn.localhost:$PORT/" PLAYWRIGHT_BROWSERS_PATH="$ROOT/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright" \
    node "$CLI" test --config "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts" --project "$loc" --grep "$pane boot\$" --output "$G/pw-early" --reporter=list > "$G/early-$pane-$loc.log" 2>&1
  rc=$?
  tail -n +"$((before+1))" "$LOG" > "$G/early-$pane-$loc.access"
  total=$(grep -cE '"[A-Z]+ ' "$G/early-$pane-$loc.access"); bridge=$(grep -c '__semio/agent-bridge' "$G/early-$pane-$loc.access")
  n404=$(grep -cE '" 404 ' "$G/early-$pane-$loc.access"); other=$(grep -E '" [45][0-9][0-9] ' "$G/early-$pane-$loc.access" | grep -vc '" 404 ')
  echo "RESULT $pane $loc rc=$rc secs=$(( $(date +%s)-start )) requests=$total bridge=$bridge 404=$n404 other4xx5xx=$other"
done; done
