#!/usr/bin/env bash
# 🪁️ Typecheck of the React target or the site with the workspace tsc under a capacity grant. Usage: typecheck.sh <react|site>
set -euo pipefail
source "$(dirname "$0")/env.sh"
case "$1" in
  react) P="$S/🎯️targets/⚛️react/📦️packages/🟦️typescript" ;;
  site) P="$S/🌐️site/📦️packages/🟦️typescript" ;;
  *) exit 2 ;;
esac
cd "$ROOT"
repo capacity run --memory="${TYPECHECK_MEMORY_MB:-3000}" -- timeout "${TYPECHECK_TIMEOUT_S:-1800}" \
  node --max-old-space-size=${TYPECHECK_HEAP_MB:-2000} "$ROOT/node_modules/typescript/lib/tsc.js" --noEmit --incremental false --pretty false -p "$P/tsconfig.json"
