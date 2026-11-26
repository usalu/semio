#!/usr/bin/env bash
# usage: browser-suites.sh <site-folder> <port> [baseline|behaviours|both] [extra playwright args...]
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"; ROOT="$(cd "$T/../../../../../../.." && pwd)"; G="$T/🗑️generated"; mkdir -p "$G"
SITE="$1"; PORT="$2"; WHICH="${3:-both}"; shift 3 || true
python3 "$T/static-origin.py" "$SITE" "$PORT" > "$G/bs-origin.log" 2>&1 & SERVER=$!
trap 'kill "$SERVER" 2>/dev/null || true' EXIT
for _ in $(seq 1 60); do curl -sf "http://127.0.0.1:$PORT/" >/dev/null && break; sleep 0.5; done
cd "$ROOT/♻️mit-bestand/🧺️demonstrator"
CLI="$(node -p 'require.resolve("@playwright/test/cli")')"
export DEMONSTRATOR_VERIFY_CDN=1 PLAYWRIGHT_BASE_URL="http://cdn.localhost:$PORT/" PLAYWRIGHT_BROWSERS_PATH="$ROOT/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
run() { PLAYWRIGHT_JSON_OUTPUT_FILE="$G/bs-$1.json" repo capacity run --memory=${BS_MEM:-2500} -- repo tools exec --light node -- "$CLI" test --config "$2" --retries=0 --output "$G/bs-$1-out" --reporter=list,json "${@:3}" 2>&1 | tee "$G/bs-$1.log" || true; }
[[ "$WHICH" != behaviours ]] && run baseline ./🔨️modules/🧪️e2e/🎚️config/🟦️.ts "$@"
[[ "$WHICH" != baseline ]] && run behaviours ./🧪️tests/🎭️acceptance/🧭️behaviours/🎚️config/🟦️.ts "$@"
echo BS-DONE
