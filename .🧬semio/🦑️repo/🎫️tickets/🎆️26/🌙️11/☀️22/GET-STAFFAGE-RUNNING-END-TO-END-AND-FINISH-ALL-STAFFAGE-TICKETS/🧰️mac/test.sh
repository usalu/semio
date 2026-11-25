#!/usr/bin/env bash
# 🧪️ Vitest of the React target or the site under a capacity grant. Usage: [SEMIO_TEST_LEVEL=..] [TEST_MEMORY_MB=..] [TEST_TIMEOUT_S=..] test.sh <react|site> [vitest args...]
set -euo pipefail
source "$(dirname "$0")/env.sh"
W="$1"; shift
cd "$ROOT"
case "$W" in
  react) PKG="$S/🎯️targets/⚛️react/📦️packages/🟦️typescript"; CFG=(--config "$S/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts") ;;
  site) PKG="$S/🌐️site/📦️packages/🟦️typescript"; CFG=(--config "$S/🌐️site/🧪️tests/🎚️config/🟦️.ts" --configLoader native) ;;
esac
SEMIO_BUILD_MODE="${SEMIO_BUILD_MODE:-prototype}" repo capacity run --memory="${TEST_MEMORY_MB:-2500}" -- timeout "${TEST_TIMEOUT_S:-1500}" \
  "$BUN" --cwd "$PKG" "$ROOT/node_modules/vitest/vitest.mjs" run "${CFG[@]}" "$@"
