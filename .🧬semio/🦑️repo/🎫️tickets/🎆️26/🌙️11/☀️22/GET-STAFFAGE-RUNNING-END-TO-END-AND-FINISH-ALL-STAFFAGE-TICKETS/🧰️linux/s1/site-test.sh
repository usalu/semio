#!/usr/bin/env bash
# 🧪️ Native fallback for `@teaching/staffage-site:test`: the site's own Vitest configuration with the pinned bun under a capacity grant. Usage: SEMIO_TEST_LEVEL=<quick|long> site-test.sh [vitest filters...]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
SITE="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🌐️site"
BUNS=("$HOME"/.cache/semio/runtimes/bun/*/bun)
cd "$ROOT"
SEMIO_BUILD_MODE="${SEMIO_BUILD_MODE:-prototype}" repo capacity run --memory="${TEST_MEMORY_MB:-2500}" -- timeout "${TEST_TIMEOUT_S:-900}" \
  "${BUNS[0]}" --cwd "$SITE/📦️packages/🟦️typescript" "$ROOT/node_modules/vitest/vitest.mjs" run --config "$SITE/🧪️tests/🎚️config/🟦️.ts" --configLoader native "$@"
