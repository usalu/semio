#!/usr/bin/env bash
# 🧪️ Native fallback for `@teaching/staffage-react:test`: the React target's own Vitest configuration with the pinned bun under a capacity grant. Usage: react-test.sh [vitest filters...]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
REACT="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react"
BUNS=("$HOME"/.cache/semio/runtimes/bun/*/bun)
cd "$ROOT"
SEMIO_BUILD_MODE="${SEMIO_BUILD_MODE:-prototype}" repo capacity run --memory="${TEST_MEMORY_MB:-2500}" -- timeout "${TEST_TIMEOUT_S:-1500}" \
  "${BUNS[0]}" --cwd "$REACT/📦️packages/🟦️typescript" "$ROOT/node_modules/vitest/vitest.mjs" run --config "$REACT/🧪️tests/🎚️config/🟦️.ts" "$@"
