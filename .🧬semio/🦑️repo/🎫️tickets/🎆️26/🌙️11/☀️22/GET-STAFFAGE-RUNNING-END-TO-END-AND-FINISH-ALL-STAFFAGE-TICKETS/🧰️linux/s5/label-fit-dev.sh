#!/usr/bin/env bash
# 🏷️ Runs the site's `label-fit` case against the development server of the live tree (no build), both under one capacity grant; screenshots and findings go to `🗑️generated/linux-s5/<name>`. Usage: label-fit-dev.sh <name> [port]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
if [ "${1:-}" = "--inside" ]; then
  NAME="$2" PORT="$3"
  SITE="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🌐️site"
  OUT="$HERE/../../🗑️generated/linux-s5/$NAME"
  BUNS=("$HOME"/.cache/semio/runtimes/bun/*/bun)
  mkdir -p "$OUT"
  "$HERE/../s1/dev.sh" "$PORT" > "$OUT/s5-dev-server.log" 2>&1 &
  SERVER=$!
  trap 'kill "$SERVER" 2>/dev/null || true' EXIT
  for _ in $(seq 1 180); do curl -s -o /dev/null "http://127.0.0.1:$PORT/" && break; sleep 1; done
  TEACHING_ARCHITECTURE_STAFFAGE_URL="http://127.0.0.1:$PORT/" TEACHING_ARCHITECTURE_STAFFAGE_LABEL_FIT_OUT="$OUT" SEMIO_TEST_LEVEL=long SEMIO_BUILD_MODE=prototype \
    "${BUNS[0]}" --cwd "$SITE/📦️packages/🟦️typescript" "$ROOT/node_modules/vitest/vitest.mjs" run --config "$SITE/🧪️tests/🎚️config/🟦️.ts" --configLoader native label-fit
  exit $?
fi
cd "$ROOT"
repo capacity run --memory="${TEST_MEMORY_MB:-3500}" -- timeout "${TEST_TIMEOUT_S:-1500}" "$HERE/label-fit-dev.sh" --inside "$1" "${2:-6275}"
